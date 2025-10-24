use std::sync::Arc;
use actix_web::{App, web, HttpRequest};
use actix_web::{Error};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::error::{JsonPayloadError, QueryPayloadError};
use crate::api::controllers::sensor_data_handler::{get_sensor_data_handler, add_sensor_data_handler};
use crate::container::Container;
use crate::domain::error::ApiError;

fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let api_error = ApiError::bad_request(format!("JSON deserialize error: {}", err));
    Error::from(api_error)
}

fn query_error_handler(err: QueryPayloadError, _req: &HttpRequest) -> Error {
    let api_error = ApiError::bad_request(format!("Query parameter error: {}", err));
    Error::from(api_error)
}

pub fn create_app(container: Arc<Container>) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let sensor_data_service = container.sensor_data_service.clone();

    App::new()
        .app_data(web::Data::from(sensor_data_service.clone()))
        .app_data(
            web::JsonConfig::default()
                .error_handler(json_error_handler)
        )
        .app_data(
            web::QueryConfig::default()
                .error_handler(query_error_handler)
        )
        .wrap(Logger::default())
        .service(
            web::scope("/api/v1/sensor")
                .route("/data", web::get().to(get_sensor_data_handler))
                .route("/data", web::post().to(add_sensor_data_handler))
        )
}
