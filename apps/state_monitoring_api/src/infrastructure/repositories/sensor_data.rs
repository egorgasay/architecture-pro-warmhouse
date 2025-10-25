use std::sync::Arc;
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::domain::models::sensor_data::{SensorData};
use crate::domain::repositories::repository::{RepositoryResult};
use crate::domain::repositories::sensor_data::{SensorDataRepository};
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::sensor_data::{SensorDataDiesel};
use crate::infrastructure::utils::parse_datetime;
use log::{info, error, debug, warn};

pub struct SensorDataRepositoryImpl {
    pub pool: Arc<DBConn>
}

impl SensorDataRepositoryImpl {
    pub fn new(db: Arc<DBConn>) -> Self {
        info!("SensorDataRepositoryImpl::new - initializing repository with database pool");
        SensorDataRepositoryImpl { pool: db }
    }
}

#[async_trait]
impl SensorDataRepository for SensorDataRepositoryImpl {

    
    async fn add(&self, sensor_id: i32, sensor_data: SensorData) -> RepositoryResult<()> {
        debug!("SensorDataRepositoryImpl::add - adding data for sensor_id: {}, data: {:?}", sensor_id, sensor_data);
        
        use crate::infrastructure::schema::sensor_data::dsl::sensor_data as sensor_data_table;
        
        let mut conn = match self.pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                error!("SensorDataRepositoryImpl::add - failed to get database connection for sensor_id: {}", sensor_id);
                return Err(DieselRepositoryError::from(diesel::result::Error::BrokenTransactionManager).into_inner());
            }
        };
        
        // Парсим дату с поддержкой различных форматов
        let parsed_datetime = parse_datetime(&sensor_data.created_at);

        let diesel_data = SensorDataDiesel {
            id: None,
            sensor_id: sensor_id,
            value: sensor_data.value.clone(),
            unit: sensor_data.unit.clone(),
            status: sensor_data.status.clone(),
            created_at: parsed_datetime.naive_utc(),
        };
        
        let result: usize = run(move || diesel::insert_into(sensor_data_table).values(&diesel_data).execute(&mut conn))
            .await
            .map_err(|v| {
                error!("SensorDataRepositoryImpl::add - database error for sensor_id: {} - error: {:?}", sensor_id, v);
                DieselRepositoryError::from(v).into_inner()
            })?;
            
        info!("SensorDataRepositoryImpl::add - successfully inserted {} rows for sensor_id: {}", result, sensor_id);
        Ok(())
    }

    async fn get(&self, sensor_id: i32) -> RepositoryResult<SensorData> {
        debug!("SensorDataRepositoryImpl::get - retrieving data for sensor_id: {}", sensor_id);
        
        use crate::infrastructure::schema::sensor_data::dsl::{sensor_data, sensor_id as sensor_id_col, created_at};
        
        let mut conn = match self.pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                error!("SensorDataRepositoryImpl::get - failed to get database connection for sensor_id: {}", sensor_id);
                return Err(DieselRepositoryError::from(diesel::result::Error::BrokenTransactionManager).into_inner());
            }
        };
        
        let result = run(move || {
            use crate::infrastructure::schema::sensor_data::dsl::id;
            sensor_data
                .filter(sensor_id_col.eq(sensor_id))
                .order((created_at.desc(), id.desc()))
                .first::<SensorDataDiesel>(&mut conn)
        })
            .await
            .map_err(|v| {
                error!("SensorDataRepositoryImpl::get - database error for sensor_id: {} - error: {:?}", sensor_id, v);
                DieselRepositoryError::from(v).into_inner()
            })
            .map(|v| v.into());
            
        match &result {
            Ok(_) => info!("SensorDataRepositoryImpl::get - successfully retrieved data for sensor_id: {}", sensor_id),
            Err(_) => warn!("SensorDataRepositoryImpl::get - no data found for sensor_id: {}", sensor_id),
        }
        
        result
    }
}