use windows_service::{
    define_windows_service,
    service::ServiceState,
    service_dispatcher,
};
use log::{info, error};
use std::{
    ffi::OsString,
    sync::{Arc, atomic::{AtomicBool, Ordering}}, 
    time::Duration
};
use tokio;

mod audio;
mod bluetooth;
mod config;

use crate::audio::AudioMonitor;
use crate::bluetooth::BluetoothController;
use crate::config::Config;

struct BluetoothManager {
    config: Config,
    running: Arc<AtomicBool>,
}

impl BluetoothManager {
    fn new(config: Config) -> Self {
        Self {
            config,
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    async fn monitor_audio_activity(&self) {
        let mut last_activity = std::time::Instant::now();
        
        while self.running.load(Ordering::Relaxed) {
            if AudioMonitor::is_audio_playing() {
                last_activity = std::time::Instant::now();
                if self.config.auto_connect {
                    self.ensure_connected().await;
                }
            } else if last_activity.elapsed() > Duration::from_secs(self.config.inactivity_timeout) {
                self.disconnect_device().await;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    async fn ensure_connected(&self) {
        info!("Подключение к устройству");
        let controller = BluetoothController::new(self.config.device_address.clone());
        if let Err(e) = controller.connect().await {
            error!("Ошибка подключения: {:?}", e);
        }
    }

    async fn disconnect_device(&self) {
        info!("Отключение устройства");
        let controller = BluetoothController::new(self.config.device_address.clone());
        if let Err(e) = controller.disconnect().await {
            error!("Ошибка отключения: {:?}", e);
        }
    }
}

define_windows_service!(ffi_service_main, service_main);

fn service_main(_arguments: Vec<OsString>) {
    if let Err(_e) = simple_logging::log_to_file("bluetooth_manager.log", log::LevelFilter::Info) {
        return;
    }

    let config = Config::load().unwrap_or_default();
    let manager = BluetoothManager::new(config);
    
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        manager.monitor_audio_activity().await;
    });
}

fn main() -> Result<(), windows_service::Error> {
    service_dispatcher::start("BluetoothManager", ffi_service_main)?;
    Ok(())
}
