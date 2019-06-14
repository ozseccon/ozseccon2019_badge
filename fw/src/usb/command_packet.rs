//use crate::flash::storage_slot::StorageSlot;
//use crate::time::Time;
use cortex_m_semihosting::hprintln;

const COMMAND_BYTE_SEQUENCE_LENGTH: usize = 64; //max length of a track 3 is cmd+slot+tracktowrite+107 chars long == 111
pub type CommandByteSequence = [u8; COMMAND_BYTE_SEQUENCE_LENGTH];

//#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[derive(Copy, Clone)]
pub enum CommandPacket {
    TurnOnLED1, 
    TurnOffLED1, 
    ToggleLED1, 
    TurnOnLED2, 
    TurnOffLED2, 
    ToggleLED2, 
    TurnOnLED3, 
    TurnOffLED3, 
    ToggleLED3, 
    TurnOnLED4, 
    TurnOffLED4, 
    ToggleLED4, 
    TurnOnLED5, 
    TurnOffLED5, 
    ToggleLED5, 
    TurnOnLED6, 
    TurnOffLED6, 
    ToggleLED6, 
    EraseAll, 
    TrackLoad(u8,u8,[u8;60]), 
    ReadCard(u8),
    //TurnOnRightLED(bool,bool,bool), 
    //Beep(u8),
    //AlarmGet,
    //AlarmSet(Time),
    //FlashRead(StorageSlot),
    //FlashWrite(StorageSlot, u8),
    //FlashEraseAll,
    //Reset,
    Unknown,
}

