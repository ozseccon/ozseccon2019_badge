#![no_std]
#![no_main]

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// pick a panicking behavior
//extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
//extern crate panic_semihosting; // you can put a breakpoint on `rust_begin_unwind` to catch panics
//extern crate panic_abort;

//use cortex_m_semihosting::hprintln;
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
//extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m_rt::{entry, exception, ExceptionFrame};
//use cortex_m;
//use cortex_m::{interrupt::Mutex};
//use cortex_m::{interrupt, Interrupt};

//extern crate stm32f0xx_hal;

//use stm32f0xx_hal as hal;
//use stm32f0xx_hal::stm32 as stm32;

use stm32f0xx_hal as hal;
//use stm32f0::stm32f0x0;

//use crate::hal::{prelude::*, stm32};

//use stm32f0xx_hal::stm32f0::u
use crate::hal::{
    prelude::*,
    rcc::*, 
    //spi::Spi,
    gpio::*,
    time::*,
    timers::*,
    delay::Delay, 
    stm32,
    stm32::Interrupt,//device
    //interrupt,
};
use nb::block;

use crate::hal::stm32::interrupt;
use stm32f0xx_hal::stm32::Interrupt::EXTI0_1;

use cortex_m::{interrupt::Mutex, peripheral::syst::SystClkSource::Core, Peripherals};

use core::cell::RefCell;
use core::ops::DerefMut;

//pub mod ozsecconusb;
pub mod usb;
use usb::systick::*;
use usb::system::*;

//pub mod systick;
//pub mod system;

//static LED1_BLUE: Mutex<RefCell<Option<gpioa::PA8<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
//static LED1_RED: Mutex<RefCell<Option<gpioa::PA9<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
//static LED1_GREEN: Mutex<RefCell<Option<gpioa::PA10<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
//
//static LED2_BLUE: Mutex<RefCell<Option<gpioa::PA15<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
//static LED2_RED: Mutex<RefCell<Option<gpiob::PB3<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
//static LED2_GREEN: Mutex<RefCell<Option<gpiob::PB10<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

//teeth
//static TOOTH_1: Mutex<RefCell<Option<gpiob::PB11<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
//static TOOTH_2: Mutex<RefCell<Option<gpiob::PB12<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
//static TOOTH_3: Mutex<RefCell<Option<gpiob::PB13<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
//static TOOTH_4: Mutex<RefCell<Option<gpiob::PB14<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
//static TOOTH_5: Mutex<RefCell<Option<gpiob::PB15<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

static MYEXTI: Mutex<RefCell<Option<stm32f0xx_hal::stm32::EXTI>>> = Mutex::new(RefCell::new(None));

static TIMER: Mutex<RefCell<Option<stm32f0xx_hal::timers::Timer<stm32f0xx_hal::stm32::TIM2>>>> = Mutex::new(RefCell::new(None));

static SYSTEM: Mutex<RefCell<Option<System<SystickHardwareImpl>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut cp = stm32::CorePeripherals::take().unwrap();
    let mut dp = stm32::Peripherals::take().unwrap();
    //let syscfg = dp.SYSCFG; 
    //let mut nvic = &cp.NVIC;
    //let exti = &dp.EXTI; 
    
    //USB config  
    //dp.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
    //// Remap PA9-10 to PA11-12 for USB.
    //syscfg 
    //    .cfgr1
    //    .modify(|_, w| w.pa11_pa12_rmp().set_bit().mem_mode().main_flash());
    
    //let mut rcc = dp.RCC.configure().sysclk(48.mhz()).freeze(&mut dp.FLASH);
    //let mut clocks = rcc.clocks;
    //let rcc = dp.RCC.constrain();
    //let rcc = dp.RCC;
    //let clocks = rcc.cfgr.sysclk(8.mhz()).freeze();
    //let delay = Delay::new(cp.SYST, &mut rcc);
    //let timer2  = Timer::tim2(dp.TIM2, Hertz(1), &mut rcc);
    
    //let gpioa = dp.GPIOA.split(&mut rcc);
    //let gpiob = dp.GPIOB.split(&mut rcc);
     
    //cortex_m::interrupt::free(move |cs| { 
    cortex_m::interrupt::free(|cs| { 
        
        let mut system = System::new(
            dp,
            usb::systick::create(cp.SYST),
            cp.SCB,
        );
        system.setup();

        *SYSTEM.borrow(cs).borrow_mut() = Some(system); 
       
        cp.NVIC.enable(Interrupt::EXTI0_1); 
        cp.NVIC.enable(Interrupt::EXTI4_15); 
        cp.NVIC.enable(Interrupt::USB); 
        //let led1blue = gpioa.pa8.into_push_pull_output(cs);
        //let led1red = gpioa.pa9.into_push_pull_output(cs);
        //let led1green = gpioa.pa10.into_push_pull_output(cs);
        //  
        //let led2blue =  gpioa.pa15.into_push_pull_output(cs);
        //let led2red =  gpiob.pb3.into_push_pull_output(cs);
        //let led2green = gpiob.pb10.into_push_pull_output(cs);
        //
        //*LED1_BLUE.borrow(cs).borrow_mut() = Some(led1blue);
        //*LED1_RED.borrow(cs).borrow_mut() = Some(led1red);
        //*LED1_GREEN.borrow(cs).borrow_mut() = Some(led1green);
       
        //*LED2_BLUE.borrow(cs).borrow_mut() = Some(led2blue);
        //*LED2_RED.borrow(cs).borrow_mut() = Some(led2red);
        //*LED2_GREEN.borrow(cs).borrow_mut() = Some(led2green);
 
        //setup switch 1 
        //syscfg.exticr1.modify(|_, w| unsafe{ w.exti0().bits(0b000)});
        //setup switch 2 
        //syscfg.exticr2.modify(|_, w| unsafe{ w.exti4().bits(0b000)});
        //exti.ftsr.modify(|_,w|  w.tr0().set_bit().tr4().set_bit());
        //exti.rtsr.modify(|_,w|  w.tr0().clear_bit().tr4().clear_bit());
        //exti.imr.modify(|_,w|  w.mr0().set_bit().mr4().set_bit());
        //unsafe{nvic.set_priority(Interrupt::EXTI0_1, 1);} 
        //unsafe{nvic.set_priority(Interrupt::EXTI4_15, 1);} 
        //nvic.enable(Interrupt::EXTI0_1); 
        //nvic.enable(Interrupt::EXTI4_15); 
        //*MYEXTI.borrow(cs).borrow_mut() = Some(exti);
        //*TIMER.borrow(cs).borrow_mut() = Some(timer2);
    
        //setup USB
        // Set "high" output speed for PA11 and PA12.
        //let _ = gpioa.pa11.into_push_pull_output_hs(cs).into_alternate_af2(cs);
        //let _ = gpioa.pa12.into_push_pull_output_hs(cs).into_alternate_af2(cs);
        //nvic.enable(Interrupt::USB); 
    });
 
    loop {
        cortex_m::asm::wfi(); 
        //if switch2.is_low() { 
        //    mymagspoof.play_card(&mycard);
        //    led2red.set_high(); 
        //    block!(timer2.wait()).ok();
        //    led2red.set_low(); 
        //}        //continue;
    }
}

