use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::ChatType;
use crate::world::wrath::Language;
use crate::world::wrath::PlayerChatTag;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_messagechat.wowm:126`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_messagechat.wowm#L126):
/// ```text
/// smsg SMSG_MESSAGECHAT = 0x0096 {
///     ChatType chat_type;
///     Language language;
///     Guid sender;
///     u32 flags;
///     if (chat_type == MONSTER_SAY
///         || chat_type == MONSTER_PARTY
///         || chat_type == MONSTER_YELL
///         || chat_type == MONSTER_WHISPER
///         || chat_type == RAID_BOSS_WHISPER
///         || chat_type == RAID_BOSS_EMOTE
///         || chat_type == MONSTER_EMOTE
///         || chat_type == BATTLENET) {
///         SizedCString sender1;
///         Guid target1;
///     }
///     else if (chat_type == WHISPER_FOREIGN) {
///         SizedCString sender2;
///         Guid target2;
///     }
///     else if (chat_type == BG_SYSTEM_NEUTRAL
///         || chat_type == BG_SYSTEM_ALLIANCE
///         || chat_type == BG_SYSTEM_HORDE) {
///         Guid target3;
///     }
///     else if (chat_type == ACHIEVEMENT
///         || chat_type == GUILD_ACHIEVEMENT) {
///         Guid target4;
///     }
///     else if (chat_type == CHANNEL) {
///         CString channel_name;
///         Guid target5;
///     }
///     else {
///         Guid target6;
///     }
///     SizedCString message;
///     PlayerChatTag chat_tag;
///     if (chat_type == ACHIEVEMENT
///         || chat_type == GUILD_ACHIEVEMENT) {
///         u32 achievement_id;
///     }
/// }
/// ```
pub struct SMSG_MESSAGECHAT {
    pub chat_type: SMSG_MESSAGECHAT_ChatType,
    pub language: Language,
    pub sender: Guid,
    /// azerothcore sets to 0.
    ///
    pub flags: u32,
    pub message: String,
    pub chat_tag: PlayerChatTag,
}

