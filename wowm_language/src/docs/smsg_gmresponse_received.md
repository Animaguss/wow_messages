# SMSG_GMRESPONSE_RECEIVED

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_gmresponse_received.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_gmresponse_received.wowm#L1).
```rust,ignore
smsg SMSG_GMRESPONSE_RECEIVED = 0x04EF {
    u32 response_id;
    u32 ticket_id;
    CString message;
    CString[4] response;
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
| 0x04 | 4 / Little | u32 | response_id |  |  |
| 0x08 | 4 / Little | u32 | ticket_id |  |  |
| 0x0C | - / - | CString | message |  |  |
| - | ? / - | CString[4] | response |  |  |
