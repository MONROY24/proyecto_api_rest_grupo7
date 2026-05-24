use sqlx::PgPool;
use crate::models::tarea::{Tarea, CreateTarea};

pub async fn obtener_tareas(pool: &PgPool) -> Result<Vec<Tarea>, sqlx::Error> {
    let tareas = sqlx::query_as!(
        Tarea,
        r#"SELECT id_tarea, id_proyecto, descripcion, prioridad, estado FROM Tareas"#
    )
    .fetch_all(pool)
    .await?;

    Ok(tareas)
}

pub async fn obtener_tarea_por_id(pool: &PgPool, id: i32) -> Result<Tarea, sqlx::Error> {
    let tarea = sqlx::query_as!(
        Tarea,
        r#"SELECT id_tarea, id_proyecto, descripcion, prioridad, estado FROM Tareas WHERE id_tarea = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(tarea)
}

pub async fn crear_tarea(pool: &PgPool, tarea: CreateTarea) -> Result<Tarea, sqlx::Error> {
    let nueva_tarea = sqlx::query_as!(
        Tarea,
        r#"
        INSERT INTO Tareas (id_proyecto, descripcion, prioridad, estado)
        VALUES ($1, $2, $3, $4)
        RETURNING id_tarea, id_proyecto, descripcion, prioridad, estado
        "#,
        tarea.id_proyecto,
        tarea.descripcion,
        tarea.prioridad,
        tarea.estado
    )
    .fetch_one(pool)
    .await?;

    Ok(nueva_tarea)
}

pub async fn actualizar_tarea(pool: &PgPool, id: i32, tarea: CreateTarea) -> Result<Tarea, sqlx::Error> {
    let tarea_actualizada = sqlx::query_as!(
        Tarea,
        r#"
        UPDATE Tareas
        SET id_proyecto = $1, descripcion = $2, prioridad = $3, estado = $4
        WHERE id_tarea = $5
        RETURNING id_tarea, id_proyecto, descripcion, prioridad, estado
        "#,
        tarea.id_proyecto,
        tarea.descripcion,
        tarea.prioridad,
        tarea.estado,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(tarea_actualizada)
}

pub async fn eliminar_tarea(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let resultado = sqlx::query!(
        r#"DELETE FROM Tareas WHERE id_tarea = $1"#,
        id
    )
    .execute(pool)
    .await?;

    Ok(resultado.rows_affected())
}