impl crate::Message for SMSG_MESSAGECHAT {
    const OPCODE: u32 = 0x0096;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int() as u8).to_le_bytes())?;

        // language: Language
        w.write_all(&(self.language.as_int() as u32).to_le_bytes())?;

        // sender: Guid
        w.write_all(&self.sender.guid().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        match &self.chat_type {
            SMSG_MESSAGECHAT_ChatType::System {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Say {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Party {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Raid {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Guild {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Officer {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Yell {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Whisper {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                sender2,
                target2,
            } => {
                // sender2: SizedCString
                w.write_all(&((sender2.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender2.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target2: Guid
                w.write_all(&target2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WhisperInform {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Emote {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::TextEmote {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterParty {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                target5,
            } => {
                // channel_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
                w.write_all(channel_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelList {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Afk {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Dnd {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Ignored {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Skill {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Loot {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Money {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Opening {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Tradeskills {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::PetInfo {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                target3,
            } => {
                // target3: Guid
                w.write_all(&target3.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                target3,
            } => {
                // target3: Guid
                w.write_all(&target3.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                target3,
            } => {
                // target3: Guid
                w.write_all(&target3.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidLeader {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidWarning {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Filtered {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Battleground {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Restricted {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Battlenet {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Achievement {
                achievement_id,
                target4,
            } => {
                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

                // achievement_id: u32
                w.write_all(&achievement_id.to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                achievement_id,
                target4,
            } => {
                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

                // achievement_id: u32
                w.write_all(&achievement_id.to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::PartyLeader {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
        }

        // message: SizedCString
        w.write_all(&((self.message.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // chat_tag: PlayerChatTag
        w.write_all(&(self.chat_tag.as_int() as u8).to_le_bytes())?;

        match &self.chat_type {
            SMSG_MESSAGECHAT_ChatType::System {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Say {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Party {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Raid {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Guild {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Officer {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Yell {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Whisper {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                sender2,
                target2,
            } => {
                // sender2: SizedCString
                w.write_all(&((sender2.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender2.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target2: Guid
                w.write_all(&target2.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::WhisperInform {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Emote {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::TextEmote {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterSay {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterParty {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterYell {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Channel {
                channel_name,
                target5,
            } => {
                // channel_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
                w.write_all(channel_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelList {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Afk {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Dnd {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Ignored {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Skill {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Loot {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Money {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Opening {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Tradeskills {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::PetInfo {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                target3,
            } => {
                // target3: Guid
                w.write_all(&target3.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                target3,
            } => {
                // target3: Guid
                w.write_all(&target3.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                target3,
            } => {
                // target3: Guid
                w.write_all(&target3.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidLeader {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidWarning {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Filtered {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Battleground {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Restricted {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Battlenet {
                sender1,
                target1,
            } => {
                // sender1: SizedCString
                w.write_all(&((sender1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::Achievement {
                achievement_id,
                target4,
            } => {
                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

                // achievement_id: u32
                w.write_all(&achievement_id.to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                achievement_id,
                target4,
            } => {
                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

                // achievement_id: u32
                w.write_all(&achievement_id.to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
            SMSG_MESSAGECHAT_ChatType::PartyLeader {
                target6,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

            }
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // chat_type: ChatType
        let chat_type: ChatType = crate::util::read_u8_le(r)?.try_into()?;

        // language: Language
        let language: Language = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // sender: Guid
        let sender = Guid::read(r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        let chat_type_if = match chat_type {
            ChatType::System => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::System {
                    target6,
                }
            }
            ChatType::Say => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Say {
                    target6,
                }
            }
            ChatType::Party => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Party {
                    target6,
                }
            }
            ChatType::Raid => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Raid {
                    target6,
                }
            }
            ChatType::Guild => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Guild {
                    target6,
                }
            }
            ChatType::Officer => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Officer {
                    target6,
                }
            }
            ChatType::Yell => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Yell {
                    target6,
                }
            }
            ChatType::Whisper => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Whisper {
                    target6,
                }
            }
            ChatType::WhisperForeign => {
                // sender2: SizedCString
                let sender2 = crate::util::read_u32_le(r)?;
                let sender2 = crate::util::read_sized_c_string_to_vec(r, sender2)?;
                let sender2 = String::from_utf8(sender2)?;;
                // target2: Guid
                let target2 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                    sender2,
                    target2,
                }
            }
            ChatType::WhisperInform => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::WhisperInform {
                    target6,
                }
            }
            ChatType::Emote => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Emote {
                    target6,
                }
            }
            ChatType::TextEmote => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::TextEmote {
                    target6,
                }
            }
            ChatType::MonsterSay => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterSay {
                    sender1,
                    target1,
                }
            }
            ChatType::MonsterParty => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterParty {
                    sender1,
                    target1,
                }
            }
            ChatType::MonsterYell => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterYell {
                    sender1,
                    target1,
                }
            }
            ChatType::MonsterWhisper => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                    sender1,
                    target1,
                }
            }
            ChatType::MonsterEmote => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                    sender1,
                    target1,
                }
            }
            ChatType::Channel => {
                // channel_name: CString
                let channel_name = crate::util::read_c_string_to_vec(r)?;
                let channel_name = String::from_utf8(channel_name)?;

                // target5: Guid
                let target5 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Channel {
                    channel_name,
                    target5,
                }
            }
            ChatType::ChannelJoin => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                    target6,
                }
            }
            ChatType::ChannelLeave => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                    target6,
                }
            }
            ChatType::ChannelList => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelList {
                    target6,
                }
            }
            ChatType::ChannelNotice => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                    target6,
                }
            }
            ChatType::ChannelNoticeUser => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                    target6,
                }
            }
            ChatType::Afk => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Afk {
                    target6,
                }
            }
            ChatType::Dnd => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Dnd {
                    target6,
                }
            }
            ChatType::Ignored => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Ignored {
                    target6,
                }
            }
            ChatType::Skill => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Skill {
                    target6,
                }
            }
            ChatType::Loot => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Loot {
                    target6,
                }
            }
            ChatType::Money => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Money {
                    target6,
                }
            }
            ChatType::Opening => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Opening {
                    target6,
                }
            }
            ChatType::Tradeskills => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Tradeskills {
                    target6,
                }
            }
            ChatType::PetInfo => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::PetInfo {
                    target6,
                }
            }
            ChatType::CombatMiscInfo => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                    target6,
                }
            }
            ChatType::CombatXpGain => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                    target6,
                }
            }
            ChatType::CombatHonorGain => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                    target6,
                }
            }
            ChatType::CombatFactionChange => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                    target6,
                }
            }
            ChatType::BgSystemNeutral => {
                // target3: Guid
                let target3 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                    target3,
                }
            }
            ChatType::BgSystemAlliance => {
                // target3: Guid
                let target3 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                    target3,
                }
            }
            ChatType::BgSystemHorde => {
                // target3: Guid
                let target3 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                    target3,
                }
            }
            ChatType::RaidLeader => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidLeader {
                    target6,
                }
            }
            ChatType::RaidWarning => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidWarning {
                    target6,
                }
            }
            ChatType::RaidBossEmote => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                    sender1,
                    target1,
                }
            }
            ChatType::RaidBossWhisper => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                    sender1,
                    target1,
                }
            }
            ChatType::Filtered => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Filtered {
                    target6,
                }
            }
            ChatType::Battleground => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Battleground {
                    target6,
                }
            }
            ChatType::BattlegroundLeader => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                    target6,
                }
            }
            ChatType::Restricted => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Restricted {
                    target6,
                }
            }
            ChatType::Battlenet => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Battlenet {
                    sender1,
                    target1,
                }
            }
            ChatType::Achievement => {
                // target4: Guid
                let target4 = Guid::read(r)?;

                // achievement_id: u32
                let achievement_id = crate::util::read_u32_le(r)?;

                SMSG_MESSAGECHAT_ChatType::Achievement {
                    achievement_id,
                    target4,
                }
            }
            ChatType::GuildAchievement => {
                // target4: Guid
                let target4 = Guid::read(r)?;

                // achievement_id: u32
                let achievement_id = crate::util::read_u32_le(r)?;

                SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                    achievement_id,
                    target4,
                }
            }
            ChatType::ArenaPoints => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                    target6,
                }
            }
            ChatType::PartyLeader => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::PartyLeader {
                    target6,
                }
            }
        };

        // message: SizedCString
        let message = crate::util::read_u32_le(r)?;
        let message = crate::util::read_sized_c_string_to_vec(r, message)?;
        let message = String::from_utf8(message)?;;
        // chat_tag: PlayerChatTag
        let chat_tag: PlayerChatTag = crate::util::read_u8_le(r)?.try_into()?;

        let chat_type_if = match chat_type {
            ChatType::System => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::System {
                    target6,
                }
            }
            ChatType::Say => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Say {
                    target6,
                }
            }
            ChatType::Party => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Party {
                    target6,
                }
            }
            ChatType::Raid => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Raid {
                    target6,
                }
            }
            ChatType::Guild => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Guild {
                    target6,
                }
            }
            ChatType::Officer => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Officer {
                    target6,
                }
            }
            ChatType::Yell => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Yell {
                    target6,
                }
            }
            ChatType::Whisper => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Whisper {
                    target6,
                }
            }
            ChatType::WhisperForeign => {
                // sender2: SizedCString
                let sender2 = crate::util::read_u32_le(r)?;
                let sender2 = crate::util::read_sized_c_string_to_vec(r, sender2)?;
                let sender2 = String::from_utf8(sender2)?;;
                // target2: Guid
                let target2 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::WhisperForeign {
                    sender2,
                    target2,
                }
            }
            ChatType::WhisperInform => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::WhisperInform {
                    target6,
                }
            }
            ChatType::Emote => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Emote {
                    target6,
                }
            }
            ChatType::TextEmote => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::TextEmote {
                    target6,
                }
            }
            ChatType::MonsterSay => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterSay {
                    sender1,
                    target1,
                }
            }
            ChatType::MonsterParty => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterParty {
                    sender1,
                    target1,
                }
            }
            ChatType::MonsterYell => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterYell {
                    sender1,
                    target1,
                }
            }
            ChatType::MonsterWhisper => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterWhisper {
                    sender1,
                    target1,
                }
            }
            ChatType::MonsterEmote => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::MonsterEmote {
                    sender1,
                    target1,
                }
            }
            ChatType::Channel => {
                // channel_name: CString
                let channel_name = crate::util::read_c_string_to_vec(r)?;
                let channel_name = String::from_utf8(channel_name)?;

                // target5: Guid
                let target5 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Channel {
                    channel_name,
                    target5,
                }
            }
            ChatType::ChannelJoin => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelJoin {
                    target6,
                }
            }
            ChatType::ChannelLeave => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelLeave {
                    target6,
                }
            }
            ChatType::ChannelList => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelList {
                    target6,
                }
            }
            ChatType::ChannelNotice => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelNotice {
                    target6,
                }
            }
            ChatType::ChannelNoticeUser => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ChannelNoticeUser {
                    target6,
                }
            }
            ChatType::Afk => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Afk {
                    target6,
                }
            }
            ChatType::Dnd => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Dnd {
                    target6,
                }
            }
            ChatType::Ignored => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Ignored {
                    target6,
                }
            }
            ChatType::Skill => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Skill {
                    target6,
                }
            }
            ChatType::Loot => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Loot {
                    target6,
                }
            }
            ChatType::Money => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Money {
                    target6,
                }
            }
            ChatType::Opening => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Opening {
                    target6,
                }
            }
            ChatType::Tradeskills => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Tradeskills {
                    target6,
                }
            }
            ChatType::PetInfo => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::PetInfo {
                    target6,
                }
            }
            ChatType::CombatMiscInfo => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CombatMiscInfo {
                    target6,
                }
            }
            ChatType::CombatXpGain => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CombatXpGain {
                    target6,
                }
            }
            ChatType::CombatHonorGain => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CombatHonorGain {
                    target6,
                }
            }
            ChatType::CombatFactionChange => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::CombatFactionChange {
                    target6,
                }
            }
            ChatType::BgSystemNeutral => {
                // target3: Guid
                let target3 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemNeutral {
                    target3,
                }
            }
            ChatType::BgSystemAlliance => {
                // target3: Guid
                let target3 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemAlliance {
                    target3,
                }
            }
            ChatType::BgSystemHorde => {
                // target3: Guid
                let target3 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BgSystemHorde {
                    target3,
                }
            }
            ChatType::RaidLeader => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidLeader {
                    target6,
                }
            }
            ChatType::RaidWarning => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidWarning {
                    target6,
                }
            }
            ChatType::RaidBossEmote => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidBossEmote {
                    sender1,
                    target1,
                }
            }
            ChatType::RaidBossWhisper => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::RaidBossWhisper {
                    sender1,
                    target1,
                }
            }
            ChatType::Filtered => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Filtered {
                    target6,
                }
            }
            ChatType::Battleground => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Battleground {
                    target6,
                }
            }
            ChatType::BattlegroundLeader => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::BattlegroundLeader {
                    target6,
                }
            }
            ChatType::Restricted => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Restricted {
                    target6,
                }
            }
            ChatType::Battlenet => {
                // sender1: SizedCString
                let sender1 = crate::util::read_u32_le(r)?;
                let sender1 = crate::util::read_sized_c_string_to_vec(r, sender1)?;
                let sender1 = String::from_utf8(sender1)?;;
                // target1: Guid
                let target1 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::Battlenet {
                    sender1,
                    target1,
                }
            }
            ChatType::Achievement => {
                // target4: Guid
                let target4 = Guid::read(r)?;

                // achievement_id: u32
                let achievement_id = crate::util::read_u32_le(r)?;

                SMSG_MESSAGECHAT_ChatType::Achievement {
                    achievement_id,
                    target4,
                }
            }
            ChatType::GuildAchievement => {
                // target4: Guid
                let target4 = Guid::read(r)?;

                // achievement_id: u32
                let achievement_id = crate::util::read_u32_le(r)?;

                SMSG_MESSAGECHAT_ChatType::GuildAchievement {
                    achievement_id,
                    target4,
                }
            }
            ChatType::ArenaPoints => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::ArenaPoints {
                    target6,
                }
            }
            ChatType::PartyLeader => {
                // target6: Guid
                let target6 = Guid::read(r)?;

                SMSG_MESSAGECHAT_ChatType::PartyLeader {
                    target6,
                }
            }
        };

        Ok(Self {
            chat_type: chat_type_if,
            language,
            sender,
            flags,
            message,
            chat_tag,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_MESSAGECHAT {}

impl SMSG_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: SMSG_MESSAGECHAT_ChatType
        + 4 // language: Language
        + 8 // sender: Guid
        + 4 // flags: u32
        + self.message.len() + 5 // message: SizedCString
        + 1 // chat_tag: PlayerChatTag
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SMSG_MESSAGECHAT_ChatType {
    System {
        target6: Guid,
    },
    Say {
        target6: Guid,
    },
    Party {
        target6: Guid,
    },
    Raid {
        target6: Guid,
    },
    Guild {
        target6: Guid,
    },
    Officer {
        target6: Guid,
    },
    Yell {
        target6: Guid,
    },
    Whisper {
        target6: Guid,
    },
    WhisperForeign {
        sender2: String,
        target2: Guid,
    },
    WhisperInform {
        target6: Guid,
    },
    Emote {
        target6: Guid,
    },
    TextEmote {
        target6: Guid,
    },
    MonsterSay {
        sender1: String,
        target1: Guid,
    },
    MonsterParty {
        sender1: String,
        target1: Guid,
    },
    MonsterYell {
        sender1: String,
        target1: Guid,
    },
    MonsterWhisper {
        sender1: String,
        target1: Guid,
    },
    MonsterEmote {
        sender1: String,
        target1: Guid,
    },
    Channel {
        channel_name: String,
        target5: Guid,
    },
    ChannelJoin {
        target6: Guid,
    },
    ChannelLeave {
        target6: Guid,
    },
    ChannelList {
        target6: Guid,
    },
    ChannelNotice {
        target6: Guid,
    },
    ChannelNoticeUser {
        target6: Guid,
    },
    Afk {
        target6: Guid,
    },
    Dnd {
        target6: Guid,
    },
    Ignored {
        target6: Guid,
    },
    Skill {
        target6: Guid,
    },
    Loot {
        target6: Guid,
    },
    Money {
        target6: Guid,
    },
    Opening {
        target6: Guid,
    },
    Tradeskills {
        target6: Guid,
    },
    PetInfo {
        target6: Guid,
    },
    CombatMiscInfo {
        target6: Guid,
    },
    CombatXpGain {
        target6: Guid,
    },
    CombatHonorGain {
        target6: Guid,
    },
    CombatFactionChange {
        target6: Guid,
    },
    BgSystemNeutral {
        target3: Guid,
    },
    BgSystemAlliance {
        target3: Guid,
    },
    BgSystemHorde {
        target3: Guid,
    },
    RaidLeader {
        target6: Guid,
    },
    RaidWarning {
        target6: Guid,
    },
    RaidBossEmote {
        sender1: String,
        target1: Guid,
    },
    RaidBossWhisper {
        sender1: String,
        target1: Guid,
    },
    Filtered {
        target6: Guid,
    },
    Battleground {
        target6: Guid,
    },
    BattlegroundLeader {
        target6: Guid,
    },
    Restricted {
        target6: Guid,
    },
    Battlenet {
        sender1: String,
        target1: Guid,
    },
    Achievement {
        achievement_id: u32,
        target4: Guid,
    },
    GuildAchievement {
        achievement_id: u32,
        target4: Guid,
    },
    ArenaPoints {
        target6: Guid,
    },
    PartyLeader {
        target6: Guid,
    },
}

impl Default for SMSG_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::System {
            target6: Default::default(),
        }
    }
}

