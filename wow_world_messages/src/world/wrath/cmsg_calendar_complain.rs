use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_complain.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_complain.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_COMPLAIN = 0x0446 {
///     Guid responsible_player;
///     Guid event;
///     Guid invite_id;
/// }
/// ```
pub struct CMSG_CALENDAR_COMPLAIN {
    pub responsible_player: Guid,
    pub event: Guid,
    pub invite_id: Guid,
}

impl crate::Message for CMSG_CALENDAR_COMPLAIN {
    const OPCODE: u32 = 0x0446;

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // responsible_player: Guid
        w.write_all(&self.responsible_player.guid().to_le_bytes())?;

        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0446, size: body_size as u32 });
        }

        // responsible_player: Guid
        let responsible_player = Guid::read(r)?;

        // event: Guid
        let event = Guid::read(r)?;

        // invite_id: Guid
        let invite_id = Guid::read(r)?;

        Ok(Self {
            responsible_player,
            event,
            invite_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CALENDAR_COMPLAIN {}
