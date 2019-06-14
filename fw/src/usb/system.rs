//use crate::{beeper, buttons, flash, rtc, usb};
//use usb;

//use stm32f0::stm32f0x2::Peripherals;
//use stm32f0xx_hal::stm32::Peripherals;

//use cortex_m_semihosting::hprintln;
//extern crate panic_semihosting;
extern crate panic_abort;
//mod usb;
//mod self::systick;

//use usb::{command_packet::CommandPacket, USBHardware, USB, UsbState};
use cortex_m::peripheral::{syst::SystClkSource, SYST, SCB};

use stm32f0xx_hal::stm32::{Peripherals, GPIOA, GPIOB, RCC};
use stm32f0xx_hal::rcc::{RccExt, Rcc, CFGR};
use stm32f0xx_hal::{time::*};
use stm32f0xx_hal::{gpio::GpioExt};

//use kroneum_api::{
//    beeper::{Melody, PWMBeeper, PWMBeeperHardware},
//    buttons::{ButtonPressType, Buttons, ButtonsHardware},
//    flash::{Flash, FlashHardware},
//    rtc::{RTCHardware, RTC},
//    system::{SystemHardware, SystemMode, SystemState},
//    systick::{SysTick, SysTickHardware},
//    time::Time,
//    usb::{command_packet::CommandPacket, USBHardware, USB},
//};
//use crate::usb::{USBHardware,USB,SystemHardware,UsbState,command_packet, CommandPacket};
use crate::usb::{USBHardware,USB,UsbState,command_packet, CommandPacket};
use crate::usb::systick::{SysTick,SysTickHardware};
use crate::usb::usb::*;

use crate::usb::leds::{Leds, LedsHardware};
use crate::usb::flash::{FlashHardwareImpl};
use crate::usb::flash::*;
use crate::usb::buttons as buttons;
use crate::usb::magspoof::*;

#[derive(Debug, Copy, Clone)]
pub enum SystemMode {
    Idle,
    Setup(u32),
    //Alarm(Time, Melody),
    Config,
}

#[derive(Debug, Copy, Clone)]
pub enum TrackMode {
    track1,
    track2,
    //Alarm(Time, Melody),
    track3,
}

#[derive(Debug, Copy, Clone)]
pub struct TrackSelector {
    state: TrackMode,
}

