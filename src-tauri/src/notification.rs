use crate::http_server::NotifyRequest;
use crate::settings::{NotificationPosition, SettingsState};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct NotificationData {
    pub id: String,
    pub title: String,
    pub body: String,
    pub icon_url: Option<String>,
    pub duration_ms: u64,
    pub animation_duration_ms: u64,
    pub style: String,
}

#[derive(Debug)]
struct NotificationSlot {
    id: String,
    slot_index: usize,
}

pub struct NotificationState {
    active: Vec<NotificationSlot>,
    pending_data: HashMap<String, NotificationData>,
}

impl NotificationState {
    pub fn new() -> Self {
        Self {
            active: Vec::new(),
            pending_data: HashMap::new(),
        }
    }

    fn next_slot(&self, max: usize) -> Option<usize> {
        for i in 0..max {
            if !self.active.iter().any(|s| s.slot_index == i) {
                return Some(i);
            }
        }
        None
    }

    pub fn remove(&mut self, id: &str) {
        self.active.retain(|s| s.id != id);
    }
}

pub struct NotificationManagerState(pub Arc<Mutex<NotificationState>>);

/// Called by frontend to get notification data for this window.
pub async fn get_data(app: &AppHandle, window_label: &str) -> Option<NotificationData> {
    let manager_state = app.state::<NotificationManagerState>();
    let mut state = manager_state.0.lock().await;
    tracing::info!("get_data for: {}, keys: {:?}", window_label, state.pending_data.keys().collect::<Vec<_>>());
    state.pending_data.remove(window_label)
}

pub async fn show_notification(
    app: &AppHandle,
    req: NotifyRequest,
) -> Result<String, String> {
    let settings_state = app.state::<SettingsState>();
    let settings = settings_state.0.lock().await.clone();

    let manager_state = app.state::<NotificationManagerState>();
    let mut state = manager_state.0.lock().await;

    let slot_index = state
        .next_slot(settings.max_visible_notifications)
        .ok_or_else(|| "Max notifications reached".to_string())?;

    let id = Uuid::new_v4().to_string();
    let window_label = format!("notification_{}", id);
    let duration = req.duration_ms.unwrap_or(settings.notification_duration_ms);

    let (x, y) = calculate_position(
        app,
        &settings.notification_position,
        slot_index,
        settings.notification_width,
        settings.notification_height,
        settings.notification_gap,
    );

    let notification_data = NotificationData {
        id: id.clone(),
        title: req.title,
        body: req.body,
        icon_url: req.icon_url,
        duration_ms: duration,
        animation_duration_ms: settings.animation_duration_ms,
        style: req.style.unwrap_or_else(|| "info".to_string()),
    };

    // Insert data BEFORE creating window so it's available when JS runs
    state.pending_data.insert(window_label.clone(), notification_data);
    state.active.push(NotificationSlot {
        id: id.clone(),
        slot_index,
    });

    // Drop lock before build
    drop(state);

    tracing::info!("Creating window: {}", &window_label);

    tauri::webview::WebviewWindowBuilder::new(
        app,
        &window_label,
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("Notification")
    .inner_size(
        settings.notification_width as f64,
        settings.notification_height as f64,
    )
    .position(x, y)
    .decorations(true)
    .transparent(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .focused(false)
    .build()
    .map_err(|e| e.to_string())?;

    // Auto-dismiss lifecycle
    if duration > 0 {
        let app_clone = app.clone();
        let id_clone = id.clone();
        let label_clone = window_label.clone();
        let anim = settings.animation_duration_ms;

        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(duration + anim)).await;

            if let Some(win) = app_clone.get_webview_window(&label_clone) {
                win.eval("window.__dismiss && window.__dismiss()").ok();
            }

            tokio::time::sleep(std::time::Duration::from_millis(anim + 100)).await;

            let state = app_clone.state::<NotificationManagerState>();
            let mut s = state.0.lock().await;
            s.remove(&id_clone);

            if let Some(win) = app_clone.get_webview_window(&label_clone) {
                win.close().ok();
            }
        });
    }

    Ok(id)
}

pub fn close_notification_window(app: &AppHandle, id: &str) {
    let label = format!("notification_{}", id);
    if let Some(win) = app.get_webview_window(&label) {
        win.close().ok();
    }
}

fn calculate_position(
    app: &AppHandle,
    position: &NotificationPosition,
    slot_index: usize,
    width: u32,
    height: u32,
    gap: u32,
) -> (f64, f64) {
    let (screen_width, screen_height) = app
        .primary_monitor()
        .ok()
        .flatten()
        .map(|m| {
            let size = m.size();
            let scale = m.scale_factor();
            (
                (size.width as f64 / scale) as u32,
                (size.height as f64 / scale) as u32,
            )
        })
        .unwrap_or((1920, 1080));

    let margin: u32 = 16;
    let offset = slot_index as u32 * (height + gap);

    match position {
        NotificationPosition::TopRight => (
            (screen_width - width - margin) as f64,
            (margin + offset) as f64,
        ),
        NotificationPosition::TopLeft => (margin as f64, (margin + offset) as f64),
        NotificationPosition::BottomRight => (
            (screen_width - width - margin) as f64,
            (screen_height - margin - height - offset) as f64,
        ),
        NotificationPosition::BottomLeft => (
            margin as f64,
            (screen_height - margin - height - offset) as f64,
        ),
    }
}
