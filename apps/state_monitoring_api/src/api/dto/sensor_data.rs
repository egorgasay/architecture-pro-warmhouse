use crate::domain::models::sensor_data::{SensorData};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct SensorDataDTO {
    pub id: i32,
    pub value: f64,
    pub unit: String,
    pub status: String,
    pub created_at: String,
}


#[derive(Debug, Deserialize)]
pub struct AddSensorDataDTO {
    pub value: f64,
    pub unit: String,
    pub status: String,
    pub created_at: String,
}

impl Into<SensorData> for AddSensorDataDTO {
    fn into(self) -> SensorData {
        SensorData {
            id: 0,
            value: self.value,
            unit: self.unit,
            status: self.status,
            created_at: self.created_at,
        }
    }
}
