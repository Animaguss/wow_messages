use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_guild_filter.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_guild_filter.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_GUILD_FILTER = 0x042B {
///     u32 minimum_level;
///     u32 maximum_level;
///     u32 minimum_rank;
/// }
/// ```
pub struct CMSG_CALENDAR_GUILD_FILTER {
    pub minimum_level: u32,
    pub maximum_level: u32,
    pub minimum_rank: u32,
}

impl crate::Message for CMSG_CALENDAR_GUILD_FILTER {
    const OPCODE: u32 = 0x042b;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // minimum_level: u32
        w.write_all(&self.minimum_level.to_le_bytes())?;

        // maximum_level: u32
        w.write_all(&self.maximum_level.to_le_bytes())?;

        // minimum_rank: u32
        w.write_all(&self.minimum_rank.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x042B, size: body_size as u32 });
        }

        // minimum_level: u32
        let minimum_level = crate::util::read_u32_le(r)?;

        // maximum_level: u32
        let maximum_level = crate::util::read_u32_le(r)?;

        // minimum_rank: u32
        let minimum_rank = crate::util::read_u32_le(r)?;

        Ok(Self {
            minimum_level,
            maximum_level,
            minimum_rank,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CALENDAR_GUILD_FILTER {}
