#[macro_use]
extern crate lazy_static;

use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use dotenv::dotenv;
use std::error::Error;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

mod auth;
mod config;

use config::Config;

pub struct AppState {
    env: Config,
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                env: config.clone(),
            }))
            .wrap(Logger::new("%a \"%r\" %s %b %T"))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (
                    Url::new("user", "/api-docs/user.json"),
                    auth::app::api::users::ApiDoc::openapi(),
                ),
                (
                    Url::new("auth", "/api-docs/auth.json"),
                    auth::app::api::auth::ApiDoc::openapi(),
                ),
            ]))
            .service(
                web::scope("/auth")
                    .wrap(NormalizePath::trim())
                    .configure(auth::app::api::auth::config),
            )
            .service(
                web::scope("/user")
                    .wrap(NormalizePath::trim())
                    .configure(auth::app::api::users::config),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
