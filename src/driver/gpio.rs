use crate::sync::NullLock;

use super::DeviceDriver;
pub struct GPIOInner {}

impl GPIOInner {
    pub const fn new() -> GPIOInner {
        GPIOInner {}
    }
}

pub struct GPIODriver {
    inner: NullLock<GPIOInner>,
}

impl GPIODriver {
    pub const fn new() -> GPIODriver {
        GPIODriver {
            inner: NullLock::new(GPIOInner::new()),
        }
    }
}

impl DeviceDriver for GPIODriver {
    fn name(&self) -> &'static str {
        "GPIO"
    }

    fn init(&self) {}
}
