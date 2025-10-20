use crate::domain::models::sensor_data::{SensorData};
use serde::{Serialize, Deserialize};

// #[derive(Deserialize, Serialize)]
// pub struct CreateTodoDTO {
//     pub title: String,
//     pub description: String,
// }

#[derive(Debug, Serialize)]
pub struct SensorDataDTO {
    id: i32,
    value: String,
    status: String,
    ts: String,
}

impl Into<SensorDataDTO> for SensorData {
    fn into(self) -> SensorDataDTO {
        SensorDataDTO {
            id: self.id,
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