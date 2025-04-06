use todo::todo_service_server::{TodoService, TodoServiceServer};
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

    async fn update_todo(
        &self,
        request: tonic::Request<todo::UpdateTodoRequest>,
    ) -> Result<tonic::Response<todo::Todo>, tonic::Status> {
        let req = request.into_inner();
        if req.id.is_none() {
            return Err(tonic::Status::invalid_argument("id is required"));
        }

        let todo = db::update_todo(
            &self.pool,
            req.id.unwrap(),
            &req.title,
            req.completed,
        )
        .await;

        Ok(tonic::Response::new(todo.into()))
    }
    async fn delete_todo(
        &self,
        request: tonic::Request<todo::DeleteTodoRequest>,
    ) -> Result<tonic::Response<todo::Empty>, tonic::Status> {
        let req = request.into_inner();
        if req.id.is_none() {
            return Err(tonic::Status::invalid_argument("id is required"));
        }
        db::delete_todo(&self.pool, req.id.unwrap()).await;

        Ok(tonic::Response::new(todo::Empty {}))
    }
}

pub async fn run_server() {
    let pool = db::init_db().await;

    let addr = "[::1]:50051".parse().unwrap();
    let todo_service = MyTodoService { pool };

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    println!("Server listening on {}", addr);

    let grpc_web_service =
        tonic_web::enable(TodoServiceServer::new(todo_service));

    tonic::transport::Server::builder()
        .accept_http1(true)
        .layer(cors)
        .add_service(grpc_web_service)
        .serve(addr)
        .await
        .unwrap();
}
