use serde::{Deserialize, Serialize};
use chrono::NaiveDate; // Necesitarás agregar 'chrono' a tu Cargo.toml si no está

#[derive(Debug, Serialize, Deserialize)]
pub struct Proyecto {
    pub id_proyecto: i32,
    pub nombre_proyecto: String,
    pub fecha_inicio: Option<NaiveDate>,
    pub id_cliente: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProyecto {
    pub nombre_proyecto: String,
    pub fecha_inicio: Option<NaiveDate>,
    pub id_cliente: Option<i32>,
}