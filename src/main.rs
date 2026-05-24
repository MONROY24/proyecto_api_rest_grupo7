mod service;
mod utils;
mod models;
mod controller;
mod config;
mod repository;

use config::config::crear_pool;
use axum::Router;

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}\n");

    let pool = crear_pool()
        .await
        .expect("No se pudo conectar a la base de datos");

}
fn unificar_routers(_pool: sqlx::PgPool) -> axum::Router {
    Router::new()
}