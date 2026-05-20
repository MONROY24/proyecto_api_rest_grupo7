use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Asignacion {
    pub id_asignacion: Option<i32>,
    pub id_tarea: i32,
    pub id_desarrollador: i32,
    pub horas_estimadas: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAsignacion {
    pub id_tarea: i32,
    pub id_desarrollador: i32,
    pub horas_estimadas: Option<i32>,
}