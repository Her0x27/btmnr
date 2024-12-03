use windows::Win32::Devices::Bluetooth::{
    BluetoothFindFirstDevice,
    BluetoothFindDeviceClose,
    BLUETOOTH_DEVICE_INFO,
    BLUETOOTH_DEVICE_SEARCH_PARAMS,
    BluetoothAuthenticateDevice,
    BluetoothSetServiceState,
};

use windows::Win32::Foundation::BOOL;
use windows::core::GUID;
use std::mem::zeroed;

const HANDSFREE_SERVICE_GUID: GUID = GUID::from_values(
    0x0000111E, 0x0000, 0x1000, 
    [0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB]
);

pub struct BluetoothController {
    device_address: String,
}

impl BluetoothController {
    pub fn new(device_address: String) -> Self {
        Self { device_address }
    }

    pub async fn connect(&self) -> windows::core::Result<()> {
        unsafe {
            let mut params: BLUETOOTH_DEVICE_SEARCH_PARAMS = zeroed();
            params.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;
            params.fReturnAuthenticated = BOOL::from(true);
            params.fReturnConnected = BOOL::from(true);
            params.fReturnRemembered = BOOL::from(true);
            params.fIssueInquiry = BOOL::from(true);
            params.cTimeoutMultiplier = 1;

            let mut device_info: BLUETOOTH_DEVICE_INFO = zeroed();
            device_info.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32;

            let device_handle = BluetoothFindFirstDevice(&params, &mut device_info)?;
            
            if self.is_target_device(&device_info) {
                BluetoothAuthenticateDevice(
                    None,
                    None,
                    &mut device_info
                );
                
                BluetoothSetServiceState(
                    None,
                    &mut device_info,
                    &HANDSFREE_SERVICE_GUID,
                    1
                );
            }

            BluetoothFindDeviceClose(device_handle);
            Ok(())
        }
    }

    pub async fn disconnect(&self) -> windows::core::Result<()> {
        unsafe {
            let mut params: BLUETOOTH_DEVICE_SEARCH_PARAMS = zeroed();
            params.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;
            params.fReturnConnected = BOOL::from(true);

            let mut device_info: BLUETOOTH_DEVICE_INFO = zeroed();
            device_info.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32;

            let device_handle = BluetoothFindFirstDevice(&params, &mut device_info)?;

            if self.is_target_device(&device_info) {
                BluetoothSetServiceState(
                    None,
                    &mut device_info,
                    &HANDSFREE_SERVICE_GUID,
                    0
                );
            }

            BluetoothFindDeviceClose(device_handle);
            Ok(())
        }
    }

    fn is_target_device(&self, device_info: &BLUETOOTH_DEVICE_INFO) -> bool {
        unsafe {
            let address = format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                device_info.Address.Anonymous.rgBytes[5],
                device_info.Address.Anonymous.rgBytes[4],
                device_info.Address.Anonymous.rgBytes[3],
                device_info.Address.Anonymous.rgBytes[2],
                device_info.Address.Anonymous.rgBytes[1],
                device_info.Address.Anonymous.rgBytes[0],
            );
            address == self.device_address
        }
    }
}
