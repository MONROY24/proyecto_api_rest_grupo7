use sqlx::PgPool;
use crate::models::tarea::{Tarea, CreateTarea};
use crate::repository::tarea_repository as repo;

pub async fn obtener_tareas(pool: &PgPool) -> Result<Vec<Tarea>, sqlx::Error> {
    repo::obtener_tareas(pool).await
}

pub async fn obtener_tarea_por_id(pool: &PgPool, id: i32) -> Result<Tarea, sqlx::Error> {
    repo::obtener_tarea_por_id(pool, id).await
}

pub async fn crear_tarea(pool: &PgPool, tarea: CreateTarea) -> Result<Tarea, sqlx::Error> {
    repo::crear_tarea(pool, tarea).await
}

pub async fn actualizar_tarea(pool: &PgPool, id: i32, tarea: CreateTarea) -> Result<Tarea, sqlx::Error> {
    repo::actualizar_tarea(pool, id, tarea).await
}

pub async fn eliminar_tarea(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    repo::eliminar_tarea(pool, id).await
}