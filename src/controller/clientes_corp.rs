use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
    routing::get,
};
use sqlx::PgPool;

use crate::models::clientes_corp::{ClienteCorp, CreateClienteCorp};
use crate::service::clientes_corp as service;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(obtener_todos).post(crear_cliente))
        .route("/{id}", get(obtener_por_id).put(actualizar_cliente).delete(eliminar_cliente))
}

async fn obtener_todos(State(pool): State<PgPool>) -> Result<Json<Vec<ClienteCorp>>, StatusCode> {
    match service::obtener_clientes_service(&pool).await {
        Ok(clientes) => Ok(Json(clientes)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn crear_cliente(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateClienteCorp>,
) -> Result<(StatusCode, Json<ClienteCorp>), StatusCode> {
    match service::crear_cliente_service(&pool, payload).await {
        Ok(cliente) => Ok((StatusCode::CREATED, Json(cliente))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<ClienteCorp>, StatusCode> {
    match service::obtener_cliente_por_id_service(&pool, id).await {
        Ok(cliente) => Ok(Json(cliente)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn actualizar_cliente(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateClienteCorp>,
) -> Result<Json<ClienteCorp>, StatusCode> {
    match service::actualizar_cliente_service(&pool, id, payload).await {
        Ok(cliente) => Ok(Json(cliente)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn eliminar_cliente(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match service::eliminar_cliente_service(&pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}