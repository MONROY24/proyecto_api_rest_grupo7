use sqlx::PgPool;

use crate::models::proyectos::{Proyecto, CreateProyecto};

pub async fn obtener_proyectos(
    pool: &PgPool,
) -> Result<Vec<Proyecto>, sqlx::Error> {

    let proyectos: Vec<Proyecto> = sqlx::query_as!(
        Proyecto,
        r#"
        SELECT
            id_proyecto,
            nombre_proyecto,
            fecha_inicio,
            id_cliente
        FROM Proyectos
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(proyectos)
}

pub async fn obtener_proyecto_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Proyecto, sqlx::Error> {

    let proyecto: Proyecto = sqlx::query_as!(
        Proyecto,
        r#"
        SELECT
            id_proyecto,
            nombre_proyecto,
            fecha_inicio,
            id_cliente
        FROM Proyectos
        WHERE id_proyecto = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(proyecto)
}

pub async fn crear_proyecto(
    pool: &PgPool,
    proyecto: CreateProyecto,
) -> Result<Proyecto, sqlx::Error> {

    let nuevo_proyecto: Proyecto = sqlx::query_as!(
        Proyecto,
        r#"
        INSERT INTO Proyectos
        (
            nombre_proyecto,
            fecha_inicio,
            id_cliente
        )
        VALUES ($1, $2, $3)
        RETURNING
            id_proyecto,
            nombre_proyecto,
            fecha_inicio,
            id_cliente
        "#,
        proyecto.nombre_proyecto,
        proyecto.fecha_inicio,
        proyecto.id_cliente
    )
    .fetch_one(pool)
    .await?;

    Ok(nuevo_proyecto)
}

pub async fn actualizar_proyecto(
    pool: &PgPool,
    id: i32,
    proyecto: CreateProyecto,
) -> Result<Proyecto, sqlx::Error> {

    let proyecto_actualizado: Proyecto = sqlx::query_as!(
        Proyecto,
        r#"
        UPDATE Proyectos
        SET
            nombre_proyecto = $1,
            fecha_inicio = $2,
            id_cliente = $3
        WHERE id_proyecto = $4
        RETURNING
            id_proyecto,
            nombre_proyecto,
            fecha_inicio,
            id_cliente
        "#,
        proyecto.nombre_proyecto,
        proyecto.fecha_inicio,
        proyecto.id_cliente,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(proyecto_actualizado)
}

pub async fn eliminar_proyecto(
    pool: &PgPool,
    id: i32,
) -> Result<u64, sqlx::Error> {

    let resultado = sqlx::query!(
        r#"
        DELETE FROM Proyectos
        WHERE id_proyecto = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(resultado.rows_affected())
}