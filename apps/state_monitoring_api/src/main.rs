use std::sync::Arc;
use actix_web::{HttpServer};
use statemon::{container::Container, create_app::create_app};
use env_logger;
use std::env;
use log::{info};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Инициализация логирования
    env_logger::init();
    
    info!("Starting State Monitoring API server");
    
    let container = Arc::new(Container::new());
    info!("Container initialized successfully");

    let port = env::var("PORT").unwrap_or_else(|_| "7676".to_string());
    
    let server = HttpServer::new(move || { create_app(container.clone()) })
        .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?;
    
    info!("Server bound to 0.0.0.0:{}", port);
    info!("State Monitoring API server is running");
    
    server.run().await
}

