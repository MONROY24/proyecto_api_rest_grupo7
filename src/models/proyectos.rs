use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Proyecto {
    pub id_proyecto: Option<i32>,
    pub id_cliente: i32,
    pub nombre_proyecto: String,
    pub descripcion: Option<String>,
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub estado: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProyecto {
    pub id_cliente: i32,
    pub nombre_proyecto: String,
    pub descripcion: Option<String>,
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub estado: Option<String>,
}