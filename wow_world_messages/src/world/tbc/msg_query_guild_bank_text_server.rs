use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_query_guild_bank_text.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_query_guild_bank_text.wowm#L13):
/// ```text
/// smsg MSG_QUERY_GUILD_BANK_TEXT_Server = 0x0409 {
///     u8 tab;
///     CString text;
/// }
/// ```
pub struct MSG_QUERY_GUILD_BANK_TEXT_Server {
    pub tab: u8,
    pub text: String,
}

impl crate::Message for MSG_QUERY_GUILD_BANK_TEXT_Server {
    const OPCODE: u32 = 0x0409;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=257).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0409, size: body_size as u32 });
        }

        // tab: u8
        let tab = crate::util::read_u8_le(r)?;

        // text: CString
        let text = crate::util::read_c_string_to_vec(r)?;
        let text = String::from_utf8(text)?;

        Ok(Self {
            tab,
            text,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_QUERY_GUILD_BANK_TEXT_Server {}

impl MSG_QUERY_GUILD_BANK_TEXT_Server {
    pub(crate) fn size(&self) -> usize {
        1 // tab: u8
        + self.text.len() + 1 // text: CString
    }
}
