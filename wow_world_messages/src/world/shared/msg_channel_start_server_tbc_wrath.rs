use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_channel_start.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_channel_start.wowm#L8):
/// ```text
/// smsg MSG_CHANNEL_START_Server = 0x0139 {
///     PackedGuid caster;
///     u32 spell;
///     u32 duration;
/// }
/// ```
pub struct MSG_CHANNEL_START_Server {
    pub caster: Guid,
    pub spell: u32,
    pub duration: u32,
}

impl crate::Message for MSG_CHANNEL_START_Server {
    const OPCODE: u32 = 0x0139;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(w);

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=17).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0139, size: body_size as u32 });
        }

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        Ok(Self {
            caster,
            spell,
            duration,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_CHANNEL_START_Server {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_CHANNEL_START_Server {}

impl MSG_CHANNEL_START_Server {
    pub(crate) fn size(&self) -> usize {
        self.caster.size() // caster: Guid
        + 4 // spell: u32
        + 4 // duration: u32
    }
}
