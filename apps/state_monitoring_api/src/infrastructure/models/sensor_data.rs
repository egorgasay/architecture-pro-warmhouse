use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::domain::models::sensor_data::SensorData;
use crate::infrastructure::schema::sensor_data;

#[derive(Queryable)]
#[diesel(table_name = sensor_data)]
pub struct SensorDataDiesel {
    pub id: i32,
    pub value: String,
    pub status: String,
    pub ts: NaiveDateTime,
}

// Factory method for creating a new SensorDataDiesel from a SensorData
impl From<SensorData> for SensorDataDiesel {
    fn from(t: SensorData) -> Self {
        SensorDataDiesel {
            id: t.id,
            value: t.value,
            status: t.status,
            ts: NaiveDateTime::parse_from_str(&t.ts, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
        }
    }
}

impl Into<SensorData> for SensorDataDiesel {
    fn into(self) -> SensorData {
        SensorData {
            id: self.id,
            value: self.value,
            status: self.status,
            ts: self.ts.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

// #[derive(Insertable)]
// #[diesel(table_name = todos)]
// pub struct CreateTodoDiesel {
//     pub title: String,
//     pub description: String,
// }

// // Factory method for creating a new Todo from a TodoDiesel
// impl Into<Todo> for TodoDiesel {
//     fn into(self) -> Todo {
//         Todo {
//             id: self.id,
//             title: self.title,
//             description: self.description,
//             completed: self.completed,
//         }
//     }
// }

// impl From<CreateTodo> for CreateTodoDiesel {
//     fn from(t: CreateTodo) -> Self {
//         CreateTodoDiesel {
//             title: t.title,
//             description: t.description,
//         }
//     }
// }

// impl Into<Todo> for CreateTodoDiesel {
//     fn into(self) -> Todo {
//         Todo {
//             id: 0,
//             title: self.title,
//             description: self.description,
//             completed: false,
//         }
//     }
// }