use sqlx::PgPool;
use crate::models::proyectos::{Proyecto, CreateProyecto};

pub async fn obtener_proyectos(pool: &PgPool) -> Result<Vec<Proyecto>, sqlx::Error> {
    let proyectos: Vec<Proyecto> = sqlx::query_as!(
        Proyecto,
        r#"
        SELECT 
            id_proyecto,
            id_cliente,
            nombre_proyecto,
            descripcion,
            fecha_inicio,
            fecha_fin,
            estado
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
            id_cliente,
            nombre_proyecto,
            descripcion,
            fecha_inicio,
            fecha_fin,
            estado
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
            id_cliente,
            nombre_proyecto,
            descripcion,
            fecha_inicio,
            fecha_fin,
            estado
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id_proyecto,
            id_cliente,
            nombre_proyecto,
            descripcion,
            fecha_inicio,
            fecha_fin,
            estado
        "#,
        proyecto.id_cliente,
        proyecto.nombre_proyecto,
        proyecto.descripcion,
        proyecto.fecha_inicio,
        proyecto.fecha_fin,
        proyecto.estado
    )
    .fetch_one(pool)
    .await?;

    Ok(nuevo_proyecto)
}