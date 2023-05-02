use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChuckNorris {
    pub categories: Vec<String>,
    pub created_at: String,
    pub icon_url: String,
    pub id: String,
    pub updated_at: String,
    pub url: String,
}
