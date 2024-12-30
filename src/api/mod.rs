use anyhow::{Ok, Result};
use axum::{response::Json, routing};
use tower_http::trace::TraceLayer;

pub mod config;
pub use config::Config;
mod ui;

#[derive(Clone)]
pub struct Api {}

impl Api {
    pub fn new() -> Self {
        Api {}
    }

    pub fn get_router(&self) -> Result<axum::Router> {
        let router = axum::Router::new()
            .route("/healthz", routing::get(healthz_handler))
            .nest("/", ui::get_router()?)
            .layer(TraceLayer::new_for_http());
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
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn healthz() {
        let api = Api::new();
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
        let api = Api::new();
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
