use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::DateTime;
use crate::world::wrath::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_calendar_raid_lockout_added.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_calendar_raid_lockout_added.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_RAID_LOCKOUT_ADDED = 0x043E {
///     DateTime time;
///     Map map;
///     u32 difficulty;
///     u32 remaining_time;
///     Guid instance_id;
/// }
/// ```
pub struct SMSG_CALENDAR_RAID_LOCKOUT_ADDED {
    pub time: DateTime,
    pub map: Map,
    pub difficulty: u32,
    pub remaining_time: u32,
    pub instance_id: Guid,
}

impl crate::Message for SMSG_CALENDAR_RAID_LOCKOUT_ADDED {
    const OPCODE: u32 = 0x043e;

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // time: DateTime
        w.write_all(&self.time.as_int().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // difficulty: u32
        w.write_all(&self.difficulty.to_le_bytes())?;

        // remaining_time: u32
        w.write_all(&self.remaining_time.to_le_bytes())?;

        // instance_id: Guid
        w.write_all(&self.instance_id.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x043E, size: body_size as u32 });
        }

        // time: DateTime
        let time: DateTime = crate::util::read_u32_le(r)?.try_into()?;
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // difficulty: u32
        let difficulty = crate::util::read_u32_le(r)?;

        // remaining_time: u32
        let remaining_time = crate::util::read_u32_le(r)?;

        // instance_id: Guid
        let instance_id = Guid::read(r)?;

        Ok(Self {
            time,
            map,
            difficulty,
            remaining_time,
            instance_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CALENDAR_RAID_LOCKOUT_ADDED {}
