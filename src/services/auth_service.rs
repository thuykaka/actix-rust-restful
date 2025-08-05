use crate::config;
use crate::models::db::User;
use crate::repositories::{
    refresh_token_repository::RefreshTokenRepository, user_repository::UserRepository,
};
use crate::{
    models::{
        errors::Error,
        request::{SignInRequest, SignUpRequest, UpdateUserRequest},
        response::{
            MeResponse, RefreshTokenResponse, SignInResponse, SignUpResponse, UpdateUserResponse,
        },
    },
    utils::{
        hash::{hash_password, verify_password},
        jwt::JwtClaims,
    },
};
use chrono::{Duration, Utc};
use entity::t_refresh_token;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use std::time::Instant;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService {
    pub user_repository: UserRepository,
    pub refresh_token_repository: RefreshTokenRepository,
}

impl AuthService {
    pub fn new(
        user_repository: UserRepository,
        refresh_token_repository: RefreshTokenRepository,
    ) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
        }
    }

    fn create_jwt_token(&self, user: User) -> Result<String, Error> {
        let jwt_token = JwtClaims::new(user.id, user.email, user.name);

        let token = jwt_token.generate_token()?;

        Ok(token)
    }

    async fn create_refresh_token(&self, user: User) -> Result<String, Error> {
        let token = Uuid::new_v4();
        let refresh_token_model = t_refresh_token::ActiveModel {
            user_id: Set(user.id),
            token: Set(token),
            expired_at: Set((Utc::now()
                + Duration::hours(*config::REFRESH_TOKEN_EXPIRATION_HOURS))
            .into()),
            data: Set(Some(serde_json::to_value(&user)?)),
            ..Default::default()
        };

        let refresh_token = self
            .refresh_token_repository
            .create_refresh_token(refresh_token_model)
            .await?;

        Ok(refresh_token.token.to_string())
    }

    pub async fn me(&self, user_id: Uuid) -> Result<MeResponse, Error> {
        let start_time = Instant::now();

        let user = self
            .user_repository
            .get_user_by_id(user_id)
            .await?
            .ok_or_else(|| Error::Unauthorized)?;

        log::info!(
            "auth_service -> me query took {}ms",
            start_time.elapsed().as_millis()
        );

        Ok(MeResponse(user.into()))
    }

    pub async fn authenticate(&self, body: SignInRequest) -> Result<SignInResponse, Error> {
        let start_time = Instant::now();

        let user = self
            .user_repository
            .get_user_by_email(&body.email)
            .await?
            .ok_or_else(|| {
                log::warn!("authenticate -> user not found with email: {}", body.email);
                Error::UnauthorizedWithMessage("Wrong email or password".to_string())
            })?;

        let is_password_valid = verify_password(&body.password, &user.password);

        if !is_password_valid {
            log::warn!("authenticate -> invalid password for user: {}", body.email);
            return Err(Error::UnauthorizedWithMessage(
                "Wrong email or password".to_string(),
            ));
        }

        let user_converted: User = user.into();
        let access_token = self.create_jwt_token(user_converted.clone())?;

        let refresh_token = self.create_refresh_token(user_converted.clone()).await?;

        log::info!("authenticate took {}ms", start_time.elapsed().as_millis());

        Ok(SignInResponse {
            access_token,
            refresh_token,
            user: user_converted,
        })
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        body: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, Error> {
        let mut user = self
            .user_repository
            .get_user_by_id(user_id)
            .await?
            .ok_or_else(|| Error::Unauthorized)?
            .into_active_model();

        if let Some(name) = body.name {
            user.name = Set(name);
        }

        if let Some(password) = body.password {
            user.password = Set(hash_password(&password));
        }

        user.updated_at = Set(Utc::now().into());
        let updated_user = self.user_repository.update_user(user).await?;

        Ok(UpdateUserResponse(updated_user.into()))
    }

    pub async fn sign_up(&self, body: SignUpRequest) -> Result<SignUpResponse, Error> {
        let start_time = Instant::now();
        let exists_user_with_email = self.user_repository.get_user_by_email(&body.email).await?;

        if exists_user_with_email.is_some() {
            log::warn!("email already exists: {} -> return", body.email);
            return Err(Error::BadRequest("Email already exists".to_string()));
        }
        log::info!(
            "check_email_exists took {}ms",
            start_time.elapsed().as_millis()
        );

        let start_time = Instant::now();

        let user_model = body.into_active_model();

        let user = self.user_repository.create_user(user_model).await?;

        log::info!("create_user took {}ms", start_time.elapsed().as_millis());

        let user_converted: User = user.into();
        let access_token = self.create_jwt_token(user_converted.clone())?;

        let refresh_token = self.create_refresh_token(user_converted.clone()).await?;

        Ok(SignUpResponse {
            access_token,
            refresh_token,
            user: user_converted,
        })
    }

    pub async fn refresh_token(&self, refresh_token: Uuid) -> Result<RefreshTokenResponse, Error> {
        let start_time = Instant::now();

        let refresh_token_model = self
            .refresh_token_repository
            .get_refresh_token(refresh_token)
            .await?
            .ok_or_else(|| Error::BadRequest("Invalid refresh token".to_string()))?;

        let refresh_token_data = refresh_token_model
            .data
            .ok_or_else(|| Error::BadRequest("Invalid refresh token".to_string()))?;

        let user = serde_json::from_value::<User>(refresh_token_data)?;

        let user_converted: User = user.into();
        let access_token = self.create_jwt_token(user_converted)?;

        log::info!("refresh_token took {}ms", start_time.elapsed().as_millis());

        Ok(RefreshTokenResponse {
            access_token,
            refresh_token: refresh_token.to_string(),
        })
    }
}
