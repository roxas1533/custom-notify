mod commands;
mod http_server;
mod notification;
mod settings;
mod tray;

use std::sync::Arc;
use tauri::{Manager, RunEvent};
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Log to file since Windows GUI apps have no console
    let log_path = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("custom-notify")
        .join("debug.log");
    std::fs::create_dir_all(log_path.parent().unwrap()).ok();
    let log_file = std::fs::File::create(&log_path).expect("Failed to create log file");
    tracing_subscriber::fmt()
        .with_writer(std::sync::Mutex::new(log_file))
        .with_ansi(false)
        .init();

    let settings = settings::Settings::load();
    let notification_state = Arc::new(Mutex::new(notification::NotificationState::new()));

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(settings::SettingsState(Arc::new(Mutex::new(settings))))
        .manage(notification::NotificationManagerState(notification_state))
        .setup(|app| {
            tray::create_tray(app.handle())?;

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                http_server::start_server(app_handle.clone()).await;
            });

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // Wait briefly for HTTP server to be ready
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                let port = app_handle
                    .state::<settings::SettingsState>()
                    .0
                    .lock()
                    .await
                    .port;
                notification::show_notification(
                    &app_handle,
                    http_server::NotifyRequest {
                        title: "Custom Notify".to_string(),
                        body: format!("起動しました (ポート: {})", port),
                        icon_url: None,
                        duration_ms: Some(3000),
                        style: Some("success".to_string()),
                    },
                )
                .await
                .ok();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_settings,
            commands::save_settings,
            commands::close_notification,
            commands::notification_ready,
            commands::frontend_log,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_app_handle, event| {
        // Prevent app exit when all windows are closed (tray-only app)
        // code 0 = explicit exit from tray menu; only block window-close exits
        if let RunEvent::ExitRequested { code, api, .. } = &event {
            if code.is_none() {
                api.prevent_exit();
            }
        }
    });
}
