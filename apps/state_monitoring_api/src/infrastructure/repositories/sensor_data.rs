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
use chrono::{NaiveDateTime};

pub struct SensorDataRepositoryImpl {
    pub pool: Arc<DBConn>
}

impl SensorDataRepositoryImpl {
    pub fn new(db: Arc<DBConn>) -> Self {
        SensorDataRepositoryImpl { pool: db }
    }
}

#[async_trait]
impl SensorDataRepository for SensorDataRepositoryImpl {

    
    async fn add(&self, sensor_id: i32, sensor_data: SensorData) -> RepositoryResult<()> {
        use crate::infrastructure::schema::sensor_data::dsl::sensor_data as sensor_data_table;
        let mut conn = self.pool.get().unwrap();
        let _result: usize = run(move || diesel::insert_into(sensor_data_table).values(
            &SensorDataDiesel {
                id: None,
                sensor_id: sensor_id,
                value: sensor_data.value.clone(),
                status: sensor_data.status.clone(),
                ts: NaiveDateTime::parse_from_str(&sensor_data.ts, "%Y-%m-%d %H:%M:%S").unwrap_or_default(),
            }
        ).execute(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(())
    }

    async fn get(&self, sensor_id: i32) -> RepositoryResult<SensorData> {
        use crate::infrastructure::schema::sensor_data::dsl::{sensor_data, sensor_id as sensor_id_col, ts};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            use crate::infrastructure::schema::sensor_data::dsl::id;
            sensor_data
                .filter(sensor_id_col.eq(sensor_id))
                .order((ts.desc(), id.desc()))
                .first::<SensorDataDiesel>(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| v.into())
    }
}