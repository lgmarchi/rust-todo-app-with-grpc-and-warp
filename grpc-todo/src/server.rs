use todo::todo_service_server::TodoService;
use tonic::Request;

use crate::db;

/// This module contains the generated code for the `todo` gRPC service.
///
/// The `tonic::include_proto!` macro is used to include the compiled Protobuf definitions
/// for the `todo` service. This allows the server to use the types and services defined
/// in the Protobuf file.
///
/// Make sure that the Protobuf file is compiled and the generated code is available
/// in the appropriate location for this macro to work correctly.
mod todo {
    tonic::include_proto!("todo");
}

#[derive(Debug)]
pub struct MyTodoService {
    pool: sqlx::SqlitePool,
}

impl From<db::Todo> for todo::Todo {
    fn from(todo: db::Todo) -> Self {
        todo::Todo {
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
        }
    }
}

#[tonic::async_trait]
impl TodoService for MyTodoService {
    async fn create_todo(
        &self,
        request: tonic::Request<todo::CreateTodoRequest>,
    ) -> Result<tonic::Response<todo::Todo>, tonic::Status> {
        let req = request.into_inner();
        let todo = db::create_todo(&self.pool, &req.title).await;

        Ok(tonic::Response::new(todo.into()))
    }

    async fn get_todos(
        &self,
        _: Request<todo::Empty>,
    ) -> Result<tonic::Response<todo::TodoList>, tonic::Status> {
        let todos = db::get_todos(&self.pool).await;

        Ok(tonic::Response::new(todo::TodoList {
            todos: todos.into_iter().map(|todo| todo.into()).collect(),
        }))
    }
}
