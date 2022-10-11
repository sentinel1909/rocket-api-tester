#[macro_use]
extern crate rocket;

use rocket::http::Status;

#[get("/health_check")]
fn health_check() -> (Status, &'static str) {
    (Status::Ok, "200 OK")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check])
}
