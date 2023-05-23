use std::error::Error;

use actix_web::{middleware::Logger, web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    #[derive(OpenApi)]
    #[openapi(paths(api1::hello1))]
    struct ApiDoc1;

    #[derive(OpenApi)]
    #[openapi(paths(api2::hello2))]
    struct ApiDoc2;

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(api1::hello1)
                    .service(api2::hello2),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi1.json", ApiDoc1::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

mod api1 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 1", body = String)
        )
    )]
    #[get("/api1/hello")]
    pub(super) async fn hello1() -> String {
        "hello from api 1".to_string()
    }
}

mod api2 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 2", body = String)
        )
    )]
    #[get("/api2/hello")]
    pub(super) async fn hello2() -> String {
        "hello from api 2".to_string()
    }
}
