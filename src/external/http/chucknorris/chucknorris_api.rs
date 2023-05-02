use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::domain::entities::chucknorris::ChuckNorris;

impl ChuckNorris {
    pub async fn get_random() -> Result<Self, Error> {
        let body = reqwest::get("https://api.chucknorris.io/jokes/random")
            .await?
            .json::<ChuckNorris>()
            .await?;

        Ok(body)
    }
}
