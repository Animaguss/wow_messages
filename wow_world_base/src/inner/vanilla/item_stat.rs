use std::convert::{TryFrom, TryInto};
use crate::vanilla::ItemStatType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:77`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L77):
/// ```text
/// struct ItemStat {
///     (u32)ItemStatType stat_type;
///     i32 value;
/// }
/// ```
pub struct ItemStat {
    pub stat_type: ItemStatType,
    pub value: i32,
}

impl ItemStat {
    pub fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // stat_type: ItemStatType
        w.write_all(&(self.stat_type.as_int() as u32).to_le_bytes())?;

        // value: i32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }
}

impl ItemStat {
    pub fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // stat_type: ItemStatType
        let stat_type: ItemStatType = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // value: i32
        let value = crate::util::read_i32_le(r)?;

        Ok(Self {
            stat_type,
            value,
        })
    }

}
