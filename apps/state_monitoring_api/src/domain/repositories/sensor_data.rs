use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::domain::repositories::repository::{RepositoryResult};
use crate::domain::models::sensor_data::{SensorData};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorDataQueryParams {
    pub sensor_id: i32,
}

#[async_trait]
pub trait SensorDataRepository: Send + Sync {
    async fn get(&self, sensor_id: i32) -> RepositoryResult<SensorData>;
    async fn add(&self, sensor_id: i32, sensor_data: SensorData) -> RepositoryResult<()>;
}
