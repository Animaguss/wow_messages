# SMSG_BATTLEFIELD_MGR_EJECTED

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_battlefield_mgr_ejected.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_battlefield_mgr_ejected.wowm#L1).
```rust,ignore
smsg SMSG_BATTLEFIELD_MGR_EJECTED = 0x04E6 {
    u32 battle_id;
    u8 reason;
    u8 battle_status;
    u8 relocated;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 4 / Little | u32 | battle_id |  |  |
| 0x08 | 1 / - | u8 | reason |  |  |
| 0x09 | 1 / - | u8 | battle_status |  |  |
| 0x0A | 1 / - | u8 | relocated |  |  |
