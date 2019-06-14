use crate::usb::systick::{SysTick, SysTickHardware};
//use stm32f0xx_hal::stm32::{Peripherals, GPIOA, GPIOB, RCC};
use stm32f0xx_hal::stm32::Peripherals;
/// Defines known button types.
pub enum ButtonType {
    SW1,
    SW2,
}

/// Defines type of the press (short, long, very long).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ButtonPressType {
    /// Button is not pressed.
    None,
    /// Button is keep pressed for less then a second.
    Short,
    /// Button is pressed for more than a second, but less than 2 seconds.
    Long,
}

impl ButtonPressType {
    pub fn is_none(self) -> bool {
        match self {
            ButtonPressType::None => true,
            _ => false,
        }
    }
}

/// Describes the Buttons hardware management interface.
pub trait ButtonsHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Releases hardware if needed.
    fn teardown(&self);

    /// Checks whether Button with specified type is pressed.
    fn is_button_pressed(&self, button_type: ButtonType) -> bool;
}

pub struct ButtonsHardwareImpl<'a> {
    p: &'a Peripherals,
}

pub struct Buttons<'a, T: ButtonsHardware, S: SysTickHardware> {
    hw: T,
    systick: &'a mut SysTick<S>,
}

impl<'a, T: ButtonsHardware, S: SysTickHardware> Buttons<'a, T, S> {
    pub fn new(hw: T, systick: &'a mut SysTick<S>) -> Self {
        Buttons { hw, systick }
    }

    /// Setups Buttons hardware.
    pub fn setup(&self) {
        self.hw.setup()
    }

    /// Tears down Buttons hardware.
    pub fn teardown(&self) {
        self.hw.teardown()
    }

    pub fn interrupt(&mut self) -> (ButtonPressType, ButtonPressType) {
        let mut sw1_state = if self.hw.is_button_pressed(ButtonType::SW1) {
            ButtonPressType::Short
        } else {
            ButtonPressType::None
        };

        let mut sw2_state = if self.hw.is_button_pressed(ButtonType::SW2) {
            ButtonPressType::Short
        } else {
            ButtonPressType::None
        };

        if sw1_state.is_none() && sw2_state.is_none() {
            return (sw1_state, sw2_state);
        }

        //hacky debounce...
        //if self.hw.is_button_pressed(ButtonType::SW1) {
        //    return ButtonType::SW1
        //} else if self.hw.is_button_pressed(ButtonType::SW2){
        //    return ButtonType::SW2
        //} else {
        //    return ButtonType::None
        //}
        //if self.hw.is_button_pressed(ButtonType::SW2) {
        //    return ButtonType::SW2
        //}
        for i in 1u8..8u8 {
            self.systick.delay_ms(250); //debounce

            if !self.hw.is_button_pressed(ButtonType::SW1)
                && !self.hw.is_button_pressed(ButtonType::SW2)
            {
                break;
            }

            let (new_state, works_for_none) = match i {
                0...4 => (ButtonPressType::Short, true),
                5...8 => (ButtonPressType::Long, false),
                _ => break,
            };

            if self.hw.is_button_pressed(ButtonType::SW1)
                && (!sw1_state.is_none() || works_for_none)
            {
                sw1_state = new_state;
            }

            if self.hw.is_button_pressed(ButtonType::SW2)
                && (!sw2_state.is_none() || works_for_none)
            {
                sw2_state = new_state;
            }
        }
        (sw1_state, sw2_state)
    }
}

impl<'a> ButtonsHardware for ButtonsHardwareImpl<'a> {
    fn setup(&self) {
        self.p.GPIOA.moder.modify(|_, w| w.moder0().input().moder4().input());
        // -----------Buttons----------------
        // enable pull-ups on button GPIOs
        self.p
            .GPIOA
            .pupdr
            .modify(|_, w| w.pupdr0().pull_up().pupdr4().pull_up());
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

        // Enable wakers.
        //self.p
        //    .PWR
        //    .csr
        //    .modify(|_, w| w.ewup1().set_bit().ewup4().set_bit());
    }

    fn teardown(&self) {
        // Disable waker.
        //self.p
        //    .PWR
        //    .csr
        //    .modify(|_, w| w.ewup1().clear_bit().ewup4().clear_bit());
    }

    fn is_button_pressed(&self, button_type: ButtonType) -> bool {
        let reg = &self.p.GPIOA.idr.read();
        match button_type {
            ButtonType::SW1 => self.p.GPIOA.idr.read().idr0().bit_is_clear(),
            ButtonType::SW2 => self.p.GPIOA.idr.read().idr4().bit_is_clear(),
            //ButtonType::None => false, 
        }
    }
}

pub fn has_pending_interrupt(p: &Peripherals) -> bool {
    let reg = p.EXTI.pr.read();
    reg.pif0().bit_is_set() || reg.pif4().bit_is_set()
}

pub fn clear_pending_interrupt(p: &Peripherals) {
    // Clear exti line 0 and 4 flags.
    p.EXTI.pr.modify(|_, w| w.pif0().set_bit().pif4().set_bit());
}

pub fn create<'a>(
    p: &'a Peripherals,
    systick: &'a mut SysTick<impl SysTickHardware>,
) -> Buttons<'a, ButtonsHardwareImpl<'a>, impl SysTickHardware> {
    Buttons::new(ButtonsHardwareImpl { p }, systick)
}
