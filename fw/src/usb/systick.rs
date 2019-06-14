use cortex_m::peripheral::{syst::SystClkSource, SYST};
pub const CLOCK_SPEED: u32 = 8_000_000;
pub const DEVICE_VID: u16 = 0xffff;
pub const DEVICE_PID: u16 = 0xffff;

use cortex_m_semihosting::hprintln;

//use systick::{SysTick, SysTickHardware};

/// Describes the SysTick hardware management interface.
pub trait SysTickHardware {
    fn configure(&mut self, reload_value: u32);
    fn enable_counter(&mut self);
    fn disable_counter(&mut self);
    fn has_wrapped(&mut self) -> bool;
}

pub struct SysTick<T: SysTickHardware> {
    hw: T,
}

impl<T: SysTickHardware> SysTick<T> {
    pub fn new(hw: T) -> Self {
        SysTick { hw }
    }

    pub fn delay_us(&mut self, us: u32) {
        let rvr = us * (CLOCK_SPEED / 1_000_000);

        assert!(rvr < (1 << 24), "timeout is too large");

        self.hw.configure(rvr);

        self.hw.enable_counter();
        while !self.hw.has_wrapped() {}
        self.hw.disable_counter();
    }

    pub fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1000);
    }
}

pub struct SystickHardwareImpl {
    syst: SYST,
}

impl SysTickHardware for SystickHardwareImpl {
    fn configure(&mut self, reload_value: u32) {
        self.syst.set_clock_source(SystClkSource::Core);
        self.syst.set_reload(reload_value);
        self.syst.clear_current();
    }

    fn enable_counter(&mut self) {
        self.syst.enable_counter();
    }

    fn disable_counter(&mut self) {
        self.syst.disable_counter();
    }

    fn has_wrapped(&mut self) -> bool {
        self.syst.has_wrapped()
    }
}

pub fn create(syst: SYST) -> SysTick<SystickHardwareImpl> {
    SysTick::new(SystickHardwareImpl { syst })
}
