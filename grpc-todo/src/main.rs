use server::run_server;

mod config;
mod db;
mod server;

#[tokio::main]
async fn main() {
    run_server().await;
    println!("Hello, world!");
}
