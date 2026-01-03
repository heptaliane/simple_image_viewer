use serde_json;
use shared::config::ViewerConfig;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;

fn read_config(path: &String) -> Result<ViewerConfig, String> {
    match File::open(path) {
        Ok(f) => match serde_json::from_reader(f) {
            Ok(config) => Ok(config),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
fn write_config(path: &String, config: &ViewerConfig) -> Result<(), String> {
    match File::create(Path::new(path)) {
        Ok(f) => match serde_json::to_writer_pretty(f, config) {
            Err(e) => Err(e.to_string()),
            _ => Ok(()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub struct ConfigManager {
    config: Mutex<ViewerConfig>,
    path: String,
}

impl ConfigManager {
    pub fn new(path: String) -> Self {
        match read_config(&path) {
            Ok(config) => Self {
                config: Mutex::new(config),
                path,
            },
            Err(e) => {
                println!("[INFO] (config) Failed to load config: {:?}", e);

                let config = ViewerConfig::default();
                write_config(&path, &config).expect("Failed to create config file");

                Self {
                    config: Mutex::new(config),
                    path,
                }
            }
        }
    }

    pub fn get_config(&self) -> Result<ViewerConfig, String> {
        match self.config.lock() {
            Ok(config) => Ok(config.clone()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn update_config(&self, new_config: ViewerConfig) -> Result<(), String> {
        match self.config.lock() {
            Ok(mut config) => {
                *config = new_config;
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
