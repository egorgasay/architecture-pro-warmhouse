use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::sensor_data::{SensorData};

#[async_trait]
pub trait SensorDataService: 'static + Sync + Send {
    // async fn create(&self, todo: CreateTodo) -> Result<Todo, CommonError>;
    async fn get(&self, sensor_id: i32) -> Result<SensorData, CommonError>;
    async fn add(&self, sensor_id: i32, sensor_data: SensorData) -> Result<(), CommonError>;
}