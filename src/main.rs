mod config;
mod controller;
mod models;
mod repository;
mod service;
mod utils;

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

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}

fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    Router::new()
        .nest("/api/clientes", controller::clientes_corp::router())
        .nest("/api/proyectos", controller::proyectos::router())
        .nest("/api/desarrolladores", controller::desarrolladores::router())
        .nest("/api/asignaciones", controller::asignacion::router())
        .nest("/api/tareas", controller::tarea::router())
        .nest("/api/reporte-general", controller::reporte::router())
        .with_state(pool)
}