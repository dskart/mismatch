use crate::app;
use anyhow::{Ok, Result};
use axum::response::Response;
use axum::{body::Body, http::Request, response::Json, routing};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::RngCore;
use std::{sync::Arc, time::Duration};
use tower_http::trace::TraceLayer;
use tracing::Span;

pub mod config;
pub use config::Config;
mod error;
mod ui;

#[derive(Debug)]
pub struct Api {
    app: Arc<app::App>,
}

struct ApiState {
    app: Arc<app::App>,
}

impl Api {
    pub fn new(_cfg: Config, app: Arc<app::App>) -> Self {
        Api { app: app.clone() }
    }

    pub fn get_router(&self) -> Result<axum::Router> {
        let api_state = Arc::new(ApiState {
            app: self.app.clone(),
        });

        let router = axum::Router::new()
            .route("/healthz", routing::get(healthz_handler))
            .nest("/", ui::get_router(api_state)?)
            .layer(                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<Body>| {
                            let mut random_bytes = [0u8; 15];
                            rand::thread_rng().fill_bytes(&mut random_bytes);
                            let request_id = STANDARD.encode(random_bytes);
                            tracing::info_span!("request", method=%request.method(), uri=%request.uri(), request_id=%request_id)
                        })
                        .on_response(|response: &Response, latency: Duration, _span: &Span| {
                            let status = response.status();
                            tracing::info!(?status, ?latency)
                        }));
        Ok(router)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
struct HealthzResponse {
    status: String,
}

async fn healthz_handler() -> Json<HealthzResponse> {
    Json(HealthzResponse {
        status: "ok".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use crate::app;

    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn healthz() {
        let app = Arc::new(app::App::new(app::Config::default()).unwrap());
        let api = Api::new(Config::default(), app);
        let router = api.get_router().expect("failed to get router");

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/healthz")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: HealthzResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            HealthzResponse {
                status: "ok".to_string()
            }
        );
    }

    #[tokio::test]
    async fn not_found() {
        let app = Arc::new(app::App::new(app::Config::default()).unwrap());
        let api = Api::new(Config::default(), app);
        let router = api.get_router().unwrap();

        let response = router
            .oneshot(
                Request::builder()
                    .uri("/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.is_empty());
    }
}
