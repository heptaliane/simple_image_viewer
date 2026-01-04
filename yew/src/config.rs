use shared::config::ViewerConfig;
use shared::event::TauriEvent;
use std::sync::{Arc, Mutex};

use crate::tauri::{emit_without_args, listen};

pub struct ConfigHandler {
    config: Arc<Mutex<ViewerConfig>>,
    listeners: Arc<Mutex<Vec<Box<dyn Fn(&ViewerConfig)>>>>,
}

impl ConfigHandler {
    pub fn new() -> Self {
        Self {
            config: Arc::new(Mutex::new(ViewerConfig::default())),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn Fn(&ViewerConfig)>) {
        match self.listeners.lock() {
            Ok(mut listeners) => listeners.push(listener),
            Err(e) => log::error!("Failed to register config listener: {:?}", e),
        }
    }

    pub fn listen(&self) {
        let config = self.config.clone();
        let listeners = self.listeners.clone();
        listen(
            TauriEvent::ReceiveConfig.as_ref(),
            move |new_config: ViewerConfig| match config.lock() {
                Ok(mut config) => {
                    *config = new_config;

                    // Broadcast config
                    match listeners.lock() {
                        Ok(listeners) => listeners.iter().for_each(|listener| listener(&config)),
                        Err(e) => log::error!("Failed to access listeners: {:?}", e),
                    }
                }
                Err(e) => log::error!("Failed to update config: {:?}", e),
            },
        );
    }

    pub fn sync(&self) {
        emit_without_args(TauriEvent::RequestConfig.as_ref());
    }
}
