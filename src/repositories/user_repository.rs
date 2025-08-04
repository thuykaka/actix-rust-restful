use crate::models::errors::Error;
use entity::t_users;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pub db: DatabaseConnection,
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<t_users::Model>, Error> {
        let user = t_users::Entity::find()
            .filter(t_users::Column::Id.eq(id))
            .one(&self.db)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<t_users::Model>, Error> {
        let user = t_users::Entity::find()
            .filter(t_users::Column::Email.eq(email))
            .one(&self.db)
            .await?;
        Ok(user)
    }

    pub async fn create_user(&self, user: t_users::ActiveModel) -> Result<t_users::Model, Error> {
        let user = user.insert(&self.db).await?;
        Ok(user)
    }

    pub async fn update_user(&self, user: t_users::ActiveModel) -> Result<t_users::Model, Error> {
        let user = user.update(&self.db).await?;
        Ok(user)
    }

    #[allow(unused)]
    pub async fn delete_user(&self, id: Uuid) -> Result<(), Error> {
        t_users::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }
}
