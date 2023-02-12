#[macro_use] extern crate rocket;

// fn main() {
//     println!("Hello, world!");
// }

#[get("/")]
fn index() -> &'static str {
    "test api rust!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}