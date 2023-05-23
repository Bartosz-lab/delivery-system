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
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
                Url::new("auth", "/api-docs/auth.json"),
                auth::app::api::ApiDoc::openapi(),
            )]))
            .service(
                web::scope("/auth")
                    .wrap(NormalizePath::default())
                    .configure(auth::app::api::config),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
