use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_pre_resurrect.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_pre_resurrect.wowm#L1):
/// ```text
/// smsg SMSG_PRE_RESURRECT = 0x0494 {
///     PackedGuid player;
/// }
/// ```
pub struct SMSG_PRE_RESURRECT {
    pub player: Guid,
}

impl crate::Message for SMSG_PRE_RESURRECT {
    const OPCODE: u32 = 0x0494;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0494, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_PRE_RESURRECT {}

impl SMSG_PRE_RESURRECT {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
    }
}
