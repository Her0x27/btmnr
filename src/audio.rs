use windows::Win32::Media::Audio::{
    IAudioSessionManager2, IAudioSessionEnumerator, IAudioSessionControl, IAudioSessionControl2,
    IAudioMeterInformation, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator,
    eRender, eConsole,
};
use windows::core::Interface;

pub struct AudioMonitor;

impl AudioMonitor {
    pub fn is_audio_playing() -> bool {
        unsafe {
            let enumerator: IMMDeviceEnumerator = CoCreateInstance(
                &MMDeviceEnumerator,
                None,
                CLSCTX_ALL
            ).unwrap();

            let device: IMMDevice = enumerator
                .GetDefaultAudioEndpoint(eRender, eConsole)
                .unwrap();

            let session_manager: IAudioSessionManager2 = device
                .Activate(CLSCTX_ALL, None)
                .unwrap();

            let session_enum: IAudioSessionEnumerator = session_manager
                .GetSessionEnumerator()
                .unwrap();

            let count = session_enum.GetCount().unwrap();

            for i in 0..count {
                if let Ok(session) = session_enum.GetSession(i) {
                    let session2: IAudioSessionControl2 = session.cast().unwrap();
                    let meter: IAudioMeterInformation = session.cast().unwrap();
                    
                    if !session2.GetSessionInstanceIdentifier().unwrap().is_empty() 
                        && meter.GetPeakValue().unwrap() > 0.0 {
                        return true;
                    }
                }
            }
            false
        }
    }
}
