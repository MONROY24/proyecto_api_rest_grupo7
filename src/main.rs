mod service;
mod utils;
mod models;
mod controller;
mod config;
mod repository;

use config::config::crear_pool;
use axum::Router;

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}\n");

    let pool = crear_pool()
        .await
        .expect("No se pudo conectar a la base de datos");

    // Batería de pruebas para Clientes_Corp
    // Esta sección realiza pruebas de creación, consulta, actualización y eliminación de clientes corporativos
    //Modifirlo segun tu tabla y modelo, es solo un ejemplo de como probar cada endpoint de tu API
    println!("--- INICIANDO BATERÍA DE PRUEBAS (TABLA: Clientes_Corp) ---\n");

    let cliente_nuevo = models::clientes_corp::CreateClienteCorp {
        nombre_empresa: "Tech Solutions SV".to_string(),
        pais: Some("El Salvador".to_string()),
        contacto_principal: Some("Melvin Monroy".to_string()),
    };
    
    println!("1. [POST] Intentando crear cliente...");
    let cliente_creado = repository::clientes_corp::crear_cliente(&pool, cliente_nuevo).await.unwrap();
    let id_creado = cliente_creado.id_cliente.expect("El ID no debería ser nulo tras crear");
    println!("   -> ¡Éxito! Cliente creado con ID asignado: {}\n", id_creado);

    println!("2. [GET] Consultando todos los clientes...");
    let clientes = repository::clientes_corp::obtener_clientes(&pool).await.unwrap();
    println!("   -> ¡Éxito! Total de clientes encontrados en la base de datos: {}\n", clientes.len());

    println!("3. [GET por ID] Consultando específicamente el cliente con ID {}...", id_creado);
    let cliente_id = repository::clientes_corp::obtener_cliente_por_id(&pool, id_creado).await.unwrap();
    println!("   -> ¡Éxito! Empresa recuperada: {} (País: {:?})\n", cliente_id.nombre_empresa, cliente_id.pais);

    println!("4. [PUT] Actualizando los datos del cliente con ID {}...", id_creado);
    let cliente_actualizar = models::clientes_corp::CreateClienteCorp {
        nombre_empresa: "Tech Solutions SV Internacional".to_string(),
        pais: Some("Costa Rica".to_string()),
        contacto_principal: Some("Josué (Departamento de Soporte)".to_string()),
    };
    let cliente_actualizado = repository::clientes_corp::actualizar_cliente(&pool, id_creado, cliente_actualizar).await.unwrap();
    println!("   -> ¡Éxito! Cliente actualizado. Nuevo Contacto: {:?}\n", cliente_actualizado.contacto_principal);

    println!("5. [DELETE] Eliminando el cliente de prueba con ID {}...", id_creado);
    let filas_eliminadas = repository::clientes_corp::eliminar_cliente(&pool, id_creado).await.unwrap();
    println!("   -> ¡Éxito! Cliente eliminado correctamente. Filas afectadas: {}\n", filas_eliminadas);

    println!("--- BATERÍA DE PRUEBAS COMPLETADA AL 100% SIN ERRORES ---");

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}

fn unificar_routers(_pool: sqlx::PgPool) -> axum::Router {
    Router::new()
}