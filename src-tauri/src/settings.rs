use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Settings {
    pub port: u16,
    pub notification_position: NotificationPosition,
    pub notification_duration_ms: u64,
    pub max_visible_notifications: usize,
    pub notification_width: u32,
    pub notification_height: u32,
    pub notification_gap: u32,
    pub animation_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            port: 19090,
            notification_position: NotificationPosition::BottomRight,
            notification_duration_ms: 5000,
            max_visible_notifications: 5,
            notification_width: 360,
            notification_height: 120,
            notification_gap: 8,
            animation_duration_ms: 300,
        }
    }
}

pub struct SettingsState(pub Arc<Mutex<Settings>>);

impl Settings {
    fn config_path() -> PathBuf {
        let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let dir = base.join("custom-notify");
        std::fs::create_dir_all(&dir).ok();
        dir.join("settings.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        match std::fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => {
                let default = Self::default();
                default.save();
                default
            }
        }
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Ok(toml_str) = toml::to_string_pretty(self) {
            std::fs::write(path, toml_str).ok();
        }
    }
}
