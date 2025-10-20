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

    // async fn create(&self, new_todo: &CreateTodo) -> RepositoryResult<Todo> {
    //     use crate::infrastructure::schema::todos::dsl::todos;
    //     let new_todo_diesel: CreateTodoDiesel = CreateTodoDiesel::from(new_todo.clone());
    //     let mut conn = self.pool.get().unwrap();
    //     let result: TodoDiesel = run(move || diesel::insert_into(todos).values(new_todo_diesel)
    //         .get_result(&mut conn))
    //         .await
    //         .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
    //     Ok(result.into())
    // }

    async fn get(&self, sensor_id: i32) -> RepositoryResult<SensorData> {
        use crate::infrastructure::schema::sensor_data::dsl::{id, sensor_data, ts};
        let mut conn = self.pool.get().unwrap();
        run(move || sensor_data.filter(id.eq(sensor_id)).order(ts.desc()).first::<SensorDataDiesel>(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| v.into())
    }
}