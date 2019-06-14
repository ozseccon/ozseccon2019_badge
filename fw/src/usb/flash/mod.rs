//use stm32f0xx_hal as hal;
//use hal::Peripherals;
use stm32f0xx_hal::stm32::Peripherals;


//pub mod storage;
//mod storage_page;
//mod storage_page_status;
//pub mod storage_slot;

//use self::{storage::Storage, storage_page::StoragePage, storage_slot::StorageSlot};

use crate::usb::magspoof::card;

use cortex_m::asm::bkpt;

use cortex_m_semihosting::hprintln;

/// Describes the Flash hardware management interface.
pub trait FlashHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Releases hardware if needed.
    fn teardown(&self);

    /// Returns addresses of the flash memory pages.
    fn page_addresses(&self) -> [usize; 2];

    /// Erases page using specified address.
    fn erase_page(&self, page_address: usize);

    /// Makes peripheral to enter `write` mode.
    fn enable_write_mode(&self);

    /// Makes peripheral to exit `write` mode.
    fn disable_write_mode(&self);
}

pub struct Flash<T: FlashHardware> {
    hw: T,
    //storage: Storage,
}

impl<T: FlashHardware> Flash<T> {
    pub fn new(hw: T) -> Self {
        let page_addresses = hw.page_addresses();
        Flash {
            hw,
            //storage: Storage {
            //    pages: [
            //        StoragePage {
            //            address: page_addresses[0],
            //            size: 1024,
            //        },
            //        StoragePage {
            //            address: page_addresses[1],
            //            size: 1024,
            //        },
            //    ],
            //},
        }
    }

    /// Setups Flash hardware.
    pub fn setup(&self) {
        self.hw.setup()
    }

    /// Tears down Flash hardware.
    pub fn teardown(&self) {
        self.hw.teardown()
    }

    /// Reads a card from a specific memory slot.
    //pub fn read(&self, slot: CardSlot) -> card {
    //    //self.storage.read(slot)
    //}

    /// Writes a card to a specific memory slot.
    //pub fn write(&self, slot: u8, cardvalue: card) -> Result<(), ()> {
    pub fn write(&self, slot: usize, cardvalue: card) { //-> Result<(), ()> {
        self.hw.enable_write_mode();
        //serialize the card 
        let serialised_data = cardvalue.serialize();
        //write the card data
        for i in (0..(serialised_data.len()-1)).step_by(2) {
            let value_to_write: u16 = (((serialised_data[i+1] as u16) << 8) | (serialised_data[i] as u16)) as u16 ;
            unsafe { core::ptr::write(((self.hw.page_addresses()[0] + i + (slot * 256)) as *mut u16), value_to_write);}   
        }        
        self.hw.disable_write_mode();
    }
    //write a chunk direct to flash 
    pub fn write_chunk(&self, slot: usize, chunk_num: usize, chunk: [u8;60]) { //-> Result() { //-> Result<(), ()> {
        self.hw.enable_write_mode();
        //write the card data
        for i in (0..60).step_by(2) {
            let value_to_write: u16 = (((chunk[i+1] as u16) << 8) | (chunk[i] as u16)) as u16 ;
            unsafe { core::ptr::write(((self.hw.page_addresses()[0] + (chunk_num * 60) + i + (slot * 256)) as *mut u16), value_to_write);}   
        }
        self.hw.disable_write_mode();
    }
    //read card from storage 
    pub fn read(&self, slot: usize) -> [u8;256] { //-> Result<(), ()> {
        //self.hw.enable_write_mode();
        //serialize the card 
        let mut raw_track_data = [0;256]; 
        for i in (0..raw_track_data.len()-1).step_by(2) {
            //data out of flash
            let raw_data: u16 = unsafe { core::ptr::read((self.hw.page_addresses()[0] + i + (slot * 256)) as *mut u16) };
            raw_track_data[i] = (raw_data & 0xff) as u8;
            raw_track_data[i+1] = ((raw_data >> 8) & 0xff) as u8;
        }
        raw_track_data 
    }
    /// Erases all storage pages.
    pub fn erase_all(&self) {
        self.hw.erase_page(self.hw.page_addresses()[0]);
//for page in self.storage.pages.iter() {
        //    self.hw.erase_page(page.address);
        //}
    }
}

/// Sector 7, page 30 and 31 of STM32F04x flash memory.
const PAGE_ADDRESSES: [usize; 2] = [0x0800_7800, 0x0800_7C00];

pub struct FlashHardwareImpl<'a> {
    p: &'a Peripherals,
}

impl<'a> FlashHardwareImpl<'a> {
    /// Disables or enables Flash write protection.
    fn toggle_write_protection(&self, enable_write_protection: bool) {
        let is_protected = self.p.FLASH.cr.read().lock().bit_is_set();
        if enable_write_protection && !is_protected {
            self.p.FLASH.cr.write(|w| w.lock().set_bit());
        } else if is_protected {
            self.p.FLASH.keyr.write(|w| unsafe { w.bits(0x4567_0123) });
            self.p.FLASH.keyr.write(|w| unsafe { w.bits(0xCDEF_89AB) });
        }
    }

    fn busy_wait_until_ready(&self) {
        // Wait until Flash is not busy.
        while self.p.FLASH.sr.read().bsy().bit_is_set() {}
    }
}

impl<'a> FlashHardware for FlashHardwareImpl<'a> {
    fn setup(&self) {}

    fn teardown(&self) {}

    fn page_addresses(&self) -> [usize; 2] {
        PAGE_ADDRESSES
    }

