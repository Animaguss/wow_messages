## Client Version 1.12

```rust,ignore
enum ChatType : u8 {
    SAY = 0x00;    
    PARTY = 0x01;    
    RAID = 0x02;    
    GUILD = 0x03;    
    OFFICER = 0x04;    
    YELL = 0x05;    
    WHISPER = 0x06;    
    WHISPER_INFORM = 0x07;    
    EMOTE = 0x08;    
    TEXT_EMOTE = 0x09;    
    SYSTEM = 0x0A;    
    MONSTER_SAY = 0x0B;    
    MONSTER_YELL = 0x0C;    
    MONSTER_EMOTE = 0x0D;    
    CHANNEL = 0x0E;    
    CHANNEL_JOIN = 0x0F;    
    CHANNEL_LEAVE = 0x10;    
    CHANNEL_LIST = 0x11;    
    CHANNEL_NOTICE = 0x12;    
    CHANNEL_NOTICE_USER = 0x13;    
    AFK = 0x14;    
    DND = 0x15;    
    IGNORED = 0x16;    
    SKILL = 0x17;    
    LOOT = 0x18;    
    MONSTER_WHISPER = 0x1A;    
    BG_SYSTEM_NEUTRAL = 0x52;    
    BG_SYSTEM_ALLIANCE = 0x53;    
    BG_SYSTEM_HORDE = 0x54;    
    RAID_LEADER = 0x57;    
    RAID_WARNING = 0x58;    
    RAID_BOSS_WHISPER = 0x59;    
    RAID_BOSS_EMOTE = 0x5A;    
    BATTLEGROUND = 0x5C;    
    BATTLEGROUND_LEADER = 0x5D;    
}

```
