use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerShutdown},
    service_dispatcher,
};

use windows::Win32::Devices::Bluetooth;
use serde::{Deserialize, Serialize};
use log::{info, error, warn};
use std::{
    sync::{
        Arc, 
        atomic::{AtomicBool, Ordering}
    }, 
    time::Duration
};
use tokio;

mod audio;
mod bluetooth;
mod config;

use crate::audio::AudioMonitor;
use crate::bluetooth::BluetoothController;
use crate::config::Config;

pub struct BluetoothManager {
    config: Config,
    running: Arc<AtomicBool>,
}

impl BluetoothManager {
    fn is_audio_playing(&self) -> bool {
        AudioMonitor::is_audio_playing()
    }

    async fn ensure_connected(&self) {
        let controller = BluetoothController::new(self.config.device_address.clone());
        if let Err(e) = controller.connect().await {
            error!("Ошибка подключения: {:?}", e);
        }
    }

    async fn disconnect_device(&self) {
        let controller = BluetoothController::new(self.config.device_address.clone());
        if let Err(e) = controller.disconnect().await {
            error!("Ошибка отключения: {:?}", e);
        }
    }
}
