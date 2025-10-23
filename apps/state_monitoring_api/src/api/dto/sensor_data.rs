use crate::domain::models::sensor_data::{SensorData};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct SensorDataDTO {
    pub id: i32,
    pub value: String,
    pub unit: String,
    pub status: String,
    pub ts: String,
}


#[derive(Debug, Deserialize)]
pub struct AddSensorDataDTO {
    pub value: String,
    pub unit: String,
    pub status: String,
    pub ts: String,
}

impl Into<SensorData> for AddSensorDataDTO {
    fn into(self) -> SensorData {
        SensorData {
            id: 0,
            value: self.value,
            unit: self.unit,
            status: self.status,
            ts: self.ts,
        }
    }
}
