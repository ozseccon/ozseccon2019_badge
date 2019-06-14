use crate::{SysTick, SysTickHardware};
use cortex_m::interrupt::free;
use stm32f0xx_hal as hal;
use crate::hal::{stm32::Peripherals,gpio::gpioa,gpio::gpiob,gpio::Output, gpio::PushPull, stm32::GPIOA, stm32::GPIOB, gpio::GpioExt, rcc::RccExt, stm32::RCC};
//use stm32f0::stm32f0x2::{Peripherals,gpioa,gpiob};

/// Describes the Buttons hardware management interface.
pub trait LedsHardware {
    fn setup(&self);
    fn setup_pwm(&self); //swap leds to PWM mode 
    fn teardown(&self);
    
    fn turnonLED1(&self); 
    fn turnoffLED1(&self); 
    fn toggleLED1(&self); 
    
    fn turnonLED2(&self); 
    fn turnoffLED2(&self); 
    fn toggleLED2(&self); 
    
    fn turnonLED3(&self); 
    fn turnoffLED3(&self); 
    fn toggleLED3(&self); 

    fn turnonLED4(&self); 
    fn turnoffLED4(&self); 
    fn toggleLED4(&self); 
    
    fn turnonLED5(&self); 
    fn turnoffLED5(&self); 
    fn toggleLED5(&self); 
    
    fn turnonLED6(&self); 
    fn turnoffLED6(&self); 
    fn toggleLED6(&self); 

    fn disable_LED4(&mut self);
    fn enable_LED4(&mut self);
    fn get_duty_LED4(&self) -> u16;
    fn get_max_duty_LED4(&self) -> u16;
    fn set_duty_LED4(&mut self, duty: u16); 
 
}

pub struct Leds <'a, T: LedsHardware, S: SysTickHardware> {
    hw: T,
    systick: &'a mut SysTick<S>,
}

impl<'a, T: LedsHardware, S: SysTickHardware> Leds<'a, T, S> {
    pub fn new(hw: T, systick: &'a mut SysTick<S>) -> Self {
        Leds { hw, systick,}
    }

    /// Setups Buttons hardware.
    pub fn setup(&self) {
        self.hw.setup();
    }
    
    pub fn setup_pwm(&self) {
        self.hw.setup_pwm();
    }
    /// Tears down Buttons hardware.
    pub fn teardown(&self) {
        self.hw.teardown();
    }

    pub fn turnonLED1(&self) {
        self.hw.turnonLED1(); 
    }

    pub fn turnoffLED1(&self) {
        self.hw.turnoffLED1(); 
    }
    
    pub fn toggleLED1(&self) {
        self.hw.toggleLED1(); 
    }

    pub fn turnonLED2(&self) {
        self.hw.turnonLED2(); 
    }

    pub fn turnoffLED2(&self) {
        self.hw.turnoffLED2(); 
    }
    pub fn toggleLED2(&self) {
        self.hw.toggleLED2();
    }

    pub fn turnonLED3(&self) {
        self.hw.turnonLED3(); 
    }

    pub fn turnoffLED3(&self) {
        self.hw.turnoffLED3(); 
    }
    pub fn toggleLED3(&self) {
        self.hw.toggleLED3();
    }
    
    pub fn turnonLED4(&self) {
        self.hw.turnonLED4(); 
    }

    pub fn turnoffLED4(&self) {
        self.hw.turnoffLED4(); 
    }
    pub fn toggleLED4(&self) {
        self.hw.toggleLED4(); 
    }

    pub fn turnonLED5(&self) {
        self.hw.turnonLED5(); 
    }

    pub fn turnoffLED5(&self) {
        self.hw.turnoffLED5(); 
    }
    pub fn toggleLED5(&self) {
        self.hw.toggleLED5(); 
    }

    pub fn turnonLED6(&self) {
        self.hw.turnonLED6(); 
    }

    pub fn turnoffLED6(&self) {
        self.hw.turnoffLED6(); 
    }
    pub fn toggleLED6(&self) {
        self.hw.toggleLED6(); 
    }

    pub fn disable_LED4(& mut self) {
        self.hw.disable_LED4(); 
    }
    
    pub fn enable_LED4(& mut self) {
        self.hw.enable_LED4(); 
    } 
    pub fn get_duty_LED4(&self) -> u16 {
        self.hw.get_duty_LED4() 
    }
    pub fn get_max_duty_LED4(&self) -> u16 {
        self.hw.get_max_duty_LED4() 
    }
    pub fn set_duty_LED4(& mut self, duty: u16) {
        self.hw.set_duty_LED4(duty);
    } 
}

