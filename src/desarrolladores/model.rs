use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Desarrollador {
    pub id: i32,
    pub nombre: String,
    pub lenguaje: String,
    pub experiencia: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NuevoDesarrollador {
    pub nombre: String,
    pub lenguaje: String,
    pub experiencia: i32,
}