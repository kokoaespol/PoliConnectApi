#![allow(non_snake_case)]
mod core;
mod promedios;

use axum::Router;
use dotenvy::dotenv;
use std::env;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::promedios::routes::WithPromediosRoutes;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "PoliConnectApi=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if dotenv().is_err() {
        tracing::warn!("No .env file was loaded");
    }

    let port = env::var("PORT")
        .expect("PORT is not set")
        .parse()
        .expect("Failed to parse PORT");

    let app = Router::new()
        .with_promedios()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap();

    tracing::info!("Starting server on port {}", port);
    axum::serve(listener, app).await.unwrap();
}
