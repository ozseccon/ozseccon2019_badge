extern crate vcell;

pub struct CardStructure {
    p: &'static mut card
}

#[repr(C)]
pub struct card<'a> {
    pub track1: &'a [u8], //String<U79>,
    //track1: [u8; 79], //String<U79>,
    pub track2: &'a [u8], //String<U40>,
    //track2: [u8; 40], //String<U40>,
    pub track3: &'a [u8], //String<U107>,    
    //track3: [u8; 107], //String<U107>,    
}

impl CardStructure {
    pub fn new() -> CardStructure {
        SystemTimer {
            p: unsafe { &mut *(0xE000_E010 as *mut CardStructure) }
        }
    }
}
 

