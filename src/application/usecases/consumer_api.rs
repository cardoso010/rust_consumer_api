use reqwest::Error;

use crate::domain::entities::{adviceslip::Adviceslip, chucknorris::ChuckNorris};
use crate::external::database::repositories::{
    adviceslip_respository::AdviceslipRepository, chucknorris_repository::ChucknorrisRepository,
};
use crate::external::http::adviceslip::adviceslip_api::Slip;

#[derive(Debug)]
enum Response {
    Chuck(ChuckNorris),
    Advise(Slip),
}

pub async fn execute(consumer_api: String, save_request: bool) {
    match get_response(consumer_api).await {
        Ok(Response::Chuck(chuck)) => println!("ChuckNorris: {:?}", chuck),
        Ok(Response::Advise(advise)) => println!("Advise: {:?}", advise),
        Err(error) => println!("Error: {:?}", error),
    };
}

async fn get_response(consumer_api: String) -> Result<Response, Error> {
    match consumer_api.as_str() {
        "chucknorris" => Ok(Response::Chuck(ChuckNorris::get_random().await?)),
        "adviceslip" => Ok(Response::Advise(Adviceslip::get_advice().await?)),
        _ => panic!("You need fill the api_name option!"),
    }
}
