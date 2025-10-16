use axum::{
    extract::Request,            // ✅ correct Request alias (Body already bound)
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use lazy_static::lazy_static;
use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounterVec, Opts, Registry, TextEncoder,
};
use std::time::Instant;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    static ref HTTP_REQUESTS_TOTAL: IntCounterVec = {
        let v = IntCounterVec::new(
            Opts::new("http_requests_total", "Total HTTP requests"),
            &["method", "path", "status"]
        ).unwrap();
        REGISTRY.register(Box::new(v.clone())).unwrap();
        v
    };

    static ref HTTP_REQUEST_DURATION_SECONDS: HistogramVec = {
        let v = HistogramVec::new(
            HistogramOpts::new(
                "http_request_duration_seconds",
                "HTTP request latencies in seconds",
            ).buckets(vec![0.005,0.01,0.025,0.05,0.1,0.25,0.5,1.0,2.5,5.0]),
            &["method", "path"]
        ).unwrap();
        REGISTRY.register(Box::new(v.clone())).unwrap();
        v
    };
}

// GET /metrics
pub async fn metrics_handler() -> Response {
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    TextEncoder::new().encode(&metric_families, &mut buffer).unwrap();

    let body = String::from_utf8(buffer).unwrap_or_default();
    (StatusCode::OK, body).into_response()
}

// Middleware for recording metrics
pub async fn track_metrics(req: Request, next: Next) -> Response { // ✅ correct signature
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let start = Instant::now();
    let res = next.run(req).await;
    let status = res.status().as_u16().to_string();
    let elapsed = start.elapsed().as_secs_f64();

    HTTP_REQUESTS_TOTAL
        .with_label_values(&[method.as_str(), &path, &status])
        .inc();

    HTTP_REQUEST_DURATION_SECONDS
        .with_label_values(&[method.as_str(), &path])
        .observe(elapsed);

    res
}