impl SMSG_MESSAGECHAT_ChatType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::System { .. } => 0,
            Self::Say { .. } => 1,
            Self::Party { .. } => 2,
            Self::Raid { .. } => 3,
            Self::Guild { .. } => 4,
            Self::Officer { .. } => 5,
            Self::Yell { .. } => 6,
            Self::Whisper { .. } => 7,
            Self::WhisperForeign { .. } => 8,
            Self::WhisperInform { .. } => 9,
            Self::Emote { .. } => 10,
            Self::TextEmote { .. } => 11,
            Self::MonsterSay { .. } => 12,
            Self::MonsterParty { .. } => 13,
            Self::MonsterYell { .. } => 14,
            Self::MonsterWhisper { .. } => 15,
            Self::MonsterEmote { .. } => 16,
            Self::Channel { .. } => 17,
            Self::ChannelJoin { .. } => 18,
            Self::ChannelLeave { .. } => 19,
            Self::ChannelList { .. } => 20,
            Self::ChannelNotice { .. } => 21,
            Self::ChannelNoticeUser { .. } => 22,
            Self::Afk { .. } => 23,
            Self::Dnd { .. } => 24,
            Self::Ignored { .. } => 25,
            Self::Skill { .. } => 26,
            Self::Loot { .. } => 27,
            Self::Money { .. } => 28,
            Self::Opening { .. } => 29,
            Self::Tradeskills { .. } => 30,
            Self::PetInfo { .. } => 31,
            Self::CombatMiscInfo { .. } => 32,
            Self::CombatXpGain { .. } => 33,
            Self::CombatHonorGain { .. } => 34,
            Self::CombatFactionChange { .. } => 35,
            Self::BgSystemNeutral { .. } => 36,
            Self::BgSystemAlliance { .. } => 37,
            Self::BgSystemHorde { .. } => 38,
            Self::RaidLeader { .. } => 39,
            Self::RaidWarning { .. } => 40,
            Self::RaidBossEmote { .. } => 41,
            Self::RaidBossWhisper { .. } => 42,
            Self::Filtered { .. } => 43,
            Self::Battleground { .. } => 44,
            Self::BattlegroundLeader { .. } => 45,
            Self::Restricted { .. } => 46,
            Self::Battlenet { .. } => 47,
            Self::Achievement { .. } => 48,
            Self::GuildAchievement { .. } => 49,
            Self::ArenaPoints { .. } => 50,
            Self::PartyLeader { .. } => 51,
        }
    }

}

