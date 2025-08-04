use crate::repositories::user_repository::UserRepository;
use crate::{
    models::{
        errors::Error,
        request::{SignInRequest, SignUpRequest, UpdateUserRequest},
        response::{MeResponse, SignInResponse, SignUpResponse, UpdateUserResponse},
    },
    utils::{
        hash::{hash_password, verify_password},
        jwt::JwtClaims,
    },
};
use chrono::Utc;
use entity::t_users;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use std::time::Instant;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService {
    pub user_repository: UserRepository,
}

impl AuthService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    fn create_jwt_token(&self, user: t_users::Model) -> Result<String, Error> {
        let jwt_token = JwtClaims::new(user.id, user.email, user.name);

        let token = jwt_token.generate_token()?;

        Ok(token)
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

        log::info!("authenticate took {}ms", start_time.elapsed().as_millis());

        let token = self.create_jwt_token(user.clone())?;

        Ok(SignInResponse {
            token,
            user: user.into(),
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

        let token = self.create_jwt_token(user.clone())?;

        Ok(SignUpResponse {
            token,
            user: user.into(),
        })
    }
}
