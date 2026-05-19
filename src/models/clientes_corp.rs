use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClienteCorp {
    pub id_cliente: Option<i32>,
    pub nombre_empresa: String,
    pub pais: Option<String>,
    pub contacto_principal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClienteCorp {
    pub nombre_empresa: String,
    pub pais: Option<String>,
    pub contacto_principal: Option<String>,
}