use crate::store::KvStore;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub kv_store: Arc<KvStore>,
}

#[derive(Deserialize)]
pub struct SetRequest {
    pub value: String,
}

#[derive(Serialize)]
pub struct GetResponse {
    pub value: String,
}

pub async fn get_handler(
    Path(key): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.kv_store.get(&key) {
        Some(value) => {
            let response = GetResponse { value };
            Ok((StatusCode::OK, Json(response)))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn put_handler(
    Path(key): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<SetRequest>,
) -> impl IntoResponse {
    state.kv_store.set(key, payload.value);
    StatusCode::CREATED
}
