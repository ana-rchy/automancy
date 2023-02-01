use serde::Deserialize;

use crate::data::id::{Id, IdRaw, Interner};

pub type ItemAmount = u64;

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub id: Id,
    pub amount: ItemAmount,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemRaw {
    pub id: IdRaw,
    pub amount: ItemAmount,
}

impl ItemRaw {
    pub fn to_item(&self, interner: &mut Interner) -> Item {
        Item {
            id: self.id.to_id(interner),
            amount: self.amount,
        }
    }
}