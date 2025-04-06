use server::run_server;

mod config;
mod db;
mod server;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    run_server().await;
}
