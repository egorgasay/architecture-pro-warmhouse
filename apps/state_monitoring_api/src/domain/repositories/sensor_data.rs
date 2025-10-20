use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::domain::repositories::repository::{QueryParams, ResultPaging, RepositoryResult, DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::domain::models::sensor_data::{Todo, CreateTodo, SensorData};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for TodoQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorDataQueryParams {
    pub sensor_id: i32,
}

#[async_trait]
pub trait SensorDataRepository: Send + Sync {
    // async fn create(&self, new_todo: &CreateSensorData) -> RepositoryResult<SensorData>;
    async fn get(&self, sensor_id: i32) -> RepositoryResult<SensorData>;
}
