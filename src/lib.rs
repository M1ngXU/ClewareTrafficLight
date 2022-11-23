use rusb::{Device, DeviceHandle, GlobalContext, UsbContext};

const VENDOR_ID: u16 = 0x0d50;
// "old" style (not switch)
const PRODUCT_ID: u16 = 0x0008;
const CTRL_ENDPOINT: u8 = 0x02;
const INTERFACE: u8 = 0x00;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum Color {
    Red   = 0x10,
    Yellow = 0x11,
    Green = 0x12,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum State {
    Off = 0x00,
    On  = 0x01,
}

pub struct InitializedGlobalDevice {
    handle: DeviceHandle<GlobalContext>,
    detached_kernel_driver: bool,
}
impl InitializedGlobalDevice {
    fn find_devices() -> rusb::Result<Vec<Device<GlobalContext>>> {
        Ok(GlobalContext::default()
            .devices()?
            .iter()
            .filter(|device| {
                if let Ok(desc) = device.device_descriptor() {
                    desc.vendor_id() == VENDOR_ID && desc.product_id() == PRODUCT_ID
                } else {
                    false
                }
            })
            .collect())
    }

    fn create_with_any() -> rusb::Result<Self> {
        Self::create(Self::find_devices()?.get(0).ok_or(rusb::Error::NotFound)?)
    }

    fn create(device: &Device<GlobalContext>) -> rusb::Result<Self> {
        let mut s = Self {
            handle: device.open()?,
            detached_kernel_driver: false,
        };
        if s.handle.kernel_driver_active(INTERFACE) == Ok(true) {
            s.detached_kernel_driver = true;
            s.handle.detach_kernel_driver(INTERFACE)?;
        }
        s.handle.claim_interface(INTERFACE)?;
        Ok(s)
    }

    pub fn set_light(color: Color, state: State) -> rusb::Result<()> {
        Self::create_with_any()?
            .handle
            .write_interrupt(
                CTRL_ENDPOINT,
                &[0x00, color as u8, state as u8],
                std::time::Duration::from_secs(1000),
            )
            .map(|_| {})
    }
}
impl Drop for InitializedGlobalDevice {
    fn drop(&mut self) {
        self.handle
            .release_interface(INTERFACE)
            .expect("Failed to release interface.");
        if self.detached_kernel_driver {
            self.handle
                .attach_kernel_driver(INTERFACE)
                .expect("Failed to re-attach kernel interface.");
        }
    }
}
