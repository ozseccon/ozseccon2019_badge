use core::slice;
use core::mem;
use cortex_m::interrupt::CriticalSection;
use vcell::VolatileCell;
use cortex_m::interrupt;
//use cortex_m::interrupt::CriticalSection;
use stm32f0xx_hal::stm32::{USB, usb};
use usb_device::{Result, UsbError};
use usb_device::endpoint::EndpointType;

type EndpointBuffer = &'static mut [VolatileCell<u32>];

pub const NUM_ENDPOINTS: usize = 8;

#[repr(C)]
struct BufferDescriptor {
    pub addr_tx: VolatileCell<usize>,
    pub count_tx: VolatileCell<usize>,
    pub addr_rx: VolatileCell<usize>,
    pub count_rx: VolatileCell<usize>,
}

/// Arbitrates access to the endpoint-specific registers and packet buffer memory.
#[derive(Default)]
pub struct Endpoint {
    out_buf: Option<EndpointBuffer>,
    in_buf: Option<EndpointBuffer>,
    ep_type: Option<EndpointType>,
    index: u8,
}

pub fn calculate_count_rx(mut size: usize) -> Result<(usize, u16)> {
    if size <= 62 {
        // Buffer size is in units of 2 bytes, 0 = 0 bytes
        size = (size + 1) & !0x01;

        let size_bits = size >> 1;

        Ok((size, (size_bits << 10) as u16))
    } else if size <= 1024 {
        // Buffer size is in units of 32 bytes, 0 = 32 bytes
        size = (size + 31) & !0x1f;

        let size_bits = (size >> 5) - 1;

        Ok((size, (0x8000 | (size_bits << 10)) as u16))
    } else {
        Err(UsbError::EndpointMemoryOverflow)
    }
}

impl Endpoint {
    pub const MEM_START: usize = mem::size_of::<BufferDescriptor>() * NUM_ENDPOINTS;
    pub const MEM_SIZE: usize = 512;
    const MEM_ADDR: *mut VolatileCell<u32> = 0x4000_6000 as *mut VolatileCell<u32>;

    pub fn new(index: u8) -> Endpoint {
        Endpoint {
            out_buf: None,
            in_buf: None,
            ep_type: None,
            index,
        }
    }

    fn make_buf(addr: usize, size: usize)
        -> Option<&'static mut [VolatileCell<u32>]>
    {
        Some(
            unsafe {
                slice::from_raw_parts_mut(
                    Self::MEM_ADDR.offset((addr >> 1) as isize),
                    size >> 1)
            }
        )
    }

    pub fn ep_type(&self) -> Option<EndpointType> {
        self.ep_type
    }

    pub fn set_ep_type(&mut self, ep_type: EndpointType) {
        self.ep_type = Some(ep_type);
    }

    pub fn is_out_buf_set(&self) -> bool {
        self.out_buf.is_some()
    }

    pub fn set_out_buf(&mut self, addr: usize, size_and_bits: (usize, u16)) {
        self.out_buf = Self::make_buf(addr, size_and_bits.0);

        let descr = self.descr();
        descr.addr_rx.set(addr);
        descr.count_rx.set(size_and_bits.1 as usize);
    }

    pub fn is_in_buf_set(&self) -> bool {
        self.in_buf.is_some()
    }

    pub fn set_in_buf(&mut self, addr: usize, max_packet_size: usize) {
        self.in_buf = Self::make_buf(addr, max_packet_size);

        let descr = self.descr();
        descr.addr_tx.set(addr);
        descr.count_tx.set(0);
    }