pub struct LedsHardwareImpl<'a> {
    p: &'a Peripherals,
}

impl<'a> LedsHardware for LedsHardwareImpl<'a> {
    fn setup(&self) {
        //config GPIOs 
        self.p.GPIOA.moder.modify(|_, w| {
                w.moder8().output()         
                 .moder9().output()         
                 .moder10().output()         
                 .moder15().output()         
            });   
        self.p.GPIOB.moder.modify(|_, w| {
                w.moder0().output()         
                 .moder1().output()         
            });   
    }
    
    fn setup_pwm(&self){ //swap leds to PWM mode 
        //LED4 - TIM1_CH1 - AF2
        //LED5 - TIM1_CH2 - AF2
        //LED6 - TIM1_CH3 - AF2
        //LED1 - TIM2_CH1_ETR - AF2
        //LED2 - TIM3_CH3 - AF1
        //LED3 - TIM3_CH4 - AF1
        //config GPIOs 
        //enable pupds 
        self.p.GPIOA.pupdr.modify(|_, w| {
                w.pupdr8().pull_up()         
                 .pupdr9().pull_up()         
                 .pupdr10().pull_up()         
                 .pupdr15().pull_up()         
            });   
        self.p.GPIOB.pupdr.modify(|_, w| {
                w.pupdr0().pull_up()         
                 .pupdr1().pull_up()         
            });   
        //leds into alternate mode 
        self.p.GPIOA.moder.modify(|_, w| {
                w.moder8().alternate()         
                 .moder9().alternate()         
                 .moder10().alternate()         
                 .moder15().alternate()         
            });   
        self.p.GPIOB.moder.modify(|_, w| {
                w.moder0().alternate()         
                 .moder1().alternate()         
            });   
        //set alternate functions 
        self.p.GPIOA.afrh.modify(|_, w| {
                w.afrh8().af2()         
                 .afrh9().af2()         
                 .afrh10().af2()         
                 .afrh15().af2()         
            });   
        self.p.GPIOB.afrl.modify(|_, w| {
                w.afrl0().af1()         
                 .afrl1().af1()         
            });   
        
        //enable timer 1 
        self.p.RCC.apb2enr.modify(|_, w| w.tim1en().set_bit());
        self.p.RCC.apb2rstr.modify(|_, w| w.tim1rst().reset());
        //self.RCC.apb2rstr.modify(|_, w| w.tim1rst().set_bit());
        //self.RCC.apb2rstr.modify(|_, w| w.tim1rst().clear_bit());
        self.p.TIM1.ccmr1_output.modify(|_, w| unsafe { w.oc1pe().set_bit().oc1m().bits(6) });
        self.p.TIM1.ccmr1_output.modify(|_, w| unsafe { w.oc2pe().set_bit().oc2m().bits(6) });
        self.p.TIM1.ccmr2_output.modify(|_, w| unsafe { w.oc3pe().set_bit().oc3m().bits(6) });
       
        //enable timer 2  
        self.p.RCC.apb1enr.modify(|_, w| w.tim2en().set_bit());
        self.p.RCC.apb1rstr.modify(|_, w| w.tim2rst().reset());
        //self.RCC.apb2rstr.modify(|_, w| w.tim1rst().set_bit());
        //self.RCC.apb2rstr.modify(|_, w| w.tim1rst().clear_bit());
        self.p.TIM2.ccmr1_output.modify(|_, w| unsafe { w.oc1pe().set_bit().oc1m().bits(6) });

        //enable timer 3
        self.p.RCC.apb1enr.modify(|_, w| w.tim3en().set_bit());
        self.p.RCC.apb1rstr.modify(|_, w| w.tim3rst().reset());
        self.p.TIM3.ccmr2_output.modify(|_, w| unsafe { w.oc3pe().set_bit().oc3m().bits(6) });
        self.p.TIM3.ccmr2_output.modify(|_, w| unsafe { w.oc3pe().set_bit().oc3m().bits(6) });

        let clk: u32 = 8_000_000; //8MHz
        let freq: u32 = 10_000; //10k 
        let ticks = clk / freq;
        let psc = ticks / (1 << 16);
        self.p.TIM1.psc.write(|w| unsafe { w.psc().bits(psc as u16) });
        let psc = ticks / (1 << 16);
        self.p.TIM2.psc.write(|w| unsafe { w.psc().bits(psc as u16) });
        self.p.TIM3.psc.write(|w| unsafe { w.psc().bits(psc as u16) });
        let arr = ticks / (psc + 1);
        //hprintln!("clk: {} freq: {} ticks: {} psc: {} arr{}:", clk, freq, ticks, psc, arr).unwrap();
        self.p.TIM1.arr.write(|w| unsafe{w.arr().bits(arr as u16)});
        self.p.TIM2.arr.write(|w| unsafe{w.arr().bits(arr as u32)});
        self.p.TIM3.arr.write(|w| unsafe{w.arr().bits(arr as u32)});
        self.p.TIM1.cr1.write(|w| unsafe {
            w.cms()
                .bits(0b00) //edge aligned mode
                .dir()
                .clear_bit() //upcounter
                .opm()
                .clear_bit() //one pulse mode
                .cen()
                .set_bit() //counter enable
        });
        self.p.TIM1.egr.write(|w| unsafe { w.ug().set_bit()});
        
        self.p.TIM2.cr1.write(|w| unsafe {
            w.cms()
                .bits(0b00) //edge aligned mode
                .dir()
                .clear_bit() //upcounter
                .opm()
                .clear_bit() //one pulse mode
                .cen()
                .set_bit() //counter enable
        });
        self.p.TIM3.cr1.write(|w| unsafe {
            w.cms()
                .bits(0b00) //edge aligned mode
                .dir()
                .clear_bit() //upcounter
                .opm()
                .clear_bit() //one pulse mode
                .cen()
                .set_bit() //counter enable
        });
 
    }
    fn teardown(&self) {
        
    }
    
