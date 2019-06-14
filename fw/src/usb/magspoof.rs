//! Magspoof thingy for OzSecCon2019 
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1
//!
//! # Examples
//!
//! 
//! 
//! 
//!
//! 
//!
//! # References
//!
//! - [magspoof by Samy Kamkar][1]
//! - [rysccorp magspoof version][2]
//!
//! [1]: https://samy.pl/magspoof/ 
//! [2]: https://github.com/RyscCorp/magspoof_r3 

#![allow(dead_code)]
//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]
use crate::usb::systick::{SysTick, SysTickHardware};
use stm32f0xx_hal::stm32::Peripherals;

extern crate stm32f0xx_hal as stm32_hal;
extern crate embedded_hal as hal;
//extern crate embedded_hal as hal;
//extern crate generic_array;
//extern crate heapless;

extern crate cortex_m_semihosting;
extern crate cortex_m;
//use cortex_m::asm::bkpt;
//use cortex_m_semihosting::hprintln;

//use core::mem;

use hal::blocking::delay::{DelayMs, DelayUs};

//use serde::{Serialize, Deserialize};
//use serde_json::ron;

#[derive(Clone,Copy,Debug)]
pub struct card<'a> {
    pub track1: &'a [u8], //String<U79>,
    //track1: [u8; 79], //String<U79>,
    pub track2: &'a [u8], //String<U40>,
    //track2: [u8; 40], //String<U40>,
    pub track3: &'a [u8], //String<U107>,    
    //track3: [u8; 107], //String<U107>,    
}

impl<'a> card<'a> {
    fn period(&self, track_num: usize) -> u32 {
       1000000 / (SWIPE_SPEED * track_density[track_num])  
    }
   pub fn serialize(self) -> [u8;229] {
        let mut serializearray: [u8;229] = [0;229]; //max size of card array will be 229 (226+3) 
        //let mut serializearray: [u8; 3+self.track1.len()+ self.track2.len()+self.track3.len()];
        serializearray[0] = self.track1.len() as u8;
        //for i in self.track1.ter() {
        //    serial
        //}
        //[self.track1.len() as u8, self.track1, self.track2.len() as u8, self.track2, self.track3.len, self.track3]
        //serializearray[0] = self.track1.len() as u8;
        for (i,n) in self.track1.iter().enumerate() {
            serializearray[1+i] = *n;    
        }; 
        serializearray[1+self.track1.len()] = self.track2.len() as u8;
        for (i,n) in self.track2.iter().enumerate() {
            serializearray[1+1+self.track1.len()+i] = *n;    
        }; 
        
        serializearray[1+self.track1.len()+1+self.track2.len()] = self.track3.len() as u8;
        for (i,n) in self.track3.iter().enumerate() {
            serializearray[1+self.track1.len()+1+self.track2.len()+1+i] = *n;    
        };
        //bkpt(); 
        //hprintln!("{:x}", &serializearray); 
        //for i in serializearray.iter() {
        //    hprintln!("{:?}", i); 
        //} 
        serializearray 
        //[self.track1.len() as u8, &self.track1[..], self.track2.len() as u8, &self.track2[..], self.track3.len, &self.track3[..]]
        //[&self.track1.len(), &self.track1[..]].concat()
    }
    
    pub fn deserialize(data: &[u8]) -> (&[u8],&[u8],&[u8]) { //[u8;79] { //([u8;79], [u8;40], [u8;107]) { // -> card<'a> {// { //-> card<'a> {
        let track1_len = data[0] as usize; 
        //let mut track1: [u8;79] = [0;79];
        //track1.copy_from_slice(&data[1..(track1_len+1)]);
        let track1 = &data[1..(track1_len+1)];
        //let mut track2: [u8;40] = [0;40];
        let track2_len = data[track1_len+1] as usize; 
        //track2.copy_from_slice(&data[(1+track1_len+1)..(1+track1_len+1+track2_len)]);
        let track2 = &data[(1+track1_len+1)..(1+track1_len+1+track2_len)];
        //
        let track3_len = (data[1+track1_len+1+track2_len]+1) as usize; 
        //let mut track3: [u8;107] = [0;107];
        //track3.copy_from_slice(&data[(1+track1_len+1+track2_len+1)..(1+track1_len+1+track2_len+track3_len)]);
        let track3 = &data[(1+track1_len+1+track2_len+1)..(1+track1_len+1+track2_len+track3_len)];
        (track1, track2, track3) 
        //track1
        //card{track1: &track1, track2: &track2, track3: &track3}      
    } 
}

