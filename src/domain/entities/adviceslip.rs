use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Adviceslip {
    pub id: i64,
    pub advice: String,
}
