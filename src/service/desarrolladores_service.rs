use sqlx::PgPool;

use crate::models::desarrolladores_model::{
    Desarrollador,
    NuevoDesarrollador,
};

use crate::repository::desarrolladores_repository;

pub async fn listar_desarrolladores(
    pool: &PgPool,
) -> Result<Vec<Desarrollador>, sqlx::Error> {

    desarrolladores_repository::obtener_todos(pool).await
}

pub async fn buscar_desarrollador(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Desarrollador>, sqlx::Error> {

    desarrolladores_repository::obtener_por_id(pool, id).await
}

pub async fn crear_desarrollador(
    pool: &PgPool,
    data: NuevoDesarrollador,
) -> Result<Desarrollador, sqlx::Error> {

    desarrolladores_repository::crear(pool, data).await
}

pub async fn eliminar_desarrollador(
    pool: &PgPool,
    id: i32,
) -> Result<u64, sqlx::Error> {

    desarrolladores_repository::eliminar(pool, id).await
}