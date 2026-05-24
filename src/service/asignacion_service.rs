use sqlx::PgPool;
use crate::models::asignacion::{Asignacion, CreateAsignacion};
use crate::repository::asignacion_repository as repo;

/// Obtiene todas las asignaciones llamando al repositorio.
pub async fn obtener_asignaciones_service(pool: &PgPool) -> Result<Vec<Asignacion>, sqlx::Error> {
    repo::obtener_asignaciones(pool).await
}

/// Obtiene una asignación por su ID llamando al repositorio.
pub async fn obtener_asignacion_por_id_service(pool: &PgPool, id: i32) -> Result<Asignacion, sqlx::Error> {
    repo::obtener_asignacion_por_id(pool, id).await
}

/// Crea una nueva asignación llamando al repositorio.
pub async fn crear_asignacion_service(pool: &PgPool, asignacion: CreateAsignacion) -> Result<Asignacion, sqlx::Error> {
    repo::crear_asignacion(pool, asignacion).await
}

/// Actualiza una asignación existente llamando al repositorio.
pub async fn actualizar_asignacion_service(pool: &PgPool, id: i32, asignacion: CreateAsignacion) -> Result<Asignacion, sqlx::Error> {
    repo::actualizar_asignacion(pool, id, asignacion).await
}

/// Elimina una asignación por su ID llamando al repositorio.
pub async fn eliminar_asignacion_service(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    repo::eliminar_asignacion(pool, id).await
}