use sqlx::PgPool;

use crate::models::desarrolladores_model::{
    Desarrollador,
    NuevoDesarrollador,
};

pub async fn obtener_todos(
    pool: &PgPool,
) -> Result<Vec<Desarrollador>, sqlx::Error> {

    sqlx::query_as::<_, Desarrollador>(
        "SELECT * FROM Desarrolladores ORDER BY id_desarrollador"
    )
    .fetch_all(pool)
    .await
}

pub async fn obtener_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Desarrollador>, sqlx::Error> {

    sqlx::query_as::<_, Desarrollador>(
        "SELECT * FROM Desarrolladores WHERE id_desarrollador = $1"
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
        "INSERT INTO Desarrolladores (nombre, rol_principal, nivel)
         VALUES ($1, $2, $3)
         RETURNING *"
    )
    .bind(data.nombre)
    .bind(data.rol_principal)
    .bind(data.nivel)
    .fetch_one(pool)
    .await
}

pub async fn actualizar(
    pool: &PgPool,
    id: i32,
    data: NuevoDesarrollador,
) -> Result<Option<Desarrollador>, sqlx::Error> {

    sqlx::query_as::<_, Desarrollador>(
        "UPDATE Desarrolladores
         SET nombre = $1, rol_principal = $2, nivel = $3
         WHERE id_desarrollador = $4
         RETURNING *"
    )
    .bind(data.nombre)
    .bind(data.rol_principal)
    .bind(data.nivel)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn eliminar(
    pool: &PgPool,
    id: i32,
) -> Result<u64, sqlx::Error> {

    let result = sqlx::query(
        "DELETE FROM Desarrolladores WHERE id_desarrollador = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}