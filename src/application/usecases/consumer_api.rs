use reqwest::Error;

use crate::domain::entities::{adviceslip::Adviceslip, chucknorris::ChuckNorris};
use crate::application::usecases::save_request;
use crate::external::http::adviceslip::adviceslip_api::Slip;

#[derive(Debug)]
enum Response {
    Chuck(ChuckNorris),
    Advise(Slip),
}

pub async fn execute(consumer_api: String, save_request: bool) {
    match get_response(consumer_api).await {
        Ok(Response::Chuck(chuck)) => save_request::maybe_save_chuck(chuck, save_request),
        Ok(Response::Advise(advise)) => save_request::maybe_save_advise(advise, save_request),
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
