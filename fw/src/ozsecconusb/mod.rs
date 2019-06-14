#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use core::slice;
use core::mem;
//use cortex_m::interrupt::CriticalSection;
use vcell::VolatileCell;
use cortex_m::interrupt;
use stm32f0xx_hal::stm32::{USB, usb, RCC, rcc};
use usb_device::{Result, UsbDirection, UsbError};
use usb_device::endpoint::{EndpointType, EndpointAddress};

use cortex_m::interrupt::Mutex;
use core::cell::RefCell;
use usb_device::bus::{UsbBusAllocator, PollResult};
//use cortex_m::asm::delay;
use stm32f0xx_hal::prelude::*;
use stm32f0xx_hal::gpio::{self, gpioa};
//use crate::atomic_mutex::AtomicMutex;

//pub mod endpoint;
mod endpoint;
//use usb_device::endpoint::{NUM_ENDPOINTS, EP0, EP1, EP2, EP3, EP4, EP5, EP6, EP7, EndpointStatus, calculate_count_rx};

struct Reset {
    delay: u32,
    pin: Mutex<RefCell<gpioa::PA12<gpio::Output<gpio::PushPull>>>>,
}

/// USB peripheral driver for STM32F042 microcontrollers.
pub struct UsbBus {
    //regs: AtomicMutex<USB>,
    regs: USB,
    //endpoints: [endpoint::EP0, endpoint::EP1, endpoint::EP2, endpoint::EP3, endpoint::EP4, endpoint::EP5, endpoint::EP6, endpoint::EP7],
    next_ep_mem: usize,
    max_endpoint: usize,
    reset: Option<Reset>,
}

