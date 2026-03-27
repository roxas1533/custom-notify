use crate::notification::{self, NotificationData, NotificationManagerState};
use crate::settings::{Settings, SettingsState};
use tauri::webview::WebviewWindow;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn get_settings(state: State<'_, SettingsState>) -> Result<Settings, String> {
    let settings = state.0.lock().await;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn save_settings(
    state: State<'_, SettingsState>,
    settings: Settings,
) -> Result<(), String> {
    settings.save();
    let mut current = state.0.lock().await;
    *current = settings;
    Ok(())
}

#[tauri::command]
pub async fn close_notification(app: AppHandle, id: String) -> Result<(), String> {
    let manager = app.state::<NotificationManagerState>();
    let mut state = manager.0.lock().await;
    state.remove(&id);
    notification::close_notification_window(&app, &id);
    Ok(())
}

#[tauri::command]
pub async fn notification_ready(
    window: WebviewWindow,
) -> Result<Option<NotificationData>, String> {
    let app = window.app_handle().clone();
    let label = window.label().to_string();
    Ok(notification::get_data(&app, &label).await)
}

#[tauri::command]
pub async fn frontend_log(message: String) -> Result<(), String> {
    tracing::info!("[frontend] {}", message);
    Ok(())
}
