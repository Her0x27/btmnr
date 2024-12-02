use windows::Win32::Media::Audio::{
    IAudioSessionManager2, IAudioSessionEnumerator, IAudioSessionControl, 
    IAudioSessionControl2, IAudioMeterInformation, IMMDevice, 
    IMMDeviceEnumerator, MMDeviceEnumerator, eRender, eConsole,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};
use windows::core::{Interface, Result};
use windows::Win32::Foundation::BOOL;

pub struct AudioMonitor;

impl AudioMonitor {
    pub fn is_audio_playing() -> bool {
        unsafe {
            let enumerator: IMMDeviceEnumerator = CoCreateInstance(
                &MMDeviceEnumerator,
                None,
                CLSCTX_ALL
            ).unwrap();

            let device: IMMDevice = match enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
                Ok(d) => d,
                Err(_) => return false,
            };

            let session_manager: IAudioSessionManager2 = match device.Activate::<IAudioSessionManager2>(CLSCTX_ALL, None) {
                Ok(sm) => sm,
                Err(_) => return false,
            };

            let session_enum: IAudioSessionEnumerator = match session_manager.GetSessionEnumerator() {
                Ok(se) => se,
                Err(_) => return false,
            };

            let count = match session_enum.GetCount() {
                Ok(c) => c,
                Err(_) => return false,
            };

            for i in 0..count {
                if let Ok(session) = session_enum.GetSession(i) {
                    if let Ok(session2) = session.cast::<IAudioSessionControl2>() {
                        if let Ok(meter) = session.cast::<IAudioMeterInformation>() {
                            if let Ok(peak) = meter.GetPeakValue() {
                                if peak > 0.0 {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        }
    }
}
