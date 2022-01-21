use self::gpio::GPIODriver;

mod gpio;

pub trait DeviceDriver {
    fn name(&self) -> &'static str;

    fn init(&self);

    fn post_init(&self) {}
}

pub struct DriverManager {
    drivers: [&'static (dyn DeviceDriver + Sync); 1],
}

impl DriverManager {
    pub fn init(&self) {
        for driver in self.drivers.iter() {
            driver.init();
        }
    }
}

static GPIO: GPIODriver = GPIODriver::new();
static DRIVER_MANAGER: DriverManager = DriverManager { drivers: [&GPIO] };

pub fn driver_manager() -> &'static DriverManager {
    &DRIVER_MANAGER
}
