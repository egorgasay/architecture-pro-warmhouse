use actix_web::{web, Result, HttpResponse};
use crate::api::dto::sensor_data::{SensorDataDTO, AddSensorDataDTO};
use crate::domain::error::{ApiError};
use crate::domain::services::sensor_data::SensorDataService;
use crate::domain::repositories::sensor_data::SensorDataQueryParams;
use log::{info, error};


pub async fn get_sensor_data_handler(
    sensor_data_service: web::Data<dyn SensorDataService>, params: web::Query<SensorDataQueryParams>,
) -> Result<web::Json<SensorDataDTO>, ApiError> {
    info!("GET /api/v1/sensor/data - sensor_id: {}", params.sensor_id);
    
    match sensor_data_service.get(params.sensor_id).await {
        Ok(sensor_data) => {
            info!("Successfully retrieved sensor data for sensor_id: {}", params.sensor_id);
            Ok(web::Json(sensor_data.into()))
        },
        Err(e) => {
            error!("Failed to retrieve sensor data for sensor_id: {} - error: {:?}", params.sensor_id, e);
            Err(ApiError::from(e))
        }
    }
}

pub async fn add_sensor_data_handler(
    sensor_data_service: web::Data<dyn SensorDataService>, 
    params: web::Query<SensorDataQueryParams>,
    sensor_data: web::Json<AddSensorDataDTO>,
) -> Result<HttpResponse, ApiError> {
    info!("POST /api/v1/sensor/data - sensor_id: {}, data: {:?}", params.sensor_id, sensor_data);
    
    let domain_data: crate::domain::models::sensor_data::SensorData = sensor_data.into_inner().into();
    match sensor_data_service.add(params.sensor_id, domain_data).await {
        Ok(_) => {
            info!("Successfully added sensor data for sensor_id: {}", params.sensor_id);
            Ok(HttpResponse::Ok().json(serde_json::json!({})))
        },
        Err(e) => {
            error!("Failed to add sensor data for sensor_id: {} - error: {:?}", params.sensor_id, e);
            Err(ApiError::from(e))
        }
    }
}