//#[interrupt]
//fn EXTI0_1()
//{
//    let trk1 = "%B4444444444444444^ABE/LINCOLN^291110100000931?";
//    let trk2 = ";4444444444444444=29111010000093100000?";
//    //let track2 = ";12=34?";
//    let trk3 = "";
//
//    let mycard = magspoof::card{
//        track1: &trk1.as_bytes(), 
//        track2: &trk2.as_bytes(),
//        track3: &trk3.as_bytes(), 
//    };    
//    cortex_m::interrupt::free(|cs| {
//        //let Some(ref mut magspoof) = *MAGSPOOF.borrow(cs).borrow_mut().deref_mut(); 
//        if let Some(ref mut exti) = *MYEXTI.borrow(cs).borrow_mut().deref_mut() {
//            if exti.imr.read().mr0().bit_is_set() && exti.pr.read().pif0().bit_is_set() {  
//                if let Some(ref mut led1blue) = *LED1_BLUE.borrow(cs).borrow_mut().deref_mut() {
//                    led1blue.toggle(); 
//                }
//                if let Some(ref mut magspoof) = *MAGSPOOF.borrow(cs).borrow_mut().deref_mut() {
//                    magspoof.play_track(&mycard,2); 
//                }
//                if let Some(ref mut timer) = *TIMER.borrow(cs).borrow_mut().deref_mut() {
//                    block!(timer.wait()).ok();
//                }
//                if let Some(ref mut led1blue) = *LED1_BLUE.borrow(cs).borrow_mut().deref_mut() {
//                    led1blue.toggle(); 
//                }
//                exti.pr.modify(|_,w| w.pif0().set_bit());
//        } 
//        } 
//    });
//    cortex_m::peripheral::NVIC::unpend(Interrupt::EXTI0_1);
//}

//#[interrupt]
//fn EXTI4_15()
//{
//    let trk1 = "%B4444444444444444^ABE/LINCOLN^291110100000931?";
//    let trk2 = ";4444444444444444=29111010000093100000?";
//    //let track2 = ";12=34?";
//    let trk3 = "";
//
//    let mycard = magspoof::card{
//        track1: &trk1.as_bytes(), 
//        track2: &trk2.as_bytes(),
//        track3: &trk3.as_bytes(), 
//    };
//    cortex_m::interrupt::free(|cs| {
//        if let Some(ref mut exti) = *MYEXTI.borrow(cs).borrow_mut().deref_mut() {
//            if exti.imr.read().mr4().bit_is_set() && exti.pr.read().pif4().bit_is_set() {  
//                if let Some(ref mut led1red) = *LED1_RED.borrow(cs).borrow_mut().deref_mut() {
//                        led1red.toggle(); 
//                    }
//                if let Some(ref mut magspoof) = *MAGSPOOF.borrow(cs).borrow_mut().deref_mut() {
//                    magspoof.play_track(&mycard, 1); 
//                }
//                if let Some(ref mut timer) = *TIMER.borrow(cs).borrow_mut().deref_mut() {
//                    block!(timer.wait()).ok();
//                }
//                if let Some(ref mut led1red) = *LED1_RED.borrow(cs).borrow_mut().deref_mut() {
//                    led1red.toggle(); 
//                } 
//            exti.pr.modify(|_,w| w.pif4().set_bit());
//        } 
//        } 
//    });
//    cortex_m::peripheral::NVIC::unpend(Interrupt::EXTI4_15);
//}

#[interrupt]
fn USB() {
    interrupt_free(|system| system.on_usb_packet());
}

#[interrupt]
fn EXTI0_1() {
    interrupt_free(|system| system.on_button_press());
}

#[interrupt]
fn EXTI4_15() {
    interrupt_free(|system| system.on_button_press());
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    panic!("hard fault (PC={})", _ef.pc);
}

fn interrupt_free<F>(f: F)
where
    F: FnOnce(&mut usb::system::System<usb::systick::SystickHardwareImpl>),
{
    cortex_m::interrupt::free(|cs| {
        if let Some(s) = SYSTEM.borrow(cs).borrow_mut().as_mut() {
            f(s);
        } else {
            panic!("Can not borrow application state!");
        }
    });
}
