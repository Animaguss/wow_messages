# SMSG_LFG_UPDATE_LFM

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_lfg_update_lfm.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_lfg_update_lfm.wowm#L1).
```rust,ignore
smsg SMSG_LFG_UPDATE_LFM = 0x036D {
    LfgUpdateLookingForMore looking_for_more;
    if (looking_for_more == LOOKING_FOR_MORE) {
        LfgData data;
    }
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
| 0x04 | 1 / - | [LfgUpdateLookingForMore](lfgupdatelookingformore.md) | looking_for_more |  |  |

If looking_for_more is equal to `LOOKING_FOR_MORE`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x05 | 4 / - | [LfgData](lfgdata.md) | data |  |  |
