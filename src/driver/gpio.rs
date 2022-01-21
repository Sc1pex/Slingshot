use super::DeviceDriver;

pub struct GPIODriver {}

impl GPIODriver {
    pub const fn new() -> GPIODriver {
        GPIODriver {}
    }
}

impl DeviceDriver for GPIODriver {
    fn name(&self) -> &'static str {
        "GPIO"
    }

    fn init(&self) {}
}
