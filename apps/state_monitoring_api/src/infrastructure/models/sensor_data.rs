use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::domain::models::sensor_data::SensorData;
use chrono::DateTime;
use crate::infrastructure::schema::sensor_data;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = sensor_data)]
pub struct SensorDataDiesel {
    pub id: Option<i32>,
    pub sensor_id: i32,
    pub value: String,
    pub status: String,
    pub ts: NaiveDateTime,
}

// Factory method for creating a new SensorDataDiesel from a SensorData
impl From<SensorData> for SensorDataDiesel {
    fn from(t: SensorData) -> Self {
        SensorDataDiesel {
            id: Some(t.id),
            sensor_id: 0,
            value: t.value,
            status: t.status,
            ts: DateTime::from_timestamp(t.ts.parse::<i64>().unwrap_or(0), 0).unwrap().naive_utc(),
        }
    }
}

impl Into<SensorData> for SensorDataDiesel {
    fn into(self) -> SensorData {
        SensorData {
            id: self.id.unwrap_or(0),
            value: self.value,
            status: self.status,
            ts: self.ts.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
