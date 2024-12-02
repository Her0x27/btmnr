use windows::Win32::Media::Audio::{
    IAudioSessionManager2, IAudioSessionEnumerator, 
    IAudioSessionControl2, IMMDevice, 
    IMMDeviceEnumerator, MMDeviceEnumerator, 
    eRender, eConsole,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};
use windows::Win32::Foundation::BOOL;
use windows::core::{Interface, Result};

impl IMMDevice {
    unsafe fn Activate<T: Interface>(&self, context: u32) -> Result<T> {
        // Implementation for Activate
        todo!()
    }
}

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
