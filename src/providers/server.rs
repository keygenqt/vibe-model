//! Provider server.
//! Binds provider router and starts axum server.

use std::sync::Arc;

use anyhow::Result;
use axum::extract::Request;
use axum::middleware::from_fn;
use axum::response::Response;
use tower_http::cors::CorsLayer;
use tower_http::normalize_path::NormalizePathLayer;

use crate::pipeline::pipeline::Pipeline;
use crate::print_info;
use crate::providers::provider::Provider;

/// Provider server — runs axum with provider router.
pub struct ProviderServer {
    provider: Box<dyn Provider>,
    pipeline: Arc<Pipeline>,
    host: String,
    port: u16,
}

impl ProviderServer {
    /// Create new provider server.
    pub fn new(
        provider: Box<dyn Provider>,
        pipeline: Arc<Pipeline>,
        host: String,
        port: u16,
    ) -> Self {
        Self {
            provider,
            pipeline,
            host,
            port,
        }
    }

    /// Start the server.
    pub async fn run(&self) -> Result<()> {
        let router = self
            .provider
            .router(self.pipeline.clone())
            .layer(from_fn(log_requests));

        let addr = format!("{}:{}", self.host, self.port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;

        print_info!("{} server started on http://{}", self.provider.name(), addr);

        let app = router
            .layer(CorsLayer::permissive())
            .layer(NormalizePathLayer::trim_trailing_slash())
            .into_make_service();

        axum::serve(listener, app).await?;
        Ok(())
    }
}

/// Log HTTP requests.
async fn log_requests(request: Request, next: axum::middleware::Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    tracing::trace!("{} {} - Headers: {:?}", method, uri, request.headers());
    let response = next.run(request).await;
    tracing::info!("{} {} -> {}", method, uri, response.status());
    response
}
