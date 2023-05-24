#[macro_use]
extern crate lazy_static;

use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use std::error::Error;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

mod auth;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (
                    Url::new("user", "/api-docs/user.json"),
                    auth::app::users::ApiDoc::openapi(),
                ),
                (
                    Url::new("auth", "/api-docs/auth.json"),
                    auth::app::auth::ApiDoc::openapi(),
                ),
            ]))
            .service(
                web::scope("/auth")
                    .wrap(NormalizePath::default())
                    .configure(auth::app::auth::config),
            )
            .service(
                web::scope("/user")
                    .wrap(NormalizePath::default())
                    .configure(auth::app::users::config),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
