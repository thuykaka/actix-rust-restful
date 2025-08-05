use crate::models::errors::Error;
use entity::t_todos;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, TransactionTrait,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct TodoRepository {
    pub db: DatabaseConnection,
}

impl TodoRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_all_todos(
        &self,
        limit: u64,
        offset: u64,
        search: Option<String>,
    ) -> Result<Vec<t_todos::Model>, Error> {
        let mut query = t_todos::Entity::find();

        if let Some(search) = search {
            query = query.filter(t_todos::Column::Title.like(format!("%{}%", search)));
        }

        let todos = query
            .order_by_desc(t_todos::Column::CreatedAt)
            .paginate(&self.db, limit)
            .fetch_page(offset)
            .await?;
        Ok(todos)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<t_todos::Model>, Error> {
        let todo = t_todos::Entity::find_by_id(id).one(&self.db).await?;
        Ok(todo)
    }

    pub async fn create(&self, todo: t_todos::ActiveModel) -> Result<t_todos::Model, Error> {
        let todo = todo.insert(&self.db).await?;
        Ok(todo)
    }

    pub async fn update(&self, todo: t_todos::ActiveModel) -> Result<t_todos::Model, Error> {
        let todo = todo.update(&self.db).await?;
        Ok(todo)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), Error> {
        self.db
            .transaction::<_, (), sea_orm::DbErr>(|txn| {
                Box::pin(async move {
                    t_todos::Entity::delete_by_id(id).exec(txn).await?;
                    Ok(())
                })
            })
            .await?;
        Ok(())
    }
}
