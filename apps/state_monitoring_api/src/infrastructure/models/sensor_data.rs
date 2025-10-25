use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::domain::models::sensor_data::SensorData;
use crate::infrastructure::schema::sensor_data;
use crate::infrastructure::utils::{parse_datetime, format_datetime_safe};

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = sensor_data)]
pub struct SensorDataDiesel {
    pub id: Option<i32>,
    pub sensor_id: i32,
    pub value: f64,
    pub unit: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}

// Factory method for creating a new SensorDataDiesel from a SensorData
impl From<SensorData> for SensorDataDiesel {
    fn from(t: SensorData) -> Self {
        // Парсим дату из строки
        let parsed_datetime = parse_datetime(&t.created_at);

        SensorDataDiesel {
            id: Some(t.id),
            sensor_id: 0,
            value: t.value,
            unit: t.unit,
            status: t.status,
            created_at: parsed_datetime.naive_utc(),
        }
    }
}

impl Into<SensorData> for SensorDataDiesel {
    fn into(self) -> SensorData {
        SensorData {
            id: self.id.unwrap_or(0),
            value: self.value,
            unit: self.unit,
            status: self.status,
            created_at: format_datetime_safe(&self.created_at),
        }
    }
}
