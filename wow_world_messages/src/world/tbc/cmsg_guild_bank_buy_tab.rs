use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_guild_bank_buy_tab.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_guild_bank_buy_tab.wowm#L1):
/// ```text
/// cmsg CMSG_GUILD_BANK_BUY_TAB = 0x03E9 {
///     Guid banker;
///     u8 tab;
/// }
/// ```
pub struct CMSG_GUILD_BANK_BUY_TAB {
    pub banker: Guid,
    pub tab: u8,
}

impl crate::Message for CMSG_GUILD_BANK_BUY_TAB {
    const OPCODE: u32 = 0x03e9;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // banker: Guid
        w.write_all(&self.banker.guid().to_le_bytes())?;

        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03E9, size: body_size as u32 });
        }

        // banker: Guid
        let banker = Guid::read(r)?;

        // tab: u8
        let tab = crate::util::read_u8_le(r)?;

        Ok(Self {
            banker,
            tab,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_GUILD_BANK_BUY_TAB {}
