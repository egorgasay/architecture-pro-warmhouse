use serde::Deserialize;
use crate::api::dto::sensor_data::SensorDataDTO;

#[derive(Clone, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Clone)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

#[derive(Clone)]
pub struct SensorData {
    pub id: i32,
    pub value: String,
    pub unit: String,
    pub status: String,
    pub ts: String,
}


impl From<SensorData> for SensorDataDTO {
    fn from(sensor_data: SensorData) -> Self {
        SensorDataDTO {
            id: sensor_data.id,
            value: sensor_data.value,
            unit: sensor_data.unit,
            status: sensor_data.status,
            ts: sensor_data.ts,
        }
    }
}