# MSG_MOVE_ROOT_Server

## Client Version 2.4.3

There does not appear to be a CMSG version of this MSG.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/msg/msg_move_root.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_root.wowm#L2).
```rust,ignore
smsg MSG_MOVE_ROOT_Server = 0x00EC {
    PackedGuid player;
    MovementInfo info;
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

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | - / - | [PackedGuid](../types/packed-guid.md) | player |  |
| - | - / - | [MovementInfo](movementinfo.md) | info |  |

## Client Version 3.3.5

There does not appear to be a CMSG version of this MSG.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/msg/msg_move_root.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_root.wowm#L10).
```rust,ignore
smsg MSG_MOVE_ROOT_Server = 0x00EC {
    MovementInfo info;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 **OR** 3 / Big           | uint16 **OR** uint16+uint8 | size | Size of the rest of the message including the opcode field but not including the size field. Wrath server messages **can** be 3 bytes. If the first (most significant) size byte has `0x80` set, the header will be 3 bytes, otherwise it is 2.|
| -      | 2 / Little| uint16 | opcode | Opcode that determines which fields the message contains. |

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| - | - / - | [MovementInfo](movementinfo.md) | info |  |

