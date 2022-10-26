use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_get_mail_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_get_mail_list.wowm#L3):
/// ```text
/// cmsg CMSG_GET_MAIL_LIST = 0x023A {
///     Guid mailbox;
/// }
/// ```
pub struct CMSG_GET_MAIL_LIST {
    pub mailbox: Guid,
}

impl crate::Message for CMSG_GET_MAIL_LIST {
    const OPCODE: u32 = 0x023a;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // mailbox: Guid
        w.write_all(&self.mailbox.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // mailbox: Guid
        let mailbox = Guid::read(r)?;

        Ok(Self {
            mailbox,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GET_MAIL_LIST {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_GET_MAIL_LIST {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GET_MAIL_LIST {}
