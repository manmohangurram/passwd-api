use actix_web::{
    error,
    middleware::{Logger, NormalizePath, TrailingSlash},
    web::{self},
    App, HttpResponse, HttpServer,
};
use env_logger::Target;
use log::LevelFilter;
use logos::LogosService;

#[actix_web::main]
async fn main() -> Result<(), String> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .target(Target::Stdout)
        .init();

    let json_cfg = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            error::InternalError::from_response(
                "",
                HttpResponse::BadRequest().json(format!(r#"{{ message: "{}" }}"#, err)),
            )
            .into()
        });

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(logger)
            .app_data(json_cfg.clone())
            .configure(LogosService::init)
    })
    .bind(("0.0.0.0", 8080))
    .map_err(|e| e.to_string())?
    .workers(2)
    .run()
    .await
    .map_err(|e| e.to_string())
}
