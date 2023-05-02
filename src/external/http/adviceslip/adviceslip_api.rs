use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::domain::entities::adviceslip::Adviceslip;

#[derive(Serialize, Deserialize, Debug)]
pub struct Slip {
    slip: Adviceslip,
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
