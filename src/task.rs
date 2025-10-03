use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task
{
    pub id: u32,
    pub title: String,
    #[serde(default)]
    pub done: bool,
}
