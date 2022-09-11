use std::convert::{TryFrom, TryInto};
use crate::world::tbc::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_stop_strafe.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_stop_strafe.wowm#L7):
/// ```text
/// cmsg MSG_MOVE_STOP_STRAFE_Client = 0x00BA {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_STOP_STRAFE_Client {
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_STOP_STRAFE_Client {
    const OPCODE: u32 = 0x00ba;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for MSG_MOVE_STOP_STRAFE_Client {}

impl MSG_MOVE_STOP_STRAFE_Client {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}
