use std::time::Instant;

use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, Data},
};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    config::constants::JWT_SECRET,
    middlewares::auth_middleware::AuthenticatedUser,
    models::{
        app_state::AppState,
        auth::{LoginRequest, LoginResponse, RegisterRequest, UserInfo},
        jwt::Claims,
    },
};

use entity::t_users;

#[post("/register")]
async fn register(app_state: Data<AppState>, body: web::Json<RegisterRequest>) -> impl Responder {
    // Check if email already exists
    match check_email_exists(&app_state.db, &body.email).await {
        Ok(true) => {
            return HttpResponse::Conflict().json(serde_json::json!({
                "message": "Email already exists"
            }));
        }
        Ok(false) => {
            // Email doesn't exist, continue with registration
        }
        Err(e) => {
            log::error!("Database error checking email: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Internal server error"
            }));
        }
    }

    match create_user(&app_state.db, &body.name, &body.email, &body.password).await {
        Ok(user) => {
            log::info!("New user registered: {}", user.email);
            HttpResponse::Created().json(serde_json::json!({
                "message": "User registered successfully",
                "user": {
                    "id": user.id,
                    "name": user.name,
                    "email": user.email
                }
            }))
        }
        Err(e) => {
            log::error!("Error creating user: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Failed to create user"
            }))
        }
    }
}

/// Check if email already exists in database
async fn check_email_exists(
    db: &sea_orm::DatabaseConnection,
    email: &str,
) -> Result<bool, sea_orm::DbErr> {
    let user = t_users::Entity::find()
        .filter(t_users::Column::Email.eq(email))
        .one(db)
        .await?;

    Ok(user.is_some())
}

/// Create a new user with hashed password
async fn create_user(
    db: &sea_orm::DatabaseConnection,
    name: &str,
    email: &str,
    password: &str,
) -> Result<t_users::Model, sea_orm::DbErr> {
    let hashed_password = hash_password(password);

    let user_model = t_users::ActiveModel {
        name: Set(name.to_string()),
        email: Set(email.to_string()),
        password: Set(hashed_password),
        ..Default::default()
    };

    user_model.insert(db).await.map_err(|e| e.into())
}

/// Hash password for storage using Argon2
fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    password_hash
}

#[post("/login")]
async fn login(app_state: Data<AppState>, body: web::Json<LoginRequest>) -> impl Responder {
    match authenticate_user(&app_state.db, &body.email, &body.password).await {
        Ok(Some(user)) => {
            log::info!("User logged in successfully: {}", user.email);

            // Create user info for JWT and response
            let user_info = UserInfo {
                id: user.id,
                name: user.name.clone(),
                email: user.email.clone(),
            };

            // Generate JWT token
            let claims = Claims::new(user_info.clone());
            match claims.generate_token(&JWT_SECRET) {
                Ok(token) => {
                    let response = LoginResponse {
                        message: "Login successful".to_string(),
                        token,
                        user: user_info,
                    };
                    HttpResponse::Ok().json(response)
                }
                Err(e) => {
                    log::error!("Failed to generate JWT token: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "message": "Failed to generate authentication token"
                    }))
                }
            }
        }
        Ok(None) => {
            log::warn!("Login attempt failed for email: {}", body.email);
            HttpResponse::Unauthorized().json(serde_json::json!({
                "message": "Invalid email or password"
            }))
        }
        Err(e) => {
            log::error!("Database error during login: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Internal server error"
            }))
        }
    }
}

/// Authenticate user by email and password
async fn authenticate_user(
    db: &sea_orm::DatabaseConnection,
    email: &str,
    password: &str,
) -> Result<Option<t_users::Model>, sea_orm::DbErr> {
    // First, find user by email only
    let start_time = Instant::now();
    let user = t_users::Entity::find()
        .filter(t_users::Column::Email.eq(email))
        .one(db)
        .await?;

    log::info!("Time taken to find user: {:?}", start_time.elapsed());

    match user {
        Some(user) => {
            if verify_password(password, &user.password) {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

/// Verify password against stored hash using Argon2
fn verify_password(password: &str, stored_hash: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&stored_hash).unwrap();
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[get("/me")]
async fn me(user: AuthenticatedUser) -> impl Responder {
    log::info!("User information retrieved for: {}", user.email);

    HttpResponse::Ok().json(serde_json::json!({
        "message": "User information retrieved successfully",
        "user": &*user
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    use crate::middlewares::auth_middleware::AuthMiddleware;

    cfg.service(
        web::scope("/auth")
            .service(register)
            .service(login)
            .service(web::scope("").wrap(AuthMiddleware).service(me)),
    );
}
