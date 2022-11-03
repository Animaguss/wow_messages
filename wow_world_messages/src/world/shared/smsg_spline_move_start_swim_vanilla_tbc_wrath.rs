use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_start_swim.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_start_swim.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_START_SWIM = 0x030B {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_START_SWIM {
    pub guid: Guid,
}

impl crate::Message for SMSG_SPLINE_MOVE_START_SWIM {
    const OPCODE: u32 = 0x030b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SPLINE_MOVE_START_SWIM {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_SPLINE_MOVE_START_SWIM {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_SPLINE_MOVE_START_SWIM {}

impl SMSG_SPLINE_MOVE_START_SWIM {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}
