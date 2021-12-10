use rocket::local::Client;
use rocket_api::rocket_builder;

pub fn setup () -> Client {
    Client::new(rocket_builder()).expect("Valid Rocket instance")
}