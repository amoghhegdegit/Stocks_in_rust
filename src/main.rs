use actix_web::{web, App, HttpServer, middleware};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod config;
mod data;
mod analysis;
mod ml;
mod portfolio;
mod education;
mod utils;

use crate::config::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Starting Stock Market Analyst Application");

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");
    
    info!("Configuration loaded successfully");
    info!("Server will run on {}:{}", settings.server.host, settings.server.port);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(settings.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .service(api::health_check)
            .service(api::stock_data)
            .service(api::technical_analysis)
            .service(api::predictions)
            .service(api::portfolio)
            .service(api::education)
    })
    .bind((settings.server.host.clone(), settings.server.port))?
    .run()
    .await
}