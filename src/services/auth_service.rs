use std::time::Instant;

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
use entity::t_users;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{DatabaseConnection, IntoActiveModel};

#[derive(Clone)]
pub struct AuthService {
    pub db: DatabaseConnection,
}

impl AuthService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn me(&self, id: String) -> Result<MeResponse, Error> {
        let start_time = Instant::now();

        let user_id = id.parse::<i32>().map_err(|_| Error::Unauthorized)?;

        let user = t_users::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| Error::Unauthorized)?;

        log::info!("me query took {}ms", start_time.elapsed().as_millis());

        Ok(MeResponse(user.into()))
    }

    pub async fn authenticate(&self, body: SignInRequest) -> Result<SignInResponse, Error> {
        let start_time = Instant::now();

        let user = t_users::Entity::find()
            .filter(t_users::Column::Email.eq(body.email.to_lowercase()))
            .one(&self.db)
            .await?;

        if user.is_none() {
            log::warn!("User not found with email: {}", body.email);
            return Err(Error::UnauthorizedWithMessage(
                "Wrong email or password".to_string(),
            ));
        }

        let user = user.unwrap();

        let is_password_valid = verify_password(&body.password, &user.password);

        if !is_password_valid {
            log::warn!("Invalid password for user: {}", body.email);
            return Err(Error::UnauthorizedWithMessage(
                "Wrong email or password".to_string(),
            ));
        }

        log::info!("authenticate took {}ms", start_time.elapsed().as_millis());

        let jwt_token = JwtClaims::new(
            user.id.to_string(),
            user.email.to_string(),
            user.name.to_string(),
        );

        let token = jwt_token.generate_token()?;

        Ok(SignInResponse {
            token,
            user: user.into(),
        })
    }

    pub async fn update(
        &self,
        id: String,
        body: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, Error> {
        let user_id = id.parse::<i32>().map_err(|_| Error::Unauthorized)?;

        let mut user = t_users::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| Error::Unauthorized)?
            .into_active_model();

        if let Some(name) = body.name {
            user.name = Set(name);
        }
        user.password = Set(hash_password(&body.password));
        let updated_user = user.update(&self.db).await?;

        Ok(UpdateUserResponse(updated_user.into()))
    }

    pub async fn sign_up(&self, body: SignUpRequest) -> Result<SignUpResponse, Error> {
        let start_time = Instant::now();
        let email_exists = self.check_email_exists(&body.email).await?;

        if email_exists {
            log::warn!("email already exists: {} -> return", body.email);
            return Err(Error::BadRequest("Email already exists".to_string()));
        }
        log::info!(
            "check_email_exists took {}ms",
            start_time.elapsed().as_millis()
        );

        let start_time = Instant::now();
        let user = self.create_user(body).await?;

        log::info!("create_user took {}ms", start_time.elapsed().as_millis());

        let jwt_token = JwtClaims::new(
            user.id.to_string(),
            user.email.to_string(),
            user.name.to_string(),
        );

        let token = jwt_token.generate_token()?;

        Ok(SignUpResponse {
            token,
            user: user.into(),
        })
    }

    async fn check_email_exists(&self, email: &str) -> Result<bool, Error> {
        let user = t_users::Entity::find()
            .filter(t_users::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        Ok(user.is_some())
    }

    async fn create_user(&self, body: SignUpRequest) -> Result<t_users::Model, Error> {
        let hashed_password = hash_password(&body.password);

        let user_model = t_users::ActiveModel {
            name: Set(body.name),
            email: Set(body.email.to_lowercase()),
            password: Set(hashed_password),
            ..Default::default()
        };

        let user = user_model.insert(&self.db).await?;

        Ok(user)
    }
}
