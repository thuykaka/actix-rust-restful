use crate::{models::todo::Todo, repositories::database_mockup::Database};

#[derive(Clone)]
pub struct TodoService {
    pub database_mock: Database,
}

impl TodoService {
    pub fn new(database_mock: Database) -> Self {
        Self { database_mock }
    }

    pub fn create_todo(&self, new_todo: Todo) -> Todo {
        println!("create todo service.");
        let todo = self.database_mock.create_todo(new_todo).unwrap();
        todo
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        let todos = self.database_mock.get_todos();
        todos
    }

    pub fn get_todo_by_id(&self, id: String) -> Option<Todo> {
        self.database_mock.get_todo_by_id(&id)
    }

    pub fn update_todo_by_id(&self, id: String, todo: Todo) -> Option<Todo> {
        self.database_mock.update_todo_by_id(&id, todo)
    }

    pub fn delete_todo_by_id(&self, id: String) -> Option<Todo> {
        self.database_mock.delete_todo_by_id(&id)
    }
}
