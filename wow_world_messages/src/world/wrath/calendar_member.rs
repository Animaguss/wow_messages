use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_calendar_filter_guild.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_calendar_filter_guild.wowm#L1):
/// ```text
/// struct CalendarMember {
///     PackedGuid member;
///     u8 level;
/// }
/// ```
pub struct CalendarMember {
    pub member: Guid,
    pub level: u8,
}

impl CalendarMember {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // member: PackedGuid
        self.member.write_packed_guid_into_vec(w);

        // level: u8
        w.write_all(&self.level.to_le_bytes())?;

        Ok(())
    }
}

impl CalendarMember {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // member: PackedGuid
        let member = Guid::read_packed(r)?;

        // level: u8
        let level = crate::util::read_u8_le(r)?;

        Ok(Self {
            member,
            level,
        })
    }

}

impl CalendarMember {
    pub(crate) fn size(&self) -> usize {
        self.member.size() // member: Guid
        + 1 // level: u8
    }
}
