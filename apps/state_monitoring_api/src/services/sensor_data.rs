use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::sensor_data::{SensorData};
use crate::domain::repositories::sensor_data::{SensorDataRepository};
use crate::domain::services::sensor_data::SensorDataService;

#[derive(Clone)]
pub struct SensorDataServiceImpl {
    pub repository: Arc<dyn SensorDataRepository>,
}

impl SensorDataServiceImpl {
    pub fn new(repository: Arc<dyn SensorDataRepository>) -> Self {
        SensorDataServiceImpl {
            repository,
        }
    }
}

#[async_trait]
impl SensorDataService for SensorDataServiceImpl {
    async fn get(&self, sensor_id: i32) -> Result<SensorData, CommonError> {
        self.repository
            .get(sensor_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
