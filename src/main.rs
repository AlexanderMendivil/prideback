mod services;
use crate::services::get_services;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "test api rust!"
}

#[get("/guests")]
    pub fn guests() -> &'static str {
        "guests"
    }

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![guests])
}