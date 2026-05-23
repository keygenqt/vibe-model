//! HTTP server launcher.
//! Binds Axum router and starts serving requests.

use std::sync::Arc;

use axum::Router;
use axum::ServiceExt;
use axum::extract::Request;
use axum::middleware::from_fn;
use axum::response::Response;
use axum::routing::get;
use axum::routing::post;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::normalize_path::NormalizePathLayer;

use crate::configs::app::AppConfig;
use crate::model::handlers;
use crate::model::state::AppState;

/// Start vibe-model HTTP server.
pub async fn run(host: String, port: u16) -> anyhow::Result<()> {
    let config = AppConfig::load()?;
    let state = AppState {
        config: Arc::new(config),
    };

    let router = Router::new()
        .route("/v1/chat/completions", post(handlers::chat::post))
        .route("/v1/models", get(handlers::models::get))
        .with_state(state)
        .layer(from_fn(log_requests));

    let route = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&route).await?;

    tracing::info!("Server started on http://{}", listener.local_addr()?);

    let app = ServiceBuilder::new()
        .layer(CorsLayer::permissive())
        .layer(NormalizePathLayer::trim_trailing_slash())
        .service(router);

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}

/// Log HTTP requests.
async fn log_requests(request: Request, next: axum::middleware::Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let response = next.run(request).await;
    tracing::info!("{} {} -> {}", method, uri, response.status());
    response
}
