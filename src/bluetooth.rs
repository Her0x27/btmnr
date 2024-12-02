use windows::Win32::Devices::Bluetooth::{
    BluetoothFindFirstDevice,
    BluetoothFindNextDevice,
    BluetoothFindDeviceClose,
    BLUETOOTH_DEVICE_INFO,
    BLUETOOTH_DEVICE_SEARCH_PARAMS,
    BLUETOOTH_SERVICE_ENABLE,
    BLUETOOTH_SERVICE_DISABLE
};
use windows::core::GUID;
const GUID_HANDSFREE_SERVICE: GUID = GUID::from_values(
    0x0000111E, 0x0000, 0x1000,
    [0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB]
);

use std::mem::zeroed;

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
            params.fReturnAuthenticated = true;
            params.fReturnConnected = true;
            params.fReturnRemembered = true;
            params.fIssueInquiry = true;
            params.cTimeoutMultiplier = 1;

            let mut device_info: BLUETOOTH_DEVICE_INFO = zeroed();
            device_info.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32;

            let device_handle = BluetoothFindFirstDevice(&params, &mut device_info)?;
            
            loop {
                if self.is_target_device(&device_info) {
                    // Подключаемся к устройству
                    let result = BluetoothAuthenticateDevice(
                        None,
                        None,
                        &mut device_info,
                        std::ptr::null(),
                        0
                    );
                    
                    if result.is_ok() {
                        BluetoothSetServiceState(
                            None,
                            &mut device_info,
                            &GUID_HANDSFREE_SERVICE,
                            BLUETOOTH_SERVICE_ENABLE
                        )?;
                    }
                    break;
                }

                if !BluetoothFindNextDevice(device_handle, &mut device_info).as_bool() {
                    break;
                }
            }

            BluetoothFindDeviceClose(device_handle);
            Ok(())
        }
    }

    pub async fn disconnect(&self) -> windows::core::Result<()> {
        unsafe {
            let mut params: BLUETOOTH_DEVICE_SEARCH_PARAMS = zeroed();
            params.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;
            params.fReturnConnected = true;

            let mut device_info: BLUETOOTH_DEVICE_INFO = zeroed();
            device_info.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32;

            let device_handle = BluetoothFindFirstDevice(&params, &mut device_info)?;

            loop {
                if self.is_target_device(&device_info) {
                    BluetoothSetServiceState(
                        None,
                        &mut device_info,
                        &GUID_HANDSFREE_SERVICE,
                        BLUETOOTH_SERVICE_DISABLE
                    )?;
                    break;
                }

                if !BluetoothFindNextDevice(device_handle, &mut device_info).as_bool() {
                    break;
                }
            }

            BluetoothFindDeviceClose(device_handle);
            Ok(())
        }
    }

    fn is_target_device(&self, device_info: &BLUETOOTH_DEVICE_INFO) -> bool {
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
