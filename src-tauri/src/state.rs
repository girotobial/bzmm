use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::settings::Settings;

#[derive(Debug)]
pub struct AppState {
    settings: RwLock<Settings>,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings: RwLock::new(settings),
        }
    }

    pub fn write_settings(&self) -> Result<RwLockWriteGuard<'_, Settings>, String> {
        self.settings.write().map_err(|e| e.to_string())
    }

    pub fn read_settings(&self) -> Result<RwLockReadGuard<'_, Settings>, String> {
        self.settings.read().map_err(|e| e.to_string())
    }

    pub fn settings_snapshot(&self) -> Result<Settings, String> {
        self.read_settings().map(|settings| settings.clone())
    }

    pub fn update_settings(&self, settings: impl AsRef<Settings>) -> Result<(), String> {
        let mut settings = settings.as_ref().clone();
        let mut app_settings = self.write_settings()?;
        std::mem::swap(&mut *app_settings, &mut settings);
        app_settings.save()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(Settings::default())
    }
}
