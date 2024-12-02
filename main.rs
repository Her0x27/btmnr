mod audio;
mod bluetooth;

use audio::AudioMonitor;
use bluetooth::BluetoothController;

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
