use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameItemEventError {
    #[error("Item with type_id '{type_id}' does not exist")]
    InvalidItemType { type_id: i32 },
    #[error("Item with type_id '{type_id}' is not a rod")]
    NotARod { type_id: i32 },
}

impl GameItemEventError {
    pub fn invalid_item_type(type_id: i32) -> Self {
        Self::InvalidItemType { type_id }
    }

    pub fn not_a_rod(type_id: i32) -> Self {
        Self::NotARod { type_id }
    }

    pub fn is_invalid_item_type(&self) -> bool {
        matches!(self, Self::InvalidItemType { .. })
    }

    pub fn is_not_a_rod(&self) -> bool {
        matches!(self, Self::NotARod { .. })
    }

    pub fn get_type_id(&self) -> Option<i32> {
        match self {
            Self::InvalidItemType { type_id } => Some(*type_id),
            Self::NotARod { type_id } => Some(*type_id),
            _ => None,
        }
    }
}