impl From<TrackMode> for TrackSelector {
    fn from(item: TrackMode) -> Self {
        match item {
            TrackMode::track1 => TrackSelector{state: TrackMode::track2}, 
            TrackMode::track2 => TrackSelector{state: TrackMode::track3}, 
            TrackMode::track3 => TrackSelector{state: TrackMode::track1},
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CardMode {
    card1,
    card2,
    card3,
//    card4,
}

#[derive(Debug, Copy, Clone)]
pub struct CardSelector {
    state: CardMode,
}

impl From<CardMode> for CardSelector {
    fn from(item: CardMode) -> Self {
        match item {
            CardMode::card1 => CardSelector{state: CardMode::card2}, 
            CardMode::card2 => CardSelector{state: CardMode::card3}, 
            CardMode::card3 => CardSelector{state: CardMode::card1},
//            CardMode::card4 => CardSelector{state: CardMode::card1},
        }
    }
}
//#[derive(Copy, Clone)]
pub struct SystemState {
    pub mode: SystemMode,
    pub usb_state: UsbState,
    pub track: TrackSelector,
    pub cardsel: CardSelector,
    pub pwmmode: bool,
}

impl Default for SystemState {
    fn default() -> Self {
        SystemState {
            mode: SystemMode::Idle,
            usb_state: UsbState::default(),
            track: TrackSelector{state: TrackMode::track1},
            cardsel: CardSelector{state: CardMode::card1},
            pwmmode: false,
        }
    }
}

pub struct SystemHardwareImpl {
    p: Peripherals,
    scb: SCB,
}

impl SystemHardwareImpl {
    fn p(&mut self) -> &mut Peripherals {
        &mut self.p
    }
    //fn rcc(&mut self) -> &mut RCC {
    //    &mut self.rcc
    //}
}
// Describes the System hardware management interface.

pub trait SystemHardware {
    //type L: leds::LedsHardware;

    // Initializes hardware if needed.
    fn setup(&self);

    // Turns on/off system standby mode.
    fn toggle_standby_mode(&mut self, on: bool);

    // Performs a software reset.
    fn reset(&mut self);

    // Returns an `RTCHardware` used to create `RTC` component.
    //fn rtc<'b: 'a>(&'b self) -> Self::R;

    // returns an `LEDShardware used to create LED stuff
    //fn Leds<'b: 'a>(&'b self) -> Self::L;
}

impl SystemHardware for SystemHardwareImpl {
    //type F = FlashHardwareImpl<'a>;
    //type U = USBHardwareImpl<'a>; 
    //type R = rtc::RTCHardwareImpl<'a>;

    fn setup(&self) {
        //hprintln!("initialising hardware...");
        // Remap PA9-10 to PA11-12 for USB.
        //let rcc = self.p.RCC.configure().sysclk(8.mhz()).hsi48().freeze(&mut self.p.FLASH);
        //let rcc = self.p.RCC.configure();
        //let gpioa = self.p.GPIOA.split(&mut self.p.RCC); 
        
        self.p.RCC.apb1enr.modify(|_, w| w.usben().set_bit().crsen().enabled());
        self.p.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
        self.p.RCC.cfgr3.modify(|_, w| w.usbsw().clear_bit());
        self.p.RCC.cr2.modify(|_, w| w.hsi48on().on());
        self.p
            .SYSCFG
            .cfgr1
            .modify(|_, w| w.pa11_pa12_rmp().set_bit().mem_mode().main_flash());

        // -----------Buttons----------------
        // enable pull-ups on button GPIOs
        //self.p
        //    .GPIOA
        //    .pupdr
        //    .modify(|_, w| w.pupdr0().pull_up().pupdr4().pull_up()); 
        // Enable EXTI0 interrupt line for PA0.
        self.p
            .SYSCFG
            .exticr1
            .modify(|_, w| w.exti0().pa0());
        self.p
            .SYSCFG
            .exticr2
            .modify(|_, w| w.exti4().pa4());

        // Configure PA0/PA4 to trigger an interrupt event on the EXTI0/EXTI4 line on a falling edge.
        self.p
            .EXTI
            .ftsr
            .modify(|_, w| w.tr0().set_bit().tr4().set_bit());

        // Unmask the external interrupt line EXTI0\EXTI4 by setting the bit corresponding to the
        // EXTI0\EXTI4 "bit 0/4" in the EXT_IMR register.
        self.p
            .EXTI
            .imr
            .modify(|_, w| w.mr0().set_bit().mr4().set_bit());

        // ---------GPIO------------------

        // Enable clock for GPIO Port A, B and F.
        self.p
            .RCC
            .ahbenr
            .modify(|_, w| w.iopaen().set_bit().iopben().set_bit().iopfen().set_bit());

        // Switch PA11 and PA12 (usb) to alternate function
        // mode. 
        self.p.GPIOA.moder.modify(|_, w| {
                w.moder11()
                .alternate()
                .moder12()
                .alternate()
        });

        // Enable AIN for GPIO B and F to reduce power consumption.
        //self.p
        //    .GPIOB
        //    .moder
        //    .modify(|_, w| w.moder1().analog().moder8().analog());
        //self.p
        //    .GPIOF
        //    .moder
        //    .modify(|_, w| w.moder0().analog().moder1().analog());

        //self.p
        //    .RCC
        //    .ahbenr
        //    .modify(|_, w| w.iopben().clear_bit().iopfen().clear_bit());

        // Enable pull-down for PA0 and PA2.
        //self.p
        //    .GPIOA
        //    .pupdr
        //    .modify(|_, w| w.pupdr0().pull_down().pupdr2().pull_down());

        // Set "high" output speed for PA11 and PA12.
        self.p.GPIOA.ospeedr.modify(|_, w| {
                w.ospeedr11().very_high_speed().ospeedr12().very_high_speed()
        });

        // Set alternative function #2 for PA0 (WKUP1), PA2 (WKUP4) and PA7 (TIM1_CH1N).
        //self.p
        //    .GPIOA
        //    .afrl
        //    .modify(|_, w| w.afrl0().af2().afrl2().af2().afrl7().af2());

        // Set alternative function #2 (USB) for PA11 and PA12.
        self.p
            .GPIOA
            .afrh
            .modify(|_, w| w.afrh11().af2().afrh12().af2());
    }

    fn toggle_standby_mode(&mut self, on: bool) {
        // Toggle STANDBY mode.
        //self.p.PWR.cr.modify(|_, w| w.pdds().bit(on));

        //self.p.PWR.cr.modify(|_, w| w.cwuf().set_bit());

        //// Toggle SLEEPDEEP bit of Cortex-M0 System Control Register.
        //if on {
        //    self.scb.set_sleepdeep();
        //} else {
        //    self.scb.clear_sleepdeep();
        //}
    }

    fn reset(&mut self) {
        self.scb.system_reset();
    }
    
    //fn usb(&self) -> Self::U {
    //    crate::usb::usb::USBHardwareImpl::new(&self.p)
    //}
    //
    //fn flash(&self) -> Self::F {
    //    crate::usb::flash::FlashHardwareImpl::new(&self.p)
    //} 
    //fn rtc<'b: 'a>(&'b self) -> Self::R {
    //    rtc::RTCHardwareImpl { p: &self.p }
    //}

}

pub struct System<S: SysTickHardware> {
    hw: SystemHardwareImpl,
    systick: SysTick<S>,
    state: SystemState,
}

impl<S: SysTickHardware> System<S> {
    pub fn new(p: Peripherals, systick: SysTick<S>, scb: SCB) -> Self {
        System {
            hw: SystemHardwareImpl {p, scb},
            state: SystemState::default(),
            systick,
        }
    }

    pub fn setup(&mut self) {
        self.hw.setup();

        self.leds().setup();
        self.buttons().setup();
        self.magspoof().setup();
        //self.set_mode(SystemMode::Idle);
        self.set_mode(SystemMode::Config);
    }
    
    pub fn set_mode(&mut self, mode: SystemMode) {
        match &mode {
            SystemMode::Idle => {
                self.hw.toggle_standby_mode(true);

                self.usb().teardown();
                //self.rtc().teardown();

                // If we are exiting `Config` or `Alarm` mode let's play special signal.
                //if let SystemMode::Setup(_) = self.state.mode {
                //    self.beeper().play(Melody::Reset);
                //} else if let SystemMode::Alarm(_, _) = self.state.mode {
                //    self.beeper().play(Melody::Reset);
                //}
            }
            SystemMode::Config => {
                //self.beeper().play(Melody::Reset);

                self.hw.toggle_standby_mode(false);

                self.usb().setup();
            }
            //SystemMode::Setup(0) => self.beeper().play(Melody::Setup),
            //SystemMode::Setup(c) if *c > 0 => self.beeper().beep(),
            //SystemMode::Alarm(time, _) => {
            //    self.beeper().play(Melody::Setup);

            //    let rtc = self.rtc();
            //    rtc.setup();
            //    rtc.set_time(Time::default());
            //    rtc.set_alarm(*time);
            //}
            _ => {}
        }

        self.state.mode = mode;
    }

    //pub fn on_rtc_alarm(&mut self) {
    //    if let SystemMode::Alarm(_, melody) = self.state.mode {
    //        self.beeper().play(melody);

    //        self.rtc().teardown();

    //        // Snooze alarm for 10 seconds.
    //        self.set_mode(SystemMode::Alarm(Time::from_seconds(10), Melody::Beep));
    //    }
    //}

    pub fn on_usb_packet(&mut self) {
        self.usb().interrupt();

        if let Some(command_packet) = self.state.usb_state.command {
            if let CommandPacket::TurnOnLED1 = command_packet {
                self.leds().turnonLED1(); 
            } 
            else if let CommandPacket::TurnOffLED1 = command_packet {
                self.leds().turnoffLED1(); 
            }
            else if let CommandPacket::ToggleLED1 = command_packet {
                self.leds().toggleLED1(); 
            } 
            else if let CommandPacket::TurnOnLED2 = command_packet {
                self.leds().turnonLED2(); 
            } 
            else if let CommandPacket::TurnOffLED2 = command_packet {
                self.leds().turnoffLED2(); 
            }
            else if let CommandPacket::ToggleLED2 = command_packet {
                self.leds().toggleLED2(); 
            }
            else if let CommandPacket::TurnOnLED3 = command_packet {
                self.leds().turnonLED3(); 
            } 
            else if let CommandPacket::TurnOffLED3 = command_packet {
                self.leds().turnoffLED3(); 
                
            }
            else if let CommandPacket::ToggleLED3 = command_packet {
                self.leds().toggleLED3(); 
            }
            else if let CommandPacket::TurnOnLED4 = command_packet {
                self.leds().turnonLED4(); 
            } 
            else if let CommandPacket::TurnOffLED4 = command_packet {
                self.leds().turnoffLED4(); 
            }
            else if let CommandPacket::ToggleLED4 = command_packet {
                self.leds().toggleLED4(); 
            }
            else if let CommandPacket::TurnOnLED5 = command_packet {
                self.leds().turnonLED5(); 
            } 
            else if let CommandPacket::TurnOffLED5 = command_packet {
                self.leds().turnoffLED5(); 
                
            }
            else if let CommandPacket::ToggleLED5 = command_packet {
                self.leds().toggleLED5(); 
            }
            else if let CommandPacket::TurnOnLED6 = command_packet {
                self.leds().turnonLED6(); 
            } 
            else if let CommandPacket::TurnOffLED6 = command_packet {
                self.leds().turnoffLED6(); 
            }
            else if let CommandPacket::ToggleLED6 = command_packet {
                self.leds().toggleLED6(); 
            }
            else if let CommandPacket::EraseAll = command_packet {
                self.flash().erase_all(); 
            }
            else if let CommandPacket::TrackLoad(slot, num_chunk, track_chunk) = command_packet {
                self.flash().write_chunk(slot as usize, num_chunk as usize, track_chunk); 
            }
            else if let CommandPacket::ReadCard(slot) = command_packet {
                let trackdata = self.flash().read(slot as usize); 
                //self.usb().send(&[1,2,3,4]); 
                for i in trackdata.chunks(64) { 
                    self.usb().send(&i);
                }
            } 
            //else if let CommandPacket::ToggleBlueLed = command_packet {
            //if let CommandPacket::ToggleBlueLed = command_packet {
            //    self.leds().toggletooth1(); 
            //} 
            //if let CommandPacket::Beep(num) = command_packet {
            //    self.beeper().beep_n(num);
            //} 
            //else if let CommandPacket::AlarmSet(time) = command_packet {
            //    self.set_mode(SystemMode::Alarm(time, Melody::Alarm));
            //} else if let CommandPacket::AlarmGet = command_packet {
            //    let alarm = self.rtc().alarm();
            //    self.usb()
            //        .send(&[alarm.hours, alarm.minutes, alarm.seconds, 0, 0, 0]);
            //} else if let CommandPacket::Reset = command_packet {
            //    self.hw.reset();
            //} else if let CommandPacket::FlashRead(slot) = command_packet {
            //    let value = self.flash().read(slot).unwrap_or_else(|| 0);
            //    self.usb().send(&[value, 0, 0, 0, 0, 0]);
            //} else if let CommandPacket::FlashWrite(slot, value) = command_packet {
            //    let status = if self.flash().write(slot, value).is_ok() {
            //        1
            //    } else {
            //        0
            //    };
            //    self.usb().send(&[status, 0, 0, 0, 0, 0]);
            //} else if let CommandPacket::FlashEraseAll = command_packet {
            //    self.flash().erase_all();
            //}
        }

        self.state.usb_state.command = None;
    }

    pub fn on_button_press(&mut self) {
        if !buttons::has_pending_interrupt(&self.hw.p()) {
            return;
        }

        let (button_i, button_x) = self.buttons().interrupt();

        match (button_i, button_x) {
            //transmit currently selected track 
 
            (buttons::ButtonPressType::Short,_) => {
                //save the gpio state
                self.leds().turnonLED1();
                let gpio_a = self.hw.p.GPIOA.idr.read().bits();  
                let gpio_b = self.hw.p.GPIOB.idr.read().bits();  
                //load the current card
                let currentcard = match self.state.cardsel.state {
                    CardMode::card1 => 0, 
                    CardMode::card2 => 1, 
                    CardMode::card3 => 2, 
                    _ => 0, 
                }; 
                let raw_data = self.flash().read(currentcard);
                if raw_data[0] != 0xff { //make sure we're not reading unwritten flash data... 
                    let (track1,track2,track3) = crate::usb::magspoof::card::deserialize(&raw_data); 
                    let newcard = crate::usb::magspoof::card{track1: track1, track2: track2, track3: track3};
                      
                    match self.state.track.state {
                        TrackMode::track1 => self.magspoof().play_track(&newcard, 1),
                        TrackMode::track2 => self.magspoof().play_track(&newcard, 2),
                        TrackMode::track3 => self.magspoof().play_track(&newcard, 3),
                        _ => (),
                    };
                } 
                self.hw.p.GPIOA.odr.write(|w| unsafe{w.bits(gpio_a)});  
                self.hw.p.GPIOB.odr.write(|w| unsafe{w.bits(gpio_b)});  
                self.leds().turnoffLED1();
                //serialise card
                //let mycardserialized = mycard.serialize();
                //let unserializedcard = &mycard.deserialize(&mycardserialized);
                //self.flash().erase_all(); 
                //self.flash().write(0, mycard); 
                //let raw_data = self.flash().read(0);
                //let (track1, track2, track3) = crate::usb::magspoof::card::deserialize(self.flash().read(0)); 
            },
            //change track 
            (_,buttons::ButtonPressType::Short) => {
                self.state.track = TrackSelector::from(self.state.track.state);
                //toggle LED based on selected track
                match self.state.track.state {
                    TrackMode::track1 => { self.leds().turnonLED4(); self.leds().turnoffLED5(); self.leds().turnoffLED6();},
                    TrackMode::track2 => { self.leds().turnonLED5(); self.leds().turnoffLED4(); self.leds().turnoffLED6();},
                    TrackMode::track3 => { self.leds().turnonLED6(); self.leds().turnoffLED4(); self.leds().turnoffLED5();},
                    _ => (),
                }
            },
            (_,buttons::ButtonPressType::Long) => {
                self.state.cardsel = CardSelector::from(self.state.cardsel.state);
                //toggle LED based on selected track
                match self.state.cardsel.state {
                    CardMode::card1 => { self.leds().turnonLED1(); self.leds().turnoffLED2(); self.leds().turnoffLED3();},
                    CardMode::card2 => { self.leds().turnonLED2(); self.leds().turnoffLED3(); self.leds().turnoffLED1();},
                    CardMode::card3 => { self.leds().turnonLED3(); self.leds().turnoffLED1(); self.leds().turnoffLED2();},
                    //CardMode::card4 => { self.leds().turnonLED3(); self.leds().turnoffLED4(); self.leds().turnoffLED5();},
                    _ => (),
                }
            }, 
            (buttons::ButtonPressType::Long,_) => {
                    if self.state.pwmmode {
                        self.leds().setup_pwm(); 
                        self.leds().enable_LED4();
                        let max_duty = self.leds().get_max_duty_LED4();
                        self.leds().set_duty_LED4(max_duty/2);
                        self.state.pwmmode = false;
                    }
                    else {
                        self.leds().setup(); 
                        self.state.pwmmode = true;
                    } 
            },
            _ => (),  
        };
        buttons::clear_pending_interrupt(self.hw.p());

    //    match (self.state.mode, button_i, button_x) {
    //        (mode, ButtonPressType::Long, ButtonPressType::Long) => {
    //            let (button_i, button_x) = self.buttons().interrupt();

    //            match (mode, button_i, button_x) {
    //                (SystemMode::Config, ButtonPressType::Long, ButtonPressType::Long)
    //                | (SystemMode::Alarm(_, _), ButtonPressType::Long, ButtonPressType::Long) => {
    //                    self.set_mode(SystemMode::Idle)
    //                }
    //                (_, ButtonPressType::Long, ButtonPressType::Long) => {
    //                    self.set_mode(SystemMode::Config)
    //                }
    //                (SystemMode::Setup(counter), _, _) => {
    //                    self.set_mode(SystemMode::Alarm(Time::from_hours(counter), Melody::Alarm))
    //                }
    //                _ => {}
    //            }
    //        }
    //        (SystemMode::Idle, ButtonPressType::Long, _)
    //        | (SystemMode::Idle, _, ButtonPressType::Long)
    //        | (SystemMode::Alarm(_, _), ButtonPressType::Long, _)
    //        | (SystemMode::Alarm(_, _), _, ButtonPressType::Long) => {
    //            self.set_mode(SystemMode::Setup(0))
    //        }
    //        (SystemMode::Setup(counter), ButtonPressType::Long, _)
    //        | (SystemMode::Setup(counter), _, ButtonPressType::Long) => {
    //            let time = match button_i {
    //                ButtonPressType::Long => Time::from_seconds(counter as u32),
    //                _ => Time::from_minutes(counter as u32),
    //            };

    //            self.set_mode(SystemMode::Alarm(time, Melody::Alarm));
    //        }
    //        (SystemMode::Setup(counter), ButtonPressType::Short, _) => {
    //            self.set_mode(SystemMode::Setup(counter + 1))
    //        }
    //        (SystemMode::Setup(counter), _, ButtonPressType::Short) => {
    //            self.set_mode(SystemMode::Setup(counter + 10))
    //        }
    //        _ => {}
    //    }

    //    buttons::clear_pending_interrupt(self.hw.p());
    //}

    // Creates an instance of `Beeper` controller.
    //fn beeper<'a>(
    //    &'a mut self,
    //) -> PWMBeeper<'a, impl PWMBeeperHardware + 'a, impl SysTickHardware> {
    //    beeper::create(self.hw.p(), &mut self.systick)
    }

    // Creates an instance of `Buttons` controller.
    fn buttons<'a>(&'a mut self) -> buttons::Buttons<'a, impl buttons::ButtonsHardware + 'a, impl SysTickHardware> {
        crate::usb::buttons::create(self.hw.p(), &mut self.systick)
    }
    // Creates an instance of `Leds` controller.
    fn leds<'a>(&'a mut self) -> Leds<'a, impl LedsHardware + 'a, impl SysTickHardware> {
        crate::usb::leds::create(self.hw.p(), &mut self.systick)
    }
    // Creates an instance of `RTC` controller.
    //fn rtc<'a>(&'a mut self) -> RTC<impl RTCHardware + 'a> {
    //    RTC::new(self.hw.rtc())
    //}

    // Creates an instance of `USB` controller.
    fn usb<'a>(&'a mut self) -> USB<impl USBHardware + 'a> {
        //usb::create(self.hw.p(), &mut self.state.usb_state)
        crate::usb::usb::create(self.hw.p(), &mut self.state.usb_state)
    }

    // Creates an instance of `Flash` controller.
    fn flash<'a>(&'a mut self) -> Flash<impl FlashHardware+ 'a> {
        crate::usb::flash::create(self.hw.p())
    }
    // Create an instance of the magspoof
    fn magspoof<'a>(&'a mut self) -> Magspoof<impl MagspoofHardware+ 'a, impl SysTickHardware> {
        crate::usb::magspoof::create(self.hw.p(), &mut self.systick)
    }
}
