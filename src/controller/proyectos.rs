use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::PgPool;

use crate::models::proyectos::{CreateProyecto, Proyecto};
use crate::service::proyectos_service as service;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(obtener_todos).post(crear_proyecto))
        .route("/{id}", get(obtener_por_id).put(actualizar_proyecto).delete(eliminar_proyecto))
}

async fn obtener_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Proyecto>>, StatusCode> {
    match service::obtener_proyectos(&pool).await {
        Ok(lista_proyectos) => Ok(Json(lista_proyectos)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn crear_proyecto(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProyecto>,
) -> Result<(StatusCode, Json<Proyecto>), StatusCode> {
    match service::crear_proyecto(&pool, payload).await {
        Ok(proyecto) => Ok((StatusCode::CREATED, Json(proyecto))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Proyecto>, StatusCode> {
    match service::obtener_proyecto_por_id(&pool, id).await {
        Ok(proyecto) => Ok(Json(proyecto)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn actualizar_proyecto(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateProyecto>,
) -> Result<Json<Proyecto>, StatusCode> {
    match service::actualizar_proyecto(&pool, id, payload).await {
        Ok(proyecto) => Ok(Json(proyecto)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn eliminar_proyecto(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match service::eliminar_proyecto(&pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}