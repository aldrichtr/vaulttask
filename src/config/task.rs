
use serde::{Serialize, Deserialize};

// TaskConfig is the `task` key in the configuration:
// ```yaml
// task:
//   id:
//     type: nanoid
//     length: 23
// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskConfig {
    pub id: TaskIdConfig,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            id: TaskIdConfig::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskIdConfig {
    #[serde(rename = "type")]
    pub kind: String,
    pub length: u8,
}

impl Default for TaskIdConfig {
    fn default() -> Self {
        Self {
            kind: String::from("nanoid"),
            length: 23,
        }
    }
}

// endregion task
