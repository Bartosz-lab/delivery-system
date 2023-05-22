use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![login, logout]
}

#[get("/login")]
pub fn login() -> &'static str {
    "Login page"
}

#[get("/logout")]
pub fn logout() -> &'static str {
    "Logout page"
}
