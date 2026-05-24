use sqlx::PgPool;
use crate::models::asignacion::{Asignacion, CreateAsignacion};

/// Obtiene todas las asignaciones registradas en la base de datos.
/// Retorna un vector con todas las filas de la tabla Asignaciones.
pub async fn obtener_asignaciones(pool: &PgPool) -> Result<Vec<Asignacion>, sqlx::Error> {
    let asignaciones = sqlx::query_as!(
        Asignacion,
        r#"SELECT id_asignacion, id_tarea, id_desarrollador, horas_estimadas FROM Asignaciones"#
    )
    .fetch_all(pool)
    .await?;
    Ok(asignaciones)
}

/// Busca una asignación específica por su ID.
/// Retorna la asignación si existe, o un error si no se encuentra.
pub async fn obtener_asignacion_por_id(pool: &PgPool, id: i32) -> Result<Asignacion, sqlx::Error> {
    let asignacion = sqlx::query_as!(
        Asignacion,
        r#"SELECT id_asignacion, id_tarea, id_desarrollador, horas_estimadas 
           FROM Asignaciones WHERE id_asignacion = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(asignacion)
}

/// Inserta una nueva asignación en la base de datos.
/// Retorna la fila recién creada con su id_asignacion generado automáticamente.
pub async fn crear_asignacion(pool: &PgPool, asignacion: CreateAsignacion) -> Result<Asignacion, sqlx::Error> {
    let nueva_asignacion = sqlx::query_as!(
        Asignacion,
        r#"
        INSERT INTO Asignaciones (id_tarea, id_desarrollador, horas_estimadas)
        VALUES ($1, $2, $3)
        RETURNING id_asignacion, id_tarea, id_desarrollador, horas_estimadas
        "#,
        asignacion.id_tarea,
        asignacion.id_desarrollador,
        asignacion.horas_estimadas
    )
    .fetch_one(pool)
    .await?;
    Ok(nueva_asignacion)
}

/// Actualiza una asignación existente buscándola por su ID.
/// Reemplaza los campos con los nuevos valores recibidos y retorna la fila actualizada.
pub async fn actualizar_asignacion(pool: &PgPool, id: i32, asignacion: CreateAsignacion) -> Result<Asignacion, sqlx::Error> {
    let asignacion_actualizada = sqlx::query_as!(
        Asignacion,
        r#"
        UPDATE Asignaciones
        SET id_tarea = $1, id_desarrollador = $2, horas_estimadas = $3
        WHERE id_asignacion = $4
        RETURNING id_asignacion, id_tarea, id_desarrollador, horas_estimadas
        "#,
        asignacion.id_tarea,
        asignacion.id_desarrollador,
        asignacion.horas_estimadas,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(asignacion_actualizada)
}

/// Elimina una asignación de la base de datos por su ID.
/// Retorna la cantidad de filas afectadas (1 si se eliminó, 0 si no existía).
pub async fn eliminar_asignacion(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let resultado = sqlx::query!(
        r#"DELETE FROM Asignaciones WHERE id_asignacion = $1"#,
        id
    )
    .execute(pool)
    .await?;
    Ok(resultado.rows_affected())
}