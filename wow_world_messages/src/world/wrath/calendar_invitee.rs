use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_add_event.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_add_event.wowm#L1):
/// ```text
/// struct CalendarInvitee {
///     PackedGuid guid;
///     u8 status;
///     u8 rank;
/// }
/// ```
pub struct CalendarInvitee {
    pub guid: Guid,
    pub status: u8,
    pub rank: u8,
}

impl CalendarInvitee {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // status: u8
        w.write_all(&self.status.to_le_bytes())?;

        // rank: u8
        w.write_all(&self.rank.to_le_bytes())?;

        Ok(())
    }
}

impl CalendarInvitee {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // status: u8
        let status = crate::util::read_u8_le(r)?;

        // rank: u8
        let rank = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            status,
            rank,
        })
    }

}

impl CalendarInvitee {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 1 // status: u8
        + 1 // rank: u8
    }
}
