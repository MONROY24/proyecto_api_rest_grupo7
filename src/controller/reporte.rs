use axum::{
    extract::State,
    http::StatusCode,
    Json, Router,
    routing::get,
};
use sqlx::PgPool;
use crate::models::reporte::ReporteGeneral;
use crate::service::reporte_service as service;

pub fn router() -> Router<PgPool> {
    Router::new().route("/", get(obtener_reporte))
}

async fn obtener_reporte(State(pool): State<PgPool>) -> Result<Json<Vec<ReporteGeneral>>, StatusCode> {
    match service::obtener_reporte_general_service(&pool).await {
        Ok(reporte) => Ok(Json(reporte)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}