impl SMSG_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::System {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Say {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Party {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Raid {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Guild {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Officer {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Yell {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Whisper {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::WhisperForeign {
                sender2,
                target2,
            } => {
                1
                + sender2.len() + 5 // sender2: SizedCString
                + 8 // target2: Guid
            }
            Self::WhisperInform {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Emote {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::TextEmote {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::MonsterSay {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + 8 // target1: Guid
            }
            Self::MonsterParty {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + 8 // target1: Guid
            }
            Self::MonsterYell {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + 8 // target1: Guid
            }
            Self::MonsterWhisper {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + 8 // target1: Guid
            }
            Self::MonsterEmote {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + 8 // target1: Guid
            }
            Self::Channel {
                channel_name,
                target5,
            } => {
                1
                + channel_name.len() + 1 // channel_name: CString
                + 8 // target5: Guid
            }
            Self::ChannelJoin {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::ChannelLeave {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::ChannelList {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::ChannelNotice {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::ChannelNoticeUser {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Afk {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Dnd {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Ignored {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Skill {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Loot {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Money {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Opening {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Tradeskills {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::PetInfo {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::CombatMiscInfo {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::CombatXpGain {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::CombatHonorGain {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::CombatFactionChange {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::BgSystemNeutral {
                target3,
            } => {
                1
                + 8 // target3: Guid
            }
            Self::BgSystemAlliance {
                target3,
            } => {
                1
                + 8 // target3: Guid
            }
            Self::BgSystemHorde {
                target3,
            } => {
                1
                + 8 // target3: Guid
            }
            Self::RaidLeader {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::RaidWarning {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::RaidBossEmote {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + 8 // target1: Guid
            }
            Self::RaidBossWhisper {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + 8 // target1: Guid
            }
            Self::Filtered {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Battleground {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::BattlegroundLeader {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Restricted {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::Battlenet {
                sender1,
                target1,
            } => {
                1
                + sender1.len() + 5 // sender1: SizedCString
                + 8 // target1: Guid
            }
            Self::Achievement {
                achievement_id,
                target4,
            } => {
                1
                + 4 // achievement_id: u32
                + 8 // target4: Guid
            }
            Self::GuildAchievement {
                achievement_id,
                target4,
            } => {
                1
                + 4 // achievement_id: u32
                + 8 // target4: Guid
            }
            Self::ArenaPoints {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
            Self::PartyLeader {
                target6,
            } => {
                1
                + 8 // target6: Guid
            }
        }
    }
}
