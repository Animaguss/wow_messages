use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Only exists as a comment in trinitycore/azerothcore.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_camera_shake.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_camera_shake.wowm#L1):
/// ```text
/// smsg SMSG_CAMERA_SHAKE = 0x050A {
///     u32 camera_shake_id;
///     u32 unknown;
/// }
/// ```
pub struct SMSG_CAMERA_SHAKE {
    /// SpellEffectCameraShakes.dbc
    ///
    pub camera_shake_id: u32,
    pub unknown: u32,
}

impl crate::Message for SMSG_CAMERA_SHAKE {
    const OPCODE: u32 = 0x050a;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // camera_shake_id: u32
        w.write_all(&self.camera_shake_id.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x050A, size: body_size as u32 });
        }

        // camera_shake_id: u32
        let camera_shake_id = crate::util::read_u32_le(r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        Ok(Self {
            camera_shake_id,
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CAMERA_SHAKE {}
