use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Slip {
    slip: Adviceslip,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Adviceslip {
    id: i64,
    advice: String,
}

impl Adviceslip {
    pub async fn get_advice() -> Result<Slip, Error> {
        let body = reqwest::get("https://api.adviceslip.com/advice")
            .await?
            .json::<Slip>()
            .await?;

        Ok(body)
    }
}
