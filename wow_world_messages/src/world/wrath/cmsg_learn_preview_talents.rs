use std::convert::{TryFrom, TryInto};
use crate::world::wrath::Talent;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_learn_preview_talents.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_learn_preview_talents.wowm#L8):
/// ```text
/// cmsg CMSG_LEARN_PREVIEW_TALENTS = 0x04C1 {
///     u32 amount_of_talents;
///     Talent[amount_of_talents] talents;
/// }
/// ```
pub struct CMSG_LEARN_PREVIEW_TALENTS {
    pub talents: Vec<Talent>,
}

impl crate::Message for CMSG_LEARN_PREVIEW_TALENTS {
    const OPCODE: u32 = 0x04c1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_talents: u32
        w.write_all(&(self.talents.len() as u32).to_le_bytes())?;

        // talents: Talent[amount_of_talents]
        for i in self.talents.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04C1, size: body_size as u32 });
        }

        // amount_of_talents: u32
        let amount_of_talents = crate::util::read_u32_le(r)?;

        // talents: Talent[amount_of_talents]
        let mut talents = Vec::with_capacity(amount_of_talents as usize);
        for i in 0..amount_of_talents {
            talents.push(Talent::read(r)?);
        }

        Ok(Self {
            talents,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_LEARN_PREVIEW_TALENTS {}

impl CMSG_LEARN_PREVIEW_TALENTS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_talents: u32
        + self.talents.len() * 8 // talents: Talent[amount_of_talents]
    }
}
