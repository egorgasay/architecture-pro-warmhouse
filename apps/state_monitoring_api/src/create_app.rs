use std::sync::Arc;
use actix_web::{App, web};
use actix_web::{Error};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use crate::api::controllers::sensor_data_handler::{get_sensor_data_handler};
use crate::container::Container;

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
        .wrap(Logger::default())
        .service(
            web::scope("/api/v1/sensor")
                .route("/data", web::get().to(get_sensor_data_handler))
        )
}
