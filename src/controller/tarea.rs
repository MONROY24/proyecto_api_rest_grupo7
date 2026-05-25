use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
    routing::get,
};
use sqlx::PgPool;

use crate::models::tarea::{Tarea, CreateTarea};
use crate::service::tarea as service;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(obtener_todas).post(crear_tarea))
        .route("/{id}", get(obtener_por_id).put(actualizar_tarea).delete(eliminar_tarea))
}

async fn obtener_todas(State(pool): State<PgPool>) -> Result<Json<Vec<Tarea>>, StatusCode> {
    match service::obtener_tareas(&pool).await {
        Ok(tareas) => Ok(Json(tareas)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn crear_tarea(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTarea>,
) -> Result<(StatusCode, Json<Tarea>), StatusCode> {
    match service::crear_tarea(&pool, payload).await {
        Ok(tarea) => Ok((StatusCode::CREATED, Json(tarea))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Tarea>, StatusCode> {
    match service::obtener_tarea_por_id(&pool, id).await {
        Ok(tarea) => Ok(Json(tarea)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn actualizar_tarea(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateTarea>,
) -> Result<Json<Tarea>, StatusCode> {
    match service::actualizar_tarea(&pool, id, payload).await {
        Ok(tarea) => Ok(Json(tarea)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn eliminar_tarea(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match service::eliminar_tarea(&pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}