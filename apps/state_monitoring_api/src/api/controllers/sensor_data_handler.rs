use actix_web::{web, Result, HttpResponse};
use crate::api::dto::sensor_data::{SensorDataDTO, AddSensorDataDTO};
use crate::domain::error::{ApiError};
use crate::domain::services::sensor_data::SensorDataService;
use crate::domain::repositories::sensor_data::SensorDataQueryParams;


pub async fn get_sensor_data_handler(
    sensor_data_service: web::Data<dyn SensorDataService>, params: web::Query<SensorDataQueryParams>,
) -> Result<web::Json<SensorDataDTO>, ApiError> {
    let sensor_data = sensor_data_service.get(params.sensor_id).await?;
    Ok(web::Json(sensor_data.into()))
}

pub async fn add_sensor_data_handler(
    sensor_data_service: web::Data<dyn SensorDataService>, 
    params: web::Query<SensorDataQueryParams>,
    sensor_data: web::Json<AddSensorDataDTO>,
) -> Result<HttpResponse, ApiError> {
    let _ = sensor_data_service.add(params.sensor_id, sensor_data.into_inner().into()).await?;
    Ok(HttpResponse::Ok().finish())
}