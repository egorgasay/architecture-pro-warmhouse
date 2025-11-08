use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::sensor_data::{SensorData};
use crate::domain::repositories::sensor_data::{SensorDataRepository};
use crate::domain::services::sensor_data::SensorDataService;
use log::{info, error, debug};

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
        debug!("SensorDataService::get called for sensor_id: {}", sensor_id);
        
        match self.repository.get(sensor_id).await {
            Ok(data) => {
                info!("SensorDataService::get - successfully retrieved data for sensor_id: {}", sensor_id);
                Ok(data)
            },
            Err(e) => {
                error!("SensorDataService::get - failed to retrieve data for sensor_id: {} - error: {:?}", sensor_id, e);
                Err(CommonError {
                    message: format!("Failed to retrieve sensor data: {}", e.message),
                    code: 404,
                })
            }
        }
    }

    async fn add(&self, sensor_id: i32, sensor_data: SensorData) -> Result<(), CommonError> {
        debug!("SensorDataService::add called for sensor_id: {}, data: {:?}", sensor_id, sensor_data);
        
        match self.repository.add(sensor_id, sensor_data.clone()).await {
            Ok(_) => {
                info!("SensorDataService::add - successfully added data for sensor_id: {}", sensor_id);
                Ok(())
            },
            Err(e) => {
                error!("SensorDataService::add - failed to add data for sensor_id: {} - error: {:?}", sensor_id, e);
                Err(CommonError {
                    message: format!("Failed to add sensor data: {}", e.message),
                    code: 422,
                })
            }
        }
    }
}
