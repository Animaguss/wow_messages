# PendingAuctionSale

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_auction_list_pending_sales.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_auction_list_pending_sales.wowm#L1).
```rust,ignore
struct PendingAuctionSale {
    CString string1;
    CString string2;
    u32 unknown1;
    u32 unknown2;
    f32 time_left;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | - / - | CString | string1 |  | mangostwo: string '%d:%d:%d:%d:%d' -> itemId, ItemRandomPropertyId, 2, auctionId, unk1 (stack size?, unused) |
| - | - / - | CString | string2 |  | mangostwo: string '%16I64X:%d:%d:%d:%d' -> bidderGuid, bid, buyout, deposit, auctionCut |
| - | 4 / Little | u32 | unknown1 |  | mangostwo sets to 97250. |
| - | 4 / Little | u32 | unknown2 |  | mangostwo sets to 68. |
| - | 4 / Little | f32 | time_left |  |  |