const leading_zeros: [u8; 3] = [ 10, 20, 10 ];
const trailing_zeros: [u8; 3] = [ 10, 10, 10 ];
const track_density: [u32; 3] = [ 210, 75, 210 ]; // bits per inch for each track
const track_sublen: [u8; 3] = [32, 48, 48];
const track_bitlen: [u8; 3] = [7, 5, 5];

//const SWIPE_SPEED: u32 = 9;
const SWIPE_SPEED: u32 = 5;

/// Describes the Buttons hardware management interface.
pub trait MagspoofHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Releases hardware if needed.
    fn teardown(&self);
    
    fn invert_coil_pole(&mut self);
    // Checks whether Button with specified type is pressed.
    //fn is_button_pressed(&self, button_type: ButtonType) -> bool;
    fn set_f2f_pole(&mut self, value_to_set: bool);
    fn clear_pins(&self);
}

pub struct MagspoofHardwareImpl<'a> {
    p: &'a Peripherals,
    f2f_pole: bool,
}

pub struct Magspoof<'a, T: MagspoofHardware, S: SysTickHardware> {
    hw: T,
    systick: &'a mut SysTick<S>,
}

impl<'a, T: MagspoofHardware, S: SysTickHardware> Magspoof<'a, T, S> {
    pub fn new(hw: T, systick: &'a mut SysTick<S>) -> Self {
        Magspoof { hw, systick}
    }

    /// Setups Buttons hardware.
    pub fn setup(&self) {
        self.hw.setup()
    }

    /// Tears down Buttons hardware.
    pub fn teardown(&self) {
        self.hw.teardown()
    }
    
