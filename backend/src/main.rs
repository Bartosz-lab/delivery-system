#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{
    http,
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use diesel::{prelude::*, r2d2};
use dotenv::dotenv;
use std::error::Error;
use utoipa_swagger_ui::SwaggerUi;

type PgPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
type IMPool = ();

mod auth;
mod config;
mod delivery;
mod schema;

use config::Config;

pub struct AppState {
    env: Config,
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::init();

    let pool = initialize_db_pool(config.database_url.clone());

    let mut swagger_urls = Vec::new();
    swagger_urls.append(auth::app::api::auth::swagger_urls().as_mut());
    swagger_urls.append(auth::app::api::users::swagger_urls().as_mut());
    swagger_urls.append(delivery::app::api::trade_partner::swagger_urls().as_mut());
    swagger_urls.append(delivery::app::api::parcel::swagger_urls().as_mut());
    swagger_urls.append(delivery::app::api::report::swagger_urls().as_mut());

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .supports_credentials()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(()))
            .app_data(web::Data::new(AppState {
                env: config.clone(),
            }))
            .wrap(cors)
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
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

pub fn initialize_db_pool(database_url: String) -> PgPool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url.clone());
    r2d2::Pool::builder()
        .build(manager)
        .expect(format!("Error connecting to {}", database_url).as_str())
}
