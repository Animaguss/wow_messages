use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellheallog.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellheallog.wowm#L24):
/// ```text
/// smsg SMSG_SPELLHEALLOG = 0x0150 {
///     PackedGuid victim;
///     PackedGuid caster;
///     u32 id;
///     u32 damage;
///     u32 overheal;
///     u32 absorb;
///     Bool critical;
///     u8 unknown;
/// }
/// ```
pub struct SMSG_SPELLHEALLOG {
    pub victim: Guid,
    pub caster: Guid,
    pub id: u32,
    pub damage: u32,
    pub overheal: u32,
    pub absorb: u32,
    pub critical: bool,
    /// mangostwo: unused in client?
    ///
    pub unknown: u8,
}

impl crate::Message for SMSG_SPELLHEALLOG {
    const OPCODE: u32 = 0x0150;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // victim: PackedGuid
        self.victim.write_packed_guid_into_vec(w);

        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(w);

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // overheal: u32
        w.write_all(&self.overheal.to_le_bytes())?;

        // absorb: u32
        w.write_all(&self.absorb.to_le_bytes())?;

        // critical: Bool
        w.write_all(u8::from(self.critical).to_le_bytes().as_slice())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(22..=36).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0150, size: body_size as u32 });
        }

        // victim: PackedGuid
        let victim = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // overheal: u32
        let overheal = crate::util::read_u32_le(r)?;

        // absorb: u32
        let absorb = crate::util::read_u32_le(r)?;

        // critical: Bool
        let critical = crate::util::read_u8_le(r)? != 0;
        // unknown: u8
        let unknown = crate::util::read_u8_le(r)?;

        Ok(Self {
            victim,
            caster,
            id,
            damage,
            overheal,
            absorb,
            critical,
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_SPELLHEALLOG {}

impl SMSG_SPELLHEALLOG {
    pub(crate) fn size(&self) -> usize {
        self.victim.size() // victim: Guid
        + self.caster.size() // caster: Guid
        + 4 // id: u32
        + 4 // damage: u32
        + 4 // overheal: u32
        + 4 // absorb: u32
        + 1 // critical: Bool
        + 1 // unknown: u8
    }
}
