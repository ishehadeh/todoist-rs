use types::*;
use command;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]

/// A Todoist filter
pub struct Filter {
    /// The filter's unique ID
    pub id : ID,

    /// The filter name
    pub name : String,

    /// The filter query
    pub query : String,

    /// The filter's color
    pub color : Color,

    /// The filter's place in the filter list (lowest is first)
    pub order : isize,

    /// whether this filter is marked as deleted
    pub is_deleted : isize,

    /// whether this filter is marked as a favorite
    pub is_favorite : isize,
}

impl Filter {
    pub fn add() -> command::filter::Add {
        command::filter::Add::default()
    }

    pub fn update(&self) -> command::filter::Update {
        command::filter::Update {
            id: self.id,
            item_order: self.order,
            is_favorite: self.is_favorite,
            name: self.name.clone(),
            query: self.query.clone(),
            color: self.color.clone(),
        }
    }

    pub fn delete(&self) -> command::filter::Delete {
        command::filter::Delete {
            ids: vec![self.id]
        }
    }
}