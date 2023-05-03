use crate::domain::entities::chucknorris::ChuckNorris;
use crate::external::database::repositories::{adviceslip_respository, chucknorris_repository};
use crate::external::http::adviceslip::adviceslip_api::Slip;

pub fn maybe_save_chuck(chuck: ChuckNorris, save_request: bool) {
    println!("ChuckNorris: {:?}", chuck);
    if save_request {
        save_chuck(chuck)
    }
}

fn save_chuck(chuck: ChuckNorris) {
    match chucknorris_repository::save(chuck) {
        Ok(_value) => println!("Chuck saved with success!"),
        Err(err) => println!("Error on save chuck: {}", err),
    }
}

pub fn maybe_save_advise(slip: Slip, save_request: bool) {
    println!("Advise: {:?}", slip);
    if save_request {
        save_advise(slip)
    }
}

fn save_advise(slip: Slip) {
    match adviceslip_respository::save(slip.slip) {
        Ok(_value) => println!("Advice saved with success!"),
        Err(err) => println!("Error on save advice: {}", err),
    }
}
