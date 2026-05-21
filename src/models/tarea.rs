use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tarea {
    pub id: Uuid,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub estado: String,
    pub prioridad: String,
    pub proyecto_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CrearTarea {
    pub titulo: String,
    pub descripcion: Option<String>,
    pub estado: String,
    pub prioridad: String,
    pub proyecto_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarTarea {
    pub titulo: Option<String>,
    pub descripcion: Option<String>,
    pub estado: Option<String>,
    pub prioridad: Option<String>,
}
