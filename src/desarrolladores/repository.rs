use sqlx::PgPool;

use super::model::{Desarrollador, NuevoDesarrollador};

pub async fn obtener_todos(
    pool: &PgPool,
) -> Result<Vec<Desarrollador>, sqlx::Error> {

    sqlx::query_as::<_, Desarrollador>(
        "SELECT * FROM desarrolladores ORDER BY id"
    )
    .fetch_all(pool)
    .await
}

pub async fn obtener_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Desarrollador>, sqlx::Error> {

    sqlx::query_as::<_, Desarrollador>(
        "SELECT * FROM desarrolladores WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn crear(
    pool: &PgPool,
    data: NuevoDesarrollador,
) -> Result<Desarrollador, sqlx::Error> {

    sqlx::query_as::<_, Desarrollador>(
        "INSERT INTO desarrolladores (nombre, lenguaje, experiencia)
         VALUES ($1, $2, $3)
         RETURNING *"
    )
    .bind(data.nombre)
    .bind(data.lenguaje)
    .bind(data.experiencia)
    .fetch_one(pool)
    .await
}

pub async fn eliminar(
    pool: &PgPool,
    id: i32,
) -> Result<u64, sqlx::Error> {

    let result = sqlx::query(
        "DELETE FROM desarrolladores WHERE id = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}