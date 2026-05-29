use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use crate::models::asignacion::{Asignacion, CreateAsignacion};
use crate::service::asignacion_service as service;

/// Define las rutas disponibles para el recurso Asignaciones.
/// GET / -> obtener todas las asignaciones
/// POST / -> crear una nueva asignación
/// GET /{id} -> obtener una asignación por ID
/// PUT /{id} -> actualizar una asignación por ID
/// DELETE /{id} -> eliminar una asignación por ID
pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(obtener_todos).post(crear_asignacion))
        .route("/{id}", get(obtener_por_id).put(actualizar_asignacion).delete(eliminar_asignacion))
}

/// Obtiene todas las asignaciones registradas en la base de datos.
async fn obtener_todos(State(pool): State<PgPool>) -> Result<Json<Vec<Asignacion>>, StatusCode> {
    match service::obtener_asignaciones_service(&pool).await {
        Ok(lista_asignaciones) => Ok(Json(lista_asignaciones)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Crea una nueva asignación con los datos recibidos en el cuerpo de la petición.
async fn crear_asignacion(
    State(pool): State<PgPool>,
    Json(datos): Json<CreateAsignacion>,
) -> Result<(StatusCode, Json<Asignacion>), StatusCode> {
    match service::crear_asignacion_service(&pool, datos).await {
        Ok(nueva_asignacion) => Ok((StatusCode::CREATED, Json(nueva_asignacion))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Obtiene una asignación específica buscándola por su ID.
async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Asignacion>, StatusCode> {
    match service::obtener_asignacion_por_id_service(&pool, id).await {
        Ok(asignacion_encontrada) => Ok(Json(asignacion_encontrada)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Actualiza los datos de una asignación existente buscándola por su ID.
async fn actualizar_asignacion(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(datos): Json<CreateAsignacion>,
) -> Result<Json<Asignacion>, StatusCode> {
    match service::actualizar_asignacion_service(&pool, id, datos).await {
        Ok(asignacion_actualizada) => Ok(Json(asignacion_actualizada)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Elimina una asignación de la base de datos por su ID.
/// Retorna 204 No Content si se eliminó, 404 Not Found si no existe.
async fn eliminar_asignacion(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match service::eliminar_asignacion_service(&pool, id).await {
        Ok(filas) if filas > 0 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}