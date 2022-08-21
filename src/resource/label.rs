use command;
use types::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]

/// A Todoist label (premium users only)
pub struct Label {
    /// The label's ID
    pub id: ID,

    /// The label's name
    pub name: String,

    /// The label's Color
    pub color: Color,

    /// This label's position in the label list, the smallest number should be at the top
    pub item_order: isize,

    // 1 if this label has been marked as deleted
    pub is_deleted: isize,
    // 1 if this label has been marked as a favorite
    pub is_favorite: isize,
}

impl Label {
    pub fn add() -> command::label::Add {
        command::label::Add::default()
    }

    pub fn update(&self) -> command::label::Update {
        command::label::Update {
            id: self.id,
            item_order: self.item_order,
            is_favorite: self.is_favorite,
            name: self.name.clone(),
            color: self.color.clone(),
        }
    }

    pub fn delete(&self) -> command::label::Delete {
        command::label::Delete { ids: vec![self.id] }
    }
}
