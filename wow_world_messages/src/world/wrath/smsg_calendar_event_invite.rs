use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::DateTime;
use crate::world::wrath::CalendarStatusTime;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_calendar_event_invite.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_calendar_event_invite.wowm#L8):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_INVITE = 0x043A {
///     PackedGuid invitee;
///     Guid event_id;
///     Guid invite_id;
///     u8 level;
///     u8 invite_status;
///     CalendarStatusTime time;
///     if (time == PRESENT) {
///         DateTime status_time;
///     }
///     Bool is_sign_up;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_INVITE {
    pub invitee: Guid,
    pub event_id: Guid,
    pub invite_id: Guid,
    pub level: u8,
    pub invite_status: u8,
    pub time: SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime,
    pub is_sign_up: bool,
}

impl crate::Message for SMSG_CALENDAR_EVENT_INVITE {
    const OPCODE: u32 = 0x043a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // invitee: PackedGuid
        self.invitee.write_packed_guid_into_vec(w);

        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // level: u8
        w.write_all(&self.level.to_le_bytes())?;

        // invite_status: u8
        w.write_all(&self.invite_status.to_le_bytes())?;

        // time: CalendarStatusTime
        w.write_all(&(self.time.as_int() as u8).to_le_bytes())?;

        match &self.time {
            SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime::NotPresent => {}
            SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime::Present {
                status_time,
            } => {
                // status_time: DateTime
                w.write_all(&status_time.as_int().to_le_bytes())?;

            }
        }

        // is_sign_up: Bool
        w.write_all(u8::from(self.is_sign_up).to_le_bytes().as_slice())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(22..=33).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x043A, size: body_size as u32 });
        }

        // invitee: PackedGuid
        let invitee = Guid::read_packed(r)?;

        // event_id: Guid
        let event_id = Guid::read(r)?;

        // invite_id: Guid
        let invite_id = Guid::read(r)?;

        // level: u8
        let level = crate::util::read_u8_le(r)?;

        // invite_status: u8
        let invite_status = crate::util::read_u8_le(r)?;

        // time: CalendarStatusTime
        let time: CalendarStatusTime = crate::util::read_u8_le(r)?.try_into()?;

        let time_if = match time {
            CalendarStatusTime::NotPresent => SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime::NotPresent,
            CalendarStatusTime::Present => {
                // status_time: DateTime
                let status_time: DateTime = crate::util::read_u32_le(r)?.try_into()?;
                SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime::Present {
                    status_time,
                }
            }
        };

        // is_sign_up: Bool
        let is_sign_up = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            invitee,
            event_id,
            invite_id,
            level,
            invite_status,
            time: time_if,
            is_sign_up,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CALENDAR_EVENT_INVITE {}

impl SMSG_CALENDAR_EVENT_INVITE {
    pub(crate) fn size(&self) -> usize {
        self.invitee.size() // invitee: Guid
        + 8 // event_id: Guid
        + 8 // invite_id: Guid
        + 1 // level: u8
        + 1 // invite_status: u8
        + self.time.size() // time: SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime
        + 1 // is_sign_up: Bool
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime {
    NotPresent,
    Present {
        status_time: DateTime,
    },
}

impl Default for SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotPresent
    }
}

impl SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0,
            Self::Present { .. } => 1,
        }
    }

}

impl SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NotPresent => {
                1
            }
            Self::Present {
                status_time,
            } => {
                1
                + 4 // status_time: DateTime
            }
        }
    }
}
