use std::sync::Arc;
use crate::domain::repositories::sensor_data::SensorDataRepository;
use crate::domain::services::service_context::ServiceContextService;
use crate::domain::services::sensor_data::SensorDataService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::sensor_data::SensorDataRepositoryImpl;
use crate::infrastructure::services::service_context::ServiceContextServiceImpl;
use crate::services::sensor_data::SensorDataServiceImpl;

pub struct Container {
    pub sensor_data_service: Arc<dyn SensorDataService>,
    pub service_context_service: Arc<dyn ServiceContextService>
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
        let service_context_service = Arc::new(
            ServiceContextServiceImpl::new(pool.clone())
        );
        Container { sensor_data_service, service_context_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
