use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tarea {
    pub id_tarea: Option<i32>,
    pub id_proyecto: Option<i32>,
    pub descripcion: String,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTarea {
    pub id_proyecto: Option<i32>,
    pub descripcion: String,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
}
