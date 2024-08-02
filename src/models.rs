use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub field1: String,
    pub field2: String,
}
