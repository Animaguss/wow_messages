use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::Vector3d;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_update_projectile_position.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_update_projectile_position.wowm#L1):
/// ```text
/// cmsg CMSG_UPDATE_PROJECTILE_POSITION = 0x04BE {
///     Guid caster;
///     u32 spell;
///     u8 cast_count;
///     Vector3d position;
/// }
/// ```
pub struct CMSG_UPDATE_PROJECTILE_POSITION {
    pub caster: Guid,
    pub spell: u32,
    pub cast_count: u8,
    pub position: Vector3d,
}

impl crate::Message for CMSG_UPDATE_PROJECTILE_POSITION {
    const OPCODE: u32 = 0x04be;

    fn size_without_header(&self) -> u32 {
        25
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 25 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04BE, size: body_size as u32 });
        }

        // caster: Guid
        let caster = Guid::read(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // cast_count: u8
        let cast_count = crate::util::read_u8_le(r)?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        Ok(Self {
            caster,
            spell,
            cast_count,
            position,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_UPDATE_PROJECTILE_POSITION {}
