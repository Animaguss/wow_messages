# SendCalendarEvent

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_calendar_send_calendar.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_calendar_send_calendar.wowm#L12).
```rust,ignore
struct SendCalendarEvent {
    Guid event_id;
    CString title;
    u32 event_type;
    DateTime event_time;
    u32 flags;
    u32 dungeon_id;
    PackedGuid creator;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 8 / Little | [Guid](../spec/packed-guid.md) | event_id |  |  |
| 0x08 | - / - | CString | title |  |  |
| - | 4 / Little | u32 | event_type |  |  |
| - | 4 / Little | DateTime | event_time |  |  |
| - | 4 / Little | u32 | flags |  |  |
| - | 4 / Little | u32 | dungeon_id |  |  |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | creator |  |  |
