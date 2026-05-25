use sqlx::PgPool;
use crate::models::clientes_corp::{ClienteCorp, CreateClienteCorp};
use crate::repository::clientes_corp as repo;
use crate::utils::util; 

pub async fn obtener_clientes_service(pool: &PgPool) -> Result<Vec<ClienteCorp>, sqlx::Error> {
    repo::obtener_clientes(pool).await
}

pub async fn obtener_cliente_por_id_service(pool: &PgPool, id: i32) -> Result<ClienteCorp, sqlx::Error> {
    repo::obtener_cliente_por_id(pool, id).await
}

pub async fn crear_cliente_service(
    pool: &PgPool, 
    mut cliente: CreateClienteCorp
) -> Result<ClienteCorp, sqlx::Error> {
    
    cliente.nombre_empresa = util::limpiar_nombre(&cliente.nombre_empresa);
    
    repo::crear_cliente(pool, cliente).await
}

pub async fn actualizar_cliente_service(
    pool: &PgPool, 
    id: i32, 
    mut cliente: CreateClienteCorp
) -> Result<ClienteCorp, sqlx::Error> {
    
    cliente.nombre_empresa = util::limpiar_nombre(&cliente.nombre_empresa);
    
    repo::actualizar_cliente(pool, id, cliente).await
}

pub async fn eliminar_cliente_service(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    repo::eliminar_cliente(pool, id).await
}