    fn erase_page(&self, page_address: usize) {
        self.busy_wait_until_ready();
        self.toggle_write_protection(false);

        self.p.FLASH.cr.modify(|_, w| w.per().set_bit());
        self.p
            .FLASH
            .ar
            .write(|w| unsafe { w.bits(page_address as u32) });
        self.p.FLASH.cr.modify(|_, w| w.strt().set_bit());

        self.busy_wait_until_ready();

        self.p.FLASH.cr.modify(|_, w| w.per().clear_bit());

        self.toggle_write_protection(true);
    }

    fn enable_write_mode(&self) {
        self.busy_wait_until_ready();

        self.toggle_write_protection(false);

        self.p.FLASH.cr.modify(|_, w| w.pg().set_bit());
    }

    fn disable_write_mode(&self) {
        self.busy_wait_until_ready();

        self.p.FLASH.cr.modify(|_, w| w.pg().clear_bit());

        self.toggle_write_protection(true);
    }
}

pub fn create(p: &Peripherals) -> Flash<FlashHardwareImpl> {
    Flash::new(FlashHardwareImpl { p })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::MockData;
    use core::cell::RefCell;

    // Size of the page in bytes (u8).
    const PAGE_SIZE: usize = 1024;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    enum Call {
        Setup,
        Teardown,
        EnableWriteMode,
        DisableWriteMode,
        ErasePage(usize),
    }

    struct FlashHardwareMock<'a, 'b: 'a> {
        data: RefCell<&'a mut MockData<'b, Call>>,
        page_addresses: [usize; 2],
    }

    impl<'a, 'b: 'a> FlashHardware for FlashHardwareMock<'a, 'b> {
        fn setup(&self) {
            self.data.borrow_mut().calls.log_call(Call::Setup);
        }

        fn teardown(&self) {
            self.data.borrow_mut().calls.log_call(Call::Teardown);
        }

        fn page_addresses(&self) -> [usize; 2] {
            self.page_addresses
        }

        fn erase_page(&self, page_address: usize) {
            self.data
                .borrow_mut()
                .calls
                .log_call(Call::ErasePage(page_address));
        }

        fn enable_write_mode(&self) {
            self.data.borrow_mut().calls.log_call(Call::EnableWriteMode);
        }

        fn disable_write_mode(&self) {
            self.data
                .borrow_mut()
                .calls
                .log_call(Call::DisableWriteMode);
        }
    }

    fn create_flash<'a, 'b: 'a>(
        mock_data: &'a mut MockData<'b, Call>,
        page_addresses: [usize; 2],
    ) -> Flash<FlashHardwareMock<'a, 'b>> {
        Flash::new(FlashHardwareMock {
            data: RefCell::new(mock_data),
            page_addresses,
        })
    }

    #[test]
    fn setup() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        create_flash(
            &mut mock_data,
            [&page1 as *const _ as usize, &page2 as *const _ as usize],
        )
        .setup();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Setup)])
    }

    #[test]
    fn teardown() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        create_flash(
            &mut mock_data,
            [&page1 as *const _ as usize, &page2 as *const _ as usize],
        )
        .teardown();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Teardown)])
    }

    #[test]
    fn read() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let mut page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let flash = create_flash(
            &mut mock_data,
            [&page1 as *const _ as usize, &page2 as *const _ as usize],
        );

        assert_eq!(flash.read(StorageSlot::One), None);
        assert_eq!(flash.read(StorageSlot::Two), None);

        page1[2] = 0x1f0f;

        assert_eq!(flash.read(StorageSlot::One), Some(0x0f));
        assert_eq!(flash.read(StorageSlot::Two), None);

        page1[3] = 0x2f01;

        assert_eq!(flash.read(StorageSlot::One), Some(0x0f));
        assert_eq!(flash.read(StorageSlot::Two), Some(0x01));
    }

    #[test]
    fn write_when_page_has_enough_space() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        {
            let flash = create_flash(
                &mut mock_data,
                [&page1 as *const _ as usize, &page2 as *const _ as usize],
            );

            assert_eq!(flash.write(StorageSlot::One, 10).is_ok(), true);
        }

        assert_eq!(page1[..4], [0x0fff, 0xffff, 0x1f0a, 0xffff]);
        assert_eq!(
            mock_data.calls.logs(),
            [Some(Call::EnableWriteMode), Some(Call::DisableWriteMode)]
        )
    }

    #[test]
    fn write_when_page_is_full() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let mut page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        {
            let flash = create_flash(
                &mut mock_data,
                [&page1 as *const _ as usize, &page2 as *const _ as usize],
            );

            // Imitate fully populated page.
            page1[0] = 0x0fff;
            page1[1] = 0x0001;
            for i in 2..(PAGE_SIZE / 2) {
                page1[i] = 0x1f12;
            }

            assert_eq!(flash.write(StorageSlot::Two, 0x17).is_ok(), true);
        }

        assert_eq!(page2[..5], [0x0fff, 0xffff, 0x1f12, 0x2f17, 0xffff]);
        assert_eq!(
            mock_data.calls.logs(),
            [
                Some(Call::EnableWriteMode),
                Some(Call::DisableWriteMode),
                Some(Call::ErasePage(&page2 as *const _ as usize)),
                Some(Call::EnableWriteMode),
                Some(Call::DisableWriteMode)
            ]
        )
    }

    #[test]
    fn erase_all() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        create_flash(
            &mut mock_data,
            [&page1 as *const _ as usize, &page2 as *const _ as usize],
        )
        .erase_all();

        assert_eq!(
            mock_data.calls.logs(),
            [
                Some(Call::ErasePage(&page1 as *const _ as usize)),
                Some(Call::ErasePage(&page2 as *const _ as usize))
            ]
        )
    }
}
