use sqlx::PgPool;

use crate::models::proyectos::{Proyecto, CreateProyecto};
use crate::repository::proyectos_repository as repo;
use crate::utils::util; 

pub async fn obtener_proyectos(pool: &PgPool) -> Result<Vec<Proyecto>, sqlx::Error> {
    repo::obtener_proyectos(pool).await
}

pub async fn obtener_proyecto_por_id(pool: &PgPool, id: i32) -> Result<Proyecto, sqlx::Error> {
    repo::obtener_proyecto_por_id(pool, id).await
}

pub async fn crear_proyecto(
    pool: &PgPool,
    mut proyecto: CreateProyecto, 
) -> Result<Proyecto, sqlx::Error> {

    if !util::texto_es_valido(&proyecto.nombre_proyecto) {
        panic!("El nombre del proyecto no puede estar vacío");
    }
    
    proyecto.nombre_proyecto = util::limpiar_nombre(&proyecto.nombre_proyecto);

    repo::crear_proyecto(pool, proyecto).await
}

pub async fn actualizar_proyecto(
    pool: &PgPool,
    id: i32,
    mut proyecto: CreateProyecto,
) -> Result<Proyecto, sqlx::Error> {

    if !util::texto_es_valido(&proyecto.nombre_proyecto) {
        panic!("El nombre del proyecto no puede estar vacío");
    }

    proyecto.nombre_proyecto = util::limpiar_nombre(&proyecto.nombre_proyecto);

    repo::actualizar_proyecto(pool, id, proyecto).await
}

pub async fn eliminar_proyecto(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    repo::eliminar_proyecto(pool, id).await
}