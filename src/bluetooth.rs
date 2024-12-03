use windows::Win32::Devices::Bluetooth::{
    BluetoothFindFirstDevice,
    BluetoothFindNextDevice,
    BluetoothFindDeviceClose,
    BLUETOOTH_DEVICE_INFO,
    BLUETOOTH_DEVICE_SEARCH_PARAMS,
    BluetoothAuthenticateDevice,
    BluetoothSetServiceState,
    BLUETOOTH_SERVICE_ENABLE,
    BLUETOOTH_SERVICE_DISABLE,
};
use windows::core::{GUID, Result};
use std::mem::zeroed;

impl BluetoothController {
    pub async fn connect(&self) -> Result<()> {
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

            let device_handle = BluetoothFindFirstDevice(&params, &mut device_info);
            
            if self.is_target_device(&device_info) {
                BluetoothAuthenticateDevice(
                    None,
                    None,
                    &mut device_info,
                    None,
                );
                
                BluetoothSetServiceState(
                    None,
                    &mut device_info,
                    &GUID_HANDSFREE_SERVICE,
                    BLUETOOTH_SERVICE_ENABLE,
                );
            }

            BluetoothFindDeviceClose(device_handle);
            Ok(())
        }
    }

    pub async fn disconnect(&self) -> Result<()> {
        unsafe {
            let mut params: BLUETOOTH_DEVICE_SEARCH_PARAMS = zeroed();
            params.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;
            params.fReturnConnected = BOOL::from(true);

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
