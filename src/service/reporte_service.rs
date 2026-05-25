use sqlx::PgPool;
use crate::models::reporte::ReporteGeneral;
use crate::repository::reporte_repository as repo;

pub async fn obtener_reporte_general_service(pool: &PgPool) -> Result<Vec<ReporteGeneral>, sqlx::Error> {
    repo::obtener_reporte_general(pool).await
}