use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChuckNorris {
    categories: Vec<String>,
    created_at: String,
    icon_url: String,
    id: String,
    updated_at: String,
    url: String,
}

impl ChuckNorris {
    pub async fn get_random() -> Result<Self, Error> {
        let body = reqwest::get("https://api.chucknorris.io/jokes/random")
            .await?
            .json::<ChuckNorris>()
            .await?;

        Ok(body)
    }
}
