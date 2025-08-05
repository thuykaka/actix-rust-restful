pub use sea_orm_migration::prelude::*;

mod m20250731_042456_create_user_table;
mod m20250805_021726_create_todo_table;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250731_042456_create_user_table::Migration),
            Box::new(m20250805_021726_create_todo_table::Migration),
        ]
    }
}
