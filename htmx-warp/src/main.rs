use std::collections::HashMap;

use serde::Deserialize;
use todo::todo_service_client::TodoServiceClient;
use tonic::transport::Channel;
use warp::{reject::Rejection, reply::Reply};

mod todo {
    tonic::include_proto!("todo");
}

#[derive(Debug, Clone)]
struct AppState {
    todo_client: TodoServiceClient<Channel>,
}

#[derive(Deserialize)]
struct CreateTodoForm {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodoForm {
    #[serde(flatten)]
    extra: HashMap<String, String>,
    completed: String,
}

fn main() {
    println!("Hello, world!");
}

async fn get_todos_handler(state: AppState) -> Result<impl Reply, Rejection> {
    let mut client = state.todo_client;
    let request = tonic::Request::new(todo::Empty {});
    let response = client.get_todos(request).await.map_err(|e| {
        eprintln!("Error calling get_todos: {:?}", e);
        warp::reject::not_found()
    })?;

    let todos = response.into_inner().todos;

    Ok(warp::reply::html(render_todo_list(&todos)))
}

fn render_todo_list(todos: &[todo::Todo]) -> String {
    let mut html = String::new();
    for todo in todos {
        html.push_str(&render_todo_item(todo))
    }
    html
}

fn render_todo_item(todo: &todo::Todo) -> String {
    format!(
        r#"
        <div class="todo-item" id="todo-{}" hx-target="this" hx-swap="outerHTML">
        <input type="checkbox" 
            hx-put="/todos/{}" 
            hx-include="[name='title-{}']"
            name="completed"
            {}/>
            <span class="{}">{}</span>
            <input type="hidden" name="title-{}" value="{}" />
            <button hx-delete="/todos/{}">Delete</button>
        </div>
    "#,
        todo.id.unwrap_or(0),
        todo.id.unwrap_or(0),
        todo.id.unwrap_or(0),
        if todo.completed { "checked" } else { "" },
        if todo.completed { "completed" } else { "" },
        todo.title,
        todo.id.unwrap_or(0),
        todo.title,
        todo.id.unwrap_or(0),
    )
}