    fn f2f_play_bit(&mut self, testbit: bool, period_us: u32) {
        let half_period: u32 = (period_us/2);
        self.hw.invert_coil_pole();
        self.systick.delay_us(half_period);
        if testbit {
            self.hw.invert_coil_pole();
        }   
        self.systick.delay_us(half_period); 
    }
    fn play_zeros(&mut self, n: u8, period_us: u32) {
        for x in 0..n {
            self.f2f_play_bit(false, period_us);
        }
    }
    fn play_ones(&mut self, n: u8, period_us: u32) {
        for x in 0..n {
            self.f2f_play_bit(true, period_us);
        }
    }
    fn play_byte(&mut self, mut byte: u8, n_bits: u8, period_us: u32, calc_lrc: bool, lrc: &mut u8) {
        let mut parity: bool = true;
        for i in 0..(n_bits-1) {
            let mut bit: bool = ((byte & 1) != 0) as bool;
            parity ^= bit;
            self.f2f_play_bit(bit, period_us ); //- 30); //TIMING SENSITIVE, subtract 30uS to compensate for delay within loop
            byte >>= 1;
            if calc_lrc {
                *lrc ^= (bit as u8) << i; 
            }
        }
        self.f2f_play_bit(parity, period_us);
    }
    pub fn play_track(&mut self, card_to_play: &card, track_num: u8) {
        let mut lrc: u8 = 0; 
        match track_num {
            1 => {
                //track = &card_to_play.track1;
                self.hw.set_f2f_pole(false);
                self.play_zeros(leading_zeros[0], card_to_play.period(0)); 
                for x in card_to_play.track1.iter() {
                    let ub: u8 = x - track_sublen[0]; 
                    self.play_byte(ub, track_bitlen[0], card_to_play.period(0), true, &mut lrc); 
                }
                self.play_byte(lrc, track_bitlen[0], card_to_play.period(0), false, &mut lrc);
            },
            2 => {
                self.hw.set_f2f_pole(false);
                //self.play_zeros(leading_zeros[1], card_to_play.period(1)); 
                self.play_zeros(10, card_to_play.period(1)); 
                for x in card_to_play.track2.iter() {
                    let ub: u8 = x - track_sublen[1]; 
                    self.play_byte(ub, track_bitlen[1], card_to_play.period(1), true, &mut lrc); 
                }
                self.play_byte(lrc, track_bitlen[1], card_to_play.period(1), false, &mut lrc);
                //self.play_ones(leading_zeros[1], card_to_play.period(1)); 
                self.play_ones(10, card_to_play.period(1)); 
                self.play_zeros(10, card_to_play.period(1)); 
            },
            3 => {
                self.play_zeros(leading_zeros[2], card_to_play.period(2)); 
                for x in card_to_play.track3.iter() {
                    let ub: u8 = x - track_sublen[2]; 
                    self.play_byte(ub, track_bitlen[2], card_to_play.period(2), true, &mut lrc); 
                }
                self.play_byte(lrc, track_bitlen[2], card_to_play.period(2), false, &mut lrc);
                self.play_ones(leading_zeros[2], card_to_play.period(2)); 
                self.play_zeros(leading_zeros[2], card_to_play.period(2)); 
            },
            _ => (), 
        }
    }
    pub fn play_card(&mut self, card_to_play: &card) {
        //set poles to low
        self.hw.clear_pins(); 
        self.hw.set_f2f_pole(false);
        //self.play_track(card_to_play, 2); 
        //self.play_track(card_to_play, 2); 
        for x in 1..3 {
            self.play_track(card_to_play, x);
        }
        self.hw.clear_pins(); 
    }
}

impl<'a> MagspoofHardware for MagspoofHardwareImpl<'a> {
    fn setup(&self) {
        self.p.GPIOA.moder.modify(|_, w| w.moder5().output().moder7().output());
        // -----------Buttons----------------
        // disable pull-ups on button GPIOs
        self.p
            .GPIOA
            .pupdr
            .modify(|_, w| w.pupdr5().pull_down().pupdr7().pull_down());
        self.p
            .GPIOA
            .otyper
            .modify(|_, w| w.ot5().push_pull().ot7().push_pull());
    }
    fn teardown(&self) {
        // Disable waker.
        //self.p
        //    .PWR
        //    .csr
        //    .modify(|_, w| w.ewup1().clear_bit().ewup4().clear_bit());
    }
    fn invert_coil_pole(&mut self) {
       self.f2f_pole ^= true;
       //let mut reg = &self.p.GPIOA.odr().write(); 
       if self.f2f_pole {
            self.p.GPIOA.odr.write(|w| w.odr5().set_bit().odr7().clear_bit()); 
       } else {
            self.p.GPIOA.odr.write(|w| w.odr5().clear_bit().odr7().set_bit()); 
       }
    }
    fn set_f2f_pole(&mut self, value_to_set: bool) {
        self.f2f_pole = value_to_set;    
    }
    fn clear_pins(&self) {
        self.p.GPIOA.odr.write(|w| w.odr5().clear_bit().odr7().clear_bit()); 
        //let mut reg = &self.hw.p.GPIOA.odr.write(); 
        //reg.odr5().low().odr7().low(); 
    }
}

pub fn create<'a>(p: &'a Peripherals,systick: &'a mut SysTick<impl SysTickHardware>,) -> Magspoof<'a, MagspoofHardwareImpl<'a>, impl SysTickHardware> {
    Magspoof::new(MagspoofHardwareImpl { p, f2f_pole:false }, systick)
}


//struct Context {
//    f2f_pole: bool,
//    track_period_us: [u32;3], 
//}

//#[derive(Copy, Clone)]

