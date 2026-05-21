use sqlx::PgPool;
use crate::models::clientes_corp::{ClienteCorp, CreateClienteCorp};

pub async fn obtener_clientes(pool: &PgPool) -> Result<Vec<ClienteCorp>, sqlx::Error> {
    let clientes = sqlx::query_as!(
        ClienteCorp,
        r#"SELECT id_cliente, nombre_empresa, pais, contacto_principal FROM Clientes_Corp"#
    )
    .fetch_all(pool)
    .await?;
    
    Ok(clientes)
}

pub async fn obtener_cliente_por_id(pool: &PgPool, id: i32) -> Result<ClienteCorp, sqlx::Error> {
    let cliente = sqlx::query_as!(
        ClienteCorp,
        r#"SELECT id_cliente, nombre_empresa, pais, contacto_principal FROM Clientes_Corp WHERE id_cliente = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(cliente)
}

pub async fn crear_cliente(pool: &PgPool, cliente: CreateClienteCorp) -> Result<ClienteCorp, sqlx::Error> {
    let nuevo_cliente = sqlx::query_as!(
        ClienteCorp,
        r#"
        INSERT INTO Clientes_Corp (nombre_empresa, pais, contacto_principal) 
        VALUES ($1, $2, $3) 
        RETURNING id_cliente, nombre_empresa, pais, contacto_principal
        "#,
        cliente.nombre_empresa,
        cliente.pais,
        cliente.contacto_principal
    )
    .fetch_one(pool)
    .await?;

    Ok(nuevo_cliente)
}

pub async fn actualizar_cliente(pool: &PgPool, id: i32, cliente: CreateClienteCorp) -> Result<ClienteCorp, sqlx::Error> {
    let cliente_actualizado = sqlx::query_as!(
        ClienteCorp,
        r#"
        UPDATE Clientes_Corp 
        SET nombre_empresa = $1, pais = $2, contacto_principal = $3 
        WHERE id_cliente = $4 
        RETURNING id_cliente, nombre_empresa, pais, contacto_principal
        "#,
        cliente.nombre_empresa,
        cliente.pais,
        cliente.contacto_principal,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(cliente_actualizado)
}

pub async fn eliminar_cliente(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let resultado = sqlx::query!(
        r#"DELETE FROM Clientes_Corp WHERE id_cliente = $1"#,
        id
    )
    .execute(pool)
    .await?;

    Ok(resultado.rows_affected())
}