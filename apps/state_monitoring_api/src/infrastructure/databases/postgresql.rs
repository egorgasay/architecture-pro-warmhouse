use std::env;

use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use log::{info, error};

use crate::domain::constants::POSTGRESQL_DB_URI;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBConn = PostgresPool;

pub fn db_pool() -> DBConn {
    info!("Initializing database connection pool");
    
    dotenv().ok();
    
    let database_url = match env::var(POSTGRESQL_DB_URI) {
        Ok(url) => {
            info!("Database URL loaded from environment variable: {}", POSTGRESQL_DB_URI);
            url
        },
        Err(_) => {
            error!("Failed to load database URL from environment variable: {}", POSTGRESQL_DB_URI);
            panic!("{} must be set", POSTGRESQL_DB_URI);
        }
    };
    
    info!("Creating connection manager for PostgreSQL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    match Pool::builder().build(manager) {
        Ok(pool) => {
            info!("Database connection pool created successfully");
            pool
        },
        Err(e) => {
            error!("Failed to create database connection pool: {:?}", e);
            panic!("Failed to create pool: {:?}", e);
        }
    }
}
