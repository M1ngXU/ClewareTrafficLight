use rusb::{Device, DeviceHandle, GlobalContext, UsbContext};

const VENDOR_ID: u16 = 0x0d50;
// "old" style (not switch)
const PRODUCT_ID: u16 = 0x0008;
const CTRL_ENDPOINT: u8 = 0x02;
const INTERFACE: u8 = 0x00;

#[repr(u8)]
pub enum Color {
    Red   = 0x10,
    Yellow = 0x11,
    Green = 0x12,
}

#[repr(u8)]
pub enum State {
    Off = 0x00,
    On  = 0x01,
}

pub type InitializedGlobalDevice = InitializedDevice<GlobalContext>;

pub struct InitializedDevice<C: UsbContext> {
    handle: DeviceHandle<C>,
    detached_kernel_driver: bool,
}
impl<C: UsbContext> InitializedDevice<C> {
    pub fn find_devices() -> rusb::Result<Vec<Device<C>>>
    where
        C: Default,
    {
        Ok(C::default()
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

    pub fn create_with_any() -> rusb::Result<Self>
    where
        C: Default,
    {
        Self::create(Self::find_devices()?.get(0).ok_or(rusb::Error::NotFound)?)
    }

    pub fn create(device: &Device<C>) -> rusb::Result<Self> {
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

    pub fn set_light(&self, color: Color, state: State) -> rusb::Result<()> {
        self.handle
            .write_interrupt(
                CTRL_ENDPOINT,
                &[0x00, color as u8, state as u8],
                std::time::Duration::from_secs(1000),
            )
            .map(|_| {})
    }
}
impl<C: UsbContext> Drop for InitializedDevice<C> {
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
