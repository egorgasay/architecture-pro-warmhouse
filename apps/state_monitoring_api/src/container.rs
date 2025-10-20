use std::sync::Arc;
use crate::domain::repositories::sensor_data::SensorDataRepository;
use crate::domain::services::sensor_data::SensorDataService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::sensor_data::SensorDataRepositoryImpl;
use crate::services::sensor_data::SensorDataServiceImpl;

pub struct Container {
    pub sensor_data_service: Arc<dyn SensorDataService>,
}

impl Container {
    pub fn new() -> Self {
        let pool = Arc::new(db_pool());
        let sensor_data_repository: Arc<dyn SensorDataRepository> = Arc::new(
            SensorDataRepositoryImpl::new(pool.clone())
        );
        let sensor_data_service = Arc::new(
            SensorDataServiceImpl { repository: sensor_data_repository }
        );
        Container { sensor_data_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