    fn turnonLED1(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr15().set_bit());
    }
    fn turnoffLED1(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr15().clear_bit());
    }
    
    fn toggleLED1(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr15().bit(self.p.GPIOA.odr.read().odr15().bit_is_set() ^ true));
    }

    fn turnonLED2(&self) {
        self.p
            .GPIOB
            .odr
            .modify(|_, w| w.odr0().set_bit());
    }
    fn turnoffLED2(&self) {
        self.p
            .GPIOB
            .odr
            .modify(|_, w| w.odr0().clear_bit());
    }
    
    fn toggleLED2(&self) {
        self.p
            .GPIOB
            .odr
            .modify(|_, w| w.odr0().bit(self.p.GPIOB.idr.read().idr0().bit_is_set() ^ true));
    }

    fn turnonLED3(&self) {
        self.p
            .GPIOB
            .odr
            .modify(|_, w| w.odr1().set_bit());
    }
    fn turnoffLED3(&self) {
        self.p
            .GPIOB
            .odr
            .modify(|_, w| w.odr1().clear_bit());
    }
    
    fn toggleLED3(&self) {
        self.p
            .GPIOB
            .odr
            .modify(|_, w| w.odr1().bit(self.p.GPIOB.idr.read().idr1().bit_is_set() ^ true));
    }

    fn turnonLED4(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr8().set_bit());
    }
    fn turnoffLED4(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr8().clear_bit());
    }
    
    fn toggleLED4(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr8().bit(self.p.GPIOA.idr.read().idr8().bit_is_set() ^ true));
    }

    fn turnonLED5(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr9().set_bit());
    }
    fn turnoffLED5(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr9().clear_bit());
    }
    
    fn toggleLED5(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr9().bit(self.p.GPIOA.idr.read().idr9().bit_is_set() ^ true));
    }

    fn turnonLED6(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr10().set_bit());
    }
    fn turnoffLED6(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr10().clear_bit());
    }
    
    fn toggleLED6(&self) {
        self.p
            .GPIOA
            .odr
            .modify(|_, w| w.odr10().bit(self.p.GPIOA.idr.read().idr10().bit_is_set() ^ true));
    }
   
    //PWM funcs 
    fn disable_LED4(&mut self) {
        self.p.TIM1.ccer.write(|w| w.cc1e().clear_bit());
    }

    fn enable_LED4(&mut self) {
        self.p.TIM1.ccer.write(|w| w.cc1e().set_bit());
    }

    fn get_duty_LED4(&self) -> u16 {
        self.p.TIM1.ccr1.read().ccr1().bits()
    }

    fn get_max_duty_LED4(&self) -> u16 {
        self.p.TIM1.arr.read().arr().bits()
    }

    fn set_duty_LED4(&mut self, duty: u16) {
        self.p.TIM1.ccr1.write(|w| unsafe{w.ccr1().bits(duty)});
        self.p.TIM1.bdtr.write(|w| w.bke().clear_bit().moe().set_bit());
        //self.p.TIM1.egr.write(|w| w.ug().set_bit());
    }
}

pub fn create<'a>(
    p: &'a Peripherals,
    systick: &'a mut SysTick<impl SysTickHardware>,
) -> Leds<'a, LedsHardwareImpl<'a>, impl SysTickHardware> {
    Leds::new(LedsHardwareImpl { p }, systick)
}

