use serde::Serialize;
use chrono::NaiveDate;

#[derive(Debug, Serialize)]
pub struct ReporteGeneral {
    pub nombre_empresa: String,
    pub pais: Option<String>,
    pub nombre_proyecto: String,
    pub fecha_inicio: Option<NaiveDate>,
    pub descripcion_tarea: String,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
    pub nombre_desarrollador: String,
    pub rol_principal: Option<String>,
    pub horas_estimadas: Option<i32>,
}