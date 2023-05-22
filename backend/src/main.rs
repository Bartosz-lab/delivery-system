#[macro_use]
extern crate rocket;

pub mod auth;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/auth", auth::app::views::routes())
}
