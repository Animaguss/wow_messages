# LfgAvailableDungeon

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_lfg_player_info.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_lfg_player_info.wowm#L9).
```rust,ignore
struct LfgAvailableDungeon {
    u32 dungeon_entry;
    Bool done;
    u32 quest_reward;
    u32 xp_reward;
    u32 unknown1;
    u32 unknown2;
    u8 amount_of_rewards;
    LfgQuestReward[amount_of_rewards] rewards;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | dungeon_entry |  |  |
| 0x04 | 1 / - | Bool | done |  |  |
| 0x05 | 4 / Little | u32 | quest_reward |  |  |
| 0x09 | 4 / Little | u32 | xp_reward |  |  |
| 0x0D | 4 / Little | u32 | unknown1 |  |  |
| 0x11 | 4 / Little | u32 | unknown2 |  |  |
| 0x15 | 1 / - | u8 | amount_of_rewards |  |  |
| 0x16 | ? / - | [LfgQuestReward](lfgquestreward.md)[amount_of_rewards] | rewards |  |  |
