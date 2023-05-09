use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tokio::sync::RwLock;

use crate::{Build, WebHook};

type SharedState = Arc<RwLock<ServerState>>;

#[derive(Debug, Default)]
pub struct ServerState {
    builds: Vec<Build>,
}

pub(crate) fn server_router() -> Router {
    Router::new()
        .route("/status", get(list_jobs))
        .route("/jobs/:id", post(take_job).put(update_job))
        .with_state(SharedState::default())
}

/// Entry point for the webhook
async fn webhook(Json(data): Json<WebHook>, State(state): State<SharedState>) -> impl IntoResponse {
    todo!()
}

async fn create_job(Json(data): Json<WebHook>, State(st): State<SharedState>) -> impl IntoResponse {
    todo!()
}

/// List pending jobs
async fn list_jobs(State(st): State<SharedState>) -> impl IntoResponse {
    todo!()
}

/// take a pending job
async fn take_job(State(st): State<SharedState>, Path(id): Path<String>) -> impl IntoResponse {
    todo!()
}

///update the job status
async fn update_job(State(st): State<SharedState>, Path(id): Path<String>) -> impl IntoResponse {
    todo!()
}
