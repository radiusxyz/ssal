use ssal_core::axum::response::IntoResponse;

pub async fn register() -> impl IntoResponse {
    "register_rollup"
}

pub async fn close_block() -> impl IntoResponse {
    "close_block"
}
