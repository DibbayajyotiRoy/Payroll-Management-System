use actix_files as fs;
use actix_web::{web, App, HttpServer, get};
use basics::services::payroll_service::PayrollService;
use basics::web::routes::configure_routes;
use std::sync::Arc;
use tokio::sync::Mutex;
use actix_cors::Cors;

#[get("/docs")]
async fn redirect_to_docs() -> impl actix_web::Responder {
    actix_web::HttpResponse::Found()
        .append_header((actix_web::http::header::LOCATION, "/docs/"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting server at http://127.0.0.1:8080");

    // Create an Arc<Mutex<>> for the PayrollService
    let payroll_service = Arc::new(Mutex::new(PayrollService::new()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(payroll_service.clone()))
            .configure(configure_routes)
            .service(redirect_to_docs)
            .service(fs::Files::new("/docs", "./api-docs").index_file("scalar.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
