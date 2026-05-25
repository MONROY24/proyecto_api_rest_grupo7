use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
    routing::get,
};
use sqlx::PgPool;

use crate::models::asignacion::{Asignacion, CreateAsignacion};
use crate::service::asignacion_service as service;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(obtener_todos).post(crear_asignacion))
        .route("/{id}", get(obtener_por_id).put(actualizar_asignacion).delete(eliminar_asignacion))
}

async fn obtener_todos(State(pool): State<PgPool>) -> Result<Json<Vec<Asignacion>>, StatusCode> {
    match service::obtener_asignaciones_service(&pool).await {
        Ok(asignaciones) => Ok(Json(asignaciones)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn crear_asignacion(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateAsignacion>,
) -> Result<(StatusCode, Json<Asignacion>), StatusCode> {
    match service::crear_asignacion_service(&pool, payload).await {
        Ok(asignacion) => Ok((StatusCode::CREATED, Json(asignacion))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Asignacion>, StatusCode> {
    match service::obtener_asignacion_por_id_service(&pool, id).await {
        Ok(asignacion) => Ok(Json(asignacion)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn actualizar_asignacion(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateAsignacion>,
) -> Result<Json<Asignacion>, StatusCode> {
    match service::actualizar_asignacion_service(&pool, id, payload).await {
        Ok(asignacion) => Ok(Json(asignacion)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn eliminar_asignacion(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match service::eliminar_asignacion_service(&pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}