use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tower_http::cors::CorsLayer;

use crate::notification;
use crate::settings::SettingsState;

#[derive(Debug, Clone, Deserialize)]
pub struct NotifyRequest {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub icon_url: Option<String>,
    #[serde(default)]
    pub duration_ms: Option<u64>,
    #[serde(default)]
    pub style: Option<String>,
}

#[derive(Serialize)]
struct NotifyResponse {
    ok: bool,
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Clone)]
struct ServerState {
    app_handle: AppHandle,
}

pub async fn start_server(app_handle: AppHandle) {
    let port = {
        let state = app_handle.state::<SettingsState>();
        let settings = state.0.lock().await;
        settings.port
    };

    let state = ServerState { app_handle };

    let app = Router::new()
        .route("/notify", post(handle_notify))
        .route("/health", axum::routing::get(|| async { "OK" }))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("127.0.0.1:{}", port);
    tracing::info!("Notification server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind HTTP server");

    axum::serve(listener, app).await.ok();
}

async fn handle_notify(
    State(state): State<ServerState>,
    Json(payload): Json<NotifyRequest>,
) -> Result<Json<NotifyResponse>, StatusCode> {
    match notification::show_notification(&state.app_handle, payload).await {
        Ok(id) => Ok(Json(NotifyResponse {
            ok: true,
            id: Some(id),
            error: None,
        })),
        Err(e) => Ok(Json(NotifyResponse {
            ok: false,
            id: None,
            error: Some(e),
        })),
    }
}
