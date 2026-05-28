use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
    routing::get,
};

use sqlx::PgPool;

use crate::models::desarrolladores_model::{Desarrollador, NuevoDesarrollador};
use crate::service::desarrolladores_service as service;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(obtener_todos).post(crear_desarrollador))
        .route(
            "/{id}",
            get(obtener_por_id)
                .put(actualizar_desarrollador)
                .delete(eliminar_desarrollador),
        )
}

async fn obtener_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Desarrollador>>, StatusCode> {
    match service::listar_desarrolladores(&pool).await {
        Ok(desarrolladores) => Ok(Json(desarrolladores)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn crear_desarrollador(
    State(pool): State<PgPool>,
    Json(payload): Json<NuevoDesarrollador>,
) -> Result<(StatusCode, Json<Desarrollador>), StatusCode> {
    match service::crear_desarrollador(&pool, payload).await {
        Ok(dev) => Ok((StatusCode::CREATED, Json(dev))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Desarrollador>, StatusCode> {
    match service::buscar_desarrollador(&pool, id).await {
        Ok(Some(dev)) => Ok(Json(dev)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn actualizar_desarrollador(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<NuevoDesarrollador>,
) -> Result<Json<Desarrollador>, StatusCode> {
    match service::actualizar_desarrollador(&pool, id, payload).await {
        Ok(Some(dev)) => Ok(Json(dev)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn eliminar_desarrollador(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match service::eliminar_desarrollador(&pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}