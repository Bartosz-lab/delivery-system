#[macro_use]
extern crate lazy_static;

use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use dotenv::dotenv;
use std::error::Error;
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod config;
mod delivery;

use config::Config;

pub struct AppState {
    env: Config,
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::init();

    let mut swagger_urls = Vec::new();
    swagger_urls.append(auth::app::api::auth::swagger_urls().as_mut());
    swagger_urls.append(auth::app::api::users::swagger_urls().as_mut());
    swagger_urls.append(delivery::app::api::trade_partner::swagger_urls().as_mut());
    swagger_urls.append(delivery::app::api::parcel::swagger_urls().as_mut());
    swagger_urls.append(delivery::app::api::report::swagger_urls().as_mut());

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                env: config.clone(),
            }))
            .wrap(Logger::new("%a \"%r\" %s %b %T"))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(swagger_urls.clone()))
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
            .service(
                web::scope("/tradepartner")
                    .wrap(NormalizePath::trim())
                    .configure(delivery::app::api::trade_partner::config),
            )
            .service(
                web::scope("/parcel")
                    .wrap(NormalizePath::trim())
                    .configure(delivery::app::api::parcel::config),
            )
            .service(
                web::scope("/report")
                    .wrap(NormalizePath::trim())
                    .configure(delivery::app::api::report::config),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
