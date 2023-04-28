use reqwest::Error;

use crate::external::http::adviceslip::adviceslip_api::{Adviceslip, Slip};
use crate::external::http::chucknorris::chucknorris_api::ChuckNorris;

#[derive(Debug)]
enum Response {
    Chuck(ChuckNorris),
    Advise(Slip),
}

pub async fn execute(consumer_api: String) {
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
