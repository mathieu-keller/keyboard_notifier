use actix_web::{App, HttpServer, middleware, web};

use github_resource::{send_check_run, send_check_suit};

mod github_resource;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(16256))
            .service(web::resource("/check-run").route(web::post().to(send_check_run)))
            .service(web::resource("/check-suites").route(web::post().to(send_check_suit)))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