impl CommandPacket {
    pub fn to_bytes(self) -> CommandByteSequence {
        match self {
            CommandPacket::TurnOnLED1 => [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOffLED1 => [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::ToggleLED1 => [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOnLED2 => [2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOffLED2 => [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::ToggleLED2 => [2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOnLED3 => [3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOffLED3 => [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::ToggleLED3 => [3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOnLED4 => [4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOffLED4 => [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::ToggleLED4 => [4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOnLED5 => [5, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOffLED5 => [5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::ToggleLED5 => [5, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOnLED6 => [6, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TurnOffLED6 => [6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::ToggleLED6 => [6, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::EraseAll => [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            CommandPacket::TrackLoad(slot, num_chunk, track_chunk) => [8, slot, num_chunk, 
track_chunk[0],track_chunk[1],track_chunk[2],track_chunk[3],track_chunk[4],track_chunk[5],track_chunk[6],track_chunk[7],track_chunk[8],track_chunk[9],track_chunk[10],track_chunk[11],track_chunk[12],track_chunk[13],track_chunk[14],track_chunk[15],track_chunk[16],track_chunk[17],track_chunk[18],track_chunk[19],track_chunk[20],track_chunk[21],track_chunk[22],track_chunk[23],track_chunk[24],track_chunk[25],track_chunk[26],track_chunk[27],track_chunk[28],track_chunk[29],track_chunk[30],track_chunk[31],track_chunk[32],track_chunk[33],track_chunk[34],track_chunk[35],track_chunk[36],track_chunk[37],track_chunk[38],track_chunk[39],track_chunk[40],track_chunk[41],track_chunk[42],track_chunk[43],track_chunk[44],track_chunk[45],track_chunk[46],track_chunk[47],track_chunk[48],track_chunk[49],track_chunk[50],track_chunk[51],track_chunk[52],track_chunk[53],track_chunk[54],track_chunk[55],track_chunk[56],track_chunk[57],track_chunk[58],track_chunk[59], 0],
            CommandPacket::ReadCard(slot) => [9, slot, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            //CommandPacket::AlarmSet(time) => [2, 0, time.hours, time.minutes, time.seconds, 0],
            //CommandPacket::AlarmGet => [3, 0, 0, 0, 0, 0],
            //CommandPacket::Reset => [4, 0, 0, 0, 0, 0],
            //CommandPacket::FlashRead(slot) => [5, 0, slot.into(), 0, 0, 0],
            //CommandPacket::FlashWrite(slot, value) => [6, 0, slot.into(), value, 0, 0],
            //CommandPacket::FlashEraseAll => [7, 0, 0, 0, 0, 0],
            CommandPacket::Unknown => [0; COMMAND_BYTE_SEQUENCE_LENGTH],
        }
    }
}

impl From<(u8, u8, u8, [u8;60], u8)> for CommandPacket {
    fn from((header, data_1, data_2, trackdata, last_bit): (u8, u8, u8, [u8;60], u8)) -> Self {
        //let command_type_byte = header & 0xff;
        let command_type_byte = header;
        let data_type_byte = data_1; 
        //let data2 = data_2; 
        //hprintln!("command type byte: {:x} data1: {:x} data2: {:x} trackdata: {:?}", command_type_byte, data_1, data_2, trackdata); 
        match command_type_byte {
            1 => {match data_type_byte {
                        0 => CommandPacket::TurnOffLED1,
                        1 => CommandPacket::TurnOnLED1,
                        2 => CommandPacket::ToggleLED1,
                        _ => CommandPacket::Unknown,
                    }
                 },
            2 => {match data_type_byte {
                        0 => CommandPacket::TurnOffLED2,
                        1 => CommandPacket::TurnOnLED2,
                        2 => CommandPacket::ToggleLED2,
                        _ => CommandPacket::Unknown,
                    }
                 },
            3 => {match data_type_byte {
                        0 => CommandPacket::TurnOffLED3,
                        1 => CommandPacket::TurnOnLED3,
                        2 => CommandPacket::ToggleLED3,
                        _ => CommandPacket::Unknown,
                    }
                 },
            4 => {match data_type_byte {
                        0 => CommandPacket::TurnOffLED4,
                        1 => CommandPacket::TurnOnLED4,
                        2 => CommandPacket::ToggleLED4,
                        _ => CommandPacket::Unknown,
                    }
                 },
            5 => {match data_type_byte {
                        0 => CommandPacket::TurnOffLED5,
                        1 => CommandPacket::TurnOnLED5,
                        2 => CommandPacket::ToggleLED5,
                        _ => CommandPacket::Unknown,
                    }
                 },
            6 => {match data_type_byte {
                        0 => CommandPacket::TurnOffLED6,
                        1 => CommandPacket::TurnOnLED6,
                        2 => CommandPacket::ToggleLED6,
                        _ => CommandPacket::Unknown,
                    }
                 },
            7 => CommandPacket::EraseAll,
            8 => CommandPacket::TrackLoad(data_type_byte, data_2, trackdata),
            9 => CommandPacket::ReadCard(data_type_byte),
            _ => CommandPacket::Unknown,
        }
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn default_command_bytes() {
//        assert_eq!(
//            CommandByteSequence::default(),
//            [0; COMMAND_BYTE_SEQUENCE_LENGTH]
//        );
//    }
//
//    #[test]
//    fn beep_command() {
//        assert_eq!(CommandPacket::from((1, 1, 0)), CommandPacket::Beep(1));
//        assert_eq!(CommandPacket::from((1, 15, 0)), CommandPacket::Beep(15));
//
//        assert_eq!(CommandPacket::Beep(1).to_bytes(), [1, 0, 1, 0, 0, 0]);
//        assert_eq!(CommandPacket::Beep(15).to_bytes(), [1, 0, 15, 0, 0, 0]);
//    }
//
//    #[test]
//    fn alarm_set_command() {
//        assert_eq!(
//            CommandPacket::from((2, 0x2112, 17)),
//            CommandPacket::AlarmSet(Time {
//                hours: 18,
//                minutes: 33,
//                seconds: 17,
//            })
//        );
//
//        assert_eq!(
//            CommandPacket::from((2, 0x1221, 1)),
//            CommandPacket::AlarmSet(Time {
//                hours: 33,
//                minutes: 18,
//                seconds: 1,
//            })
//        );
//
//        assert_eq!(
//            CommandPacket::AlarmSet(Time {
//                hours: 18,
//                minutes: 33,
//                seconds: 17,
//            })
//            .to_bytes(),
//            [2, 0, 18, 33, 17, 0]
//        );
//        assert_eq!(
//            CommandPacket::AlarmSet(Time {
//                hours: 33,
//                minutes: 18,
//                seconds: 1,
//            })
//            .to_bytes(),
//            [2, 0, 33, 18, 1, 0]
//        );
//    }
//
//    #[test]
//    fn alarm_get_command() {
//        assert_eq!(CommandPacket::from((3, 0, 0)), CommandPacket::AlarmGet);
//        assert_eq!(CommandPacket::from((3, 11, 22)), CommandPacket::AlarmGet);
//
//        assert_eq!(CommandPacket::AlarmGet.to_bytes(), [3, 0, 0, 0, 0, 0]);
//    }
//
//    #[test]
//    fn reset_command() {
//        assert_eq!(CommandPacket::from((4, 0, 0)), CommandPacket::Reset);
//
//        assert_eq!(CommandPacket::Reset.to_bytes(), [4, 0, 0, 0, 0, 0]);
//    }
//
//    #[test]
//    fn flash_read_command() {
//        assert_eq!(
//            CommandPacket::from((5, 0x1f, 0)),
//            CommandPacket::FlashRead(StorageSlot::One)
//        );
//
//        assert_eq!(
//            CommandPacket::FlashRead(StorageSlot::One).to_bytes(),
//            [5, 0, 0x1f, 0, 0, 0]
//        );
//    }
//
//    #[test]
//    fn flash_write_command() {
//        assert_eq!(
//            CommandPacket::from((6, 0x051f, 0)),
//            CommandPacket::FlashWrite(StorageSlot::One, 5)
//        );
//
//        assert_eq!(
//            CommandPacket::FlashWrite(StorageSlot::One, 5).to_bytes(),
//            [6, 0, 0x1f, 5, 0, 0]
//        );
//    }
//
//    #[test]
//    fn flash_erase_all_command() {
//        assert_eq!(CommandPacket::from((7, 0, 0)), CommandPacket::FlashEraseAll);
//        assert_eq!(CommandPacket::FlashEraseAll.to_bytes(), [7, 0, 0, 0, 0, 0]);
//    }
//
//    #[test]
//    fn unknown_command() {
//        assert_eq!(CommandPacket::from((0, 0, 0)), CommandPacket::Unknown);
//        assert_eq!(CommandPacket::from((8, 0, 0)), CommandPacket::Unknown);
//        assert_eq!(CommandPacket::from((10, 11, 22)), CommandPacket::Unknown);
//
//        assert_eq!(
//            CommandPacket::Unknown.to_bytes(),
//            [0; COMMAND_BYTE_SEQUENCE_LENGTH]
//        );
//    }
//}
