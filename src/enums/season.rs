use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Season {
    #[default]
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Season::Spring,
            1 => Season::Summer,
            2 => Season::Autumn,
            3 => Season::Winter,
            _ => panic!("Invalid season index '{}'", index),
        }
    }
}
