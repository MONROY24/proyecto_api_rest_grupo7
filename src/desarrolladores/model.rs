use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Desarrollador {
    pub id_desarrollador: i32,
    pub nombre: String,
    pub rol_principal: Option<String>,
    pub nivel: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NuevoDesarrollador {
    pub nombre: String,
    pub rol_principal: Option<String>,
    pub nivel: Option<String>,
}