# SMSG_LOOT_LIST

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_loot_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_loot_list.wowm#L1).
```rust,ignore
smsg SMSG_LOOT_LIST = 0x03F8 {
    Guid creature;
    PackedGuid master_looter;
    PackedGuid group_looter;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | creature |  |  |
| 0x0C | - / - | [PackedGuid](../spec/packed-guid.md) | master_looter |  |  |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | group_looter |  |  |

# SMSG_LOOT_LIST

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_loot_list.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_loot_list.wowm#L9).
```rust,ignore
smsg SMSG_LOOT_LIST = 0x03F9 {
    Guid creature;
    PackedGuid master_looter;
    PackedGuid group_looter;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | creature |  |  |
| 0x0C | - / - | [PackedGuid](../spec/packed-guid.md) | master_looter |  |  |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | group_looter |  |  |
