use crate::domain::models::sensor_data::{SensorData};
use serde::{Serialize, Deserialize};

// #[derive(Deserialize, Serialize)]
// pub struct CreateTodoDTO {
//     pub title: String,
//     pub description: String,
// }

#[derive(Debug, Serialize)]
pub struct SensorDataDTO {
    pub id: i32,
    pub value: String,
    pub status: String,
    pub ts: String,
}


#[derive(Debug, Deserialize)]
pub struct AddSensorDataDTO {
    pub value: String,
    pub status: String,
    pub ts: String,
}

impl Into<SensorData> for AddSensorDataDTO {
    fn into(self) -> SensorData {
        SensorData {
            id: 0,
            value: self.value,
            status: self.status,
            ts: self.ts,
        }
    }
}

// impl Into<CreateTodo> for CreateTodoDTO {
//     fn into(self) -> CreateTodo {
//         CreateTodo {
//             title: self.title,
//             description: self.description,
//         }
//     }
// }

// impl Into<CreateTodoDTO> for CreateTodo {
//     fn into(self) -> CreateTodoDTO {
//         CreateTodoDTO {
//             title: self.title,
//             description: self.description,
//         }
//     }
// }

// impl Into<ResultPaging<TodoDTO>> for ResultPaging<Todo> {
//     fn into(self) -> ResultPaging<TodoDTO> {
//         ResultPaging {
//             total: self.total,
//             items: self.items.into_iter().map(|todo| todo.into()).collect(),
//         }
//     }
// }