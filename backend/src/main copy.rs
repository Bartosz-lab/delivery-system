use actix_web::{
    get,
    middleware::{Logger, NormalizePath},
    web, App, HttpResponse, HttpServer, Responder,
};
use std::error::Error;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

mod auth;

#[utoipa::path(
    context_path = "/app",
    responses(
        (status = 200, description = "Hello from api 2", body = String)
    )
)]
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    #[derive(OpenApi)]
    #[openapi(paths(hello))]
    pub struct ApiDoc;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::default())
            .service(web::scope("/auth").configure(auth::app::api::config))
            .service(web::scope("/app").service(hello))
            // .service(hello)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()), // .url("/api-docs/openapi2.json", auth::app::api::ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
