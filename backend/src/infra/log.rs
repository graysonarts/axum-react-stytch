use axum::http::{Request, Response};
use tower_http::trace::{OnRequest, OnResponse};
use tracing::Span;

#[derive(Default, Clone)]
pub struct LogRequest {}

impl<B> OnRequest<B> for LogRequest {
    fn on_request(&mut self, request: &Request<B>, _span: &Span) {
        tracing::info!("start: {} {}", request.method(), request.uri());
    }
}

impl<B> OnResponse<B> for LogRequest {
    fn on_response(self, response: &Response<B>, latency: std::time::Duration, _span: &Span) {
        tracing::info!("finished: {} {}", response.status(), latency.as_micros());
    }
}
