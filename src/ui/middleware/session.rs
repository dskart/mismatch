use crate::app;
use axum::extract::Request;
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::RngCore;
use std::{
    sync::Arc,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::Instrument;

#[derive(Clone)]
pub struct SessionLayer {
    pub app: Arc<app::App>,
}

impl<S> Layer<S> for SessionLayer {
    type Service = SessionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionService {
            inner,
            app: self.app.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SessionService<S> {
    inner: S,
    app: Arc<app::App>,
}

impl<S, B> Service<Request<B>> for SessionService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = tracing::instrument::Instrumented<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let mut random_bytes = [0u8; 15];
        rand::thread_rng().fill_bytes(&mut random_bytes);
        let session_id = STANDARD.encode(random_bytes);
        let span = tracing::info_span!("session", session_id=%session_id);
        let session = self.app.new_session();

        req.extensions_mut().insert(Arc::new(session));
        self.inner.call(req).instrument(span)
    }
}
