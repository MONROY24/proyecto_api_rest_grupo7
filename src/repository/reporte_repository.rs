use sqlx::PgPool;
use crate::models::reporte::ReporteGeneral;

pub async fn obtener_reporte_general(pool: &PgPool) -> Result<Vec<ReporteGeneral>, sqlx::Error> {
    let reporte = sqlx::query_as!(
        ReporteGeneral,
        r#"
        SELECT 
            c.nombre_empresa,
            c.pais,
            p.nombre_proyecto,
            p.fecha_inicio,
            t.descripcion as descripcion_tarea,
            t.prioridad,
            t.estado,
            d.nombre as nombre_desarrollador,
            d.rol_principal,
            a.horas_estimadas
        FROM Asignaciones a
        INNER JOIN Tareas t ON a.id_tarea = t.id_tarea
        INNER JOIN Proyectos p ON t.id_proyecto = p.id_proyecto
        INNER JOIN Clientes_Corp c ON p.id_cliente = c.id_cliente
        INNER JOIN Desarrolladores d ON a.id_desarrollador = d.id_desarrollador
        ORDER BY p.nombre_proyecto, d.nombre
        "#
    )
    .fetch_all(pool)
    .await?;
    
    Ok(reporte)
}