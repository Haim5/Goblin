use axum::{routing::{get, post}, Json, Router};
use common::types::{
    LayoutRequest, LayoutResult, ValidationResponse,
};
use graph::builder::build;
use layout::pipeline::run;
use crate::dto::{ValidateRequest, HealthResponse};

pub fn router() -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/layout", post(layout))
        .route("/api/validate", post(validate))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

async fn layout(Json(req): Json<LayoutRequest>) -> Json<LayoutResult> {
    Json(run(req))
}

async fn validate(Json(req): Json<ValidateRequest>) -> Json<ValidationResponse> {
    let (_adj, errors) = build(&req.network);
    Json(ValidationResponse {
        valid: errors.is_empty(),
        errors,
    })
}