    fn descr(&self) -> &'static BufferDescriptor {
        unsafe { &*(Self::MEM_ADDR as *const BufferDescriptor).offset(self.index as isize) }
    }

    fn reg(&self) -> &'static usb::EP0R {
        unsafe { &(*USB::ptr()).epr[self.index as usize] }
    }

    pub fn configure(&self, cs: &CriticalSection) {
        let ep_type = match self.ep_type {
            Some(t) => t,
            None => { return },
        };

        //self.reg().modify(|_, w|
        //    Self::clear_toggle_bits(w)
        //        .ctr_rx().clear_bit()
        //        // dtog_rx
        //        // stat_rx
        //        .ep_type().bits(ep_type.bits())
        //        .ep_kind().clear_bit()
        //        .ctr_tx().clear_bit()
        //        // dtog_rx
        //        // stat_tx
        //        .ea().bits(self.index));
//
//        //if self.out_buf.is_some() {
//        //    self.set_stat_rx(cs, EndpointStatus::Valid);
//        //}
//
        //if self.in_buf.is_some() {
        //    self.set_stat_tx(cs, EndpointStatus::Nak);
        //}
    }

    //pub fn write(&self, buf: &[u8]) -> Result<usize> {
    //    let guard = self.in_buf.as_ref().unwrap();

    //    let in_buf = match guard {
    //        Some(ref b) => b,
    //        None => { return Err(UsbError::WouldBlock); }
    //    };

    //    if buf.len() > in_buf.len() << 1 {
    //        return Err(UsbError::BufferOverflow);
    //    }

    //    let reg = self.reg();

    //    match reg.read().stat_tx().bits().into() {
    //        EndpointStatus::Valid | EndpointStatus::Disabled => return Err(UsbError::WouldBlock),
    //        _ => {},
    //    };

    //    self.write_mem(in_buf, buf);
    //    self.descr().count_tx.set(buf.len());

    //    interrupt::free(|cs| {
    //        self.set_stat_tx(cs, EndpointStatus::Valid);
    //    });

    //    Ok(buf.len())
    //}

    fn write_mem(&self, mem: &[VolatileCell<u32>], mut buf: &[u8]) {
        let mut addr = 0;

        while buf.len() >= 2 {
            mem[addr].set((buf[0] as u16 | ((buf[1] as u16) << 8)) as u32);
            addr += 1;

            buf = &buf[2..];
        }

        if buf.len() > 0 {
            mem[addr].set(buf[0] as u32);
        }
    }

    //pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
    //    let guard = self.out_buf.as_ref().unwrap();

    //    let out_buf = match guard {
    //        Some(ref b) => b,
    //        None => { return Err(UsbError::WouldBlock); }
    //    };

    //    let reg = self.reg();
    //    let reg_v = reg.read();

    //    let status: EndpointStatus = reg_v.stat_rx().bits().into();

    //    if status == EndpointStatus::Disabled || !reg_v.ctr_rx().bit_is_set() {
    //        return Err(UsbError::WouldBlock);
    //    }

    //    let count = self.descr().count_rx.get() & 0x3ff;
    //    if count > buf.len() {
    //        return Err(UsbError::BufferOverflow);
    //    }

    //    self.read_mem(out_buf, &mut buf[0..count]);

    //    interrupt::free(|cs| {
    //        self.clear_ctr_rx(cs);
    //        self.set_stat_rx(cs, EndpointStatus::Valid);
    //    });

    //    Ok(count)
    //}

    fn read_mem(&self, mem: &[VolatileCell<u32>], mut buf: &mut [u8]) {
        let mut addr = 0;

        while buf.len() >= 2 {
            let word = mem[addr].get();

            buf[0] = word as u8;
            buf[1] = (word >> 8) as u8;

            addr += 1;

            buf = &mut {buf}[2..];
        }

        if buf.len() > 0 {
            buf[0] = mem[addr].get() as u8;
        }
    }

    //pub fn read_reg(&self) -> usb::epr::R {
    //    self.reg().read()
    //}

    /*pub fn modify<F>(&self, _cs: CriticalSection, f: F)
        where for<'w> F: FnOnce(&usb::ep0r::R, &'w mut usb::ep0r::W) -> &'w mut usb::ep0r::W
    {
        self.reg().modify(f)
    }*/

    //fn clear_toggle_bits(w: &mut usb::epr::W) -> &mut usb::epr::W {
    //    unsafe {
    //        w
    //            .dtog_rx().clear_bit()
    //            .dtog_tx().clear_bit()
    //            .stat_rx().bits(0)
    //            .stat_tx().bits(0)
    //    }
    //}

    //pub fn clear_ctr_rx(&self, _cs: &CriticalSection) {
    //    self.reg().modify(|_, w| Self::clear_toggle_bits(w).ctr_rx().clear_bit());
    //}

    //pub fn clear_ctr_tx(&self, _cs: &CriticalSection) {
    //    self.reg().modify(|_, w| Self::clear_toggle_bits(w).ctr_tx().clear_bit());
    //}

    //pub fn set_stat_rx(&self, _cs: &CriticalSection, status: EndpointStatus) {
    //    self.reg().modify(|r, w| unsafe {
    //        Self::clear_toggle_bits(w)
    //            .stat_rx().bits(r.stat_rx().bits() ^ (status as u8))
    //    });
    //}

    //pub fn set_stat_tx(&self, _cs: &CriticalSection, status: EndpointStatus) {
    //    self.reg().modify(|r, w| unsafe {
    //        Self::clear_toggle_bits(w)
    //            .stat_tx().bits(r.stat_tx().bits() ^ (status as u8))
    //    });
    //}
}

/*#[repr(transparent)]
struct EndpointReg(usb::EP0R);

impl EndpointReg {
}*/

trait EndpointTypeExt {
    fn bits(self) -> u8;
}

impl EndpointTypeExt for EndpointType {
    fn bits(self) -> u8 {
        const BITS: [u8; 4] = [0b01, 0b10, 0b00, 0b11];
        return BITS[self as usize];
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Debug)]
#[allow(unused)]
pub enum EndpointStatus {
    Disabled = 0b00,
    Stall = 0b01,
    Nak = 0b10,
    Valid = 0b11,
}

impl From<u8> for EndpointStatus {
    fn from(v: u8) -> EndpointStatus {
        if v <= 0b11 {
            unsafe { mem::transmute(v) }
        } else {
            EndpointStatus::Disabled
        }
    }
}
