//! Project related structures
use types::*;
use uuid::Uuid;

use command;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
#[serde(default)]

/// A Todoist Project
pub struct Project {
    /// The project's unique ID
    pub id: ID,

    /// The project's name
    pub name: String,

    /// The project's Color
    pub color: Color,

    /// The project's indent (hierarchy level) is a number between from 1-4
    pub indent: u8,

    /// This project's position in the project list, the smallest number should be at the top
    pub item_order: isize,

    /// Whether this project's child's project's children are visible
    pub collapsed: isize,

    pub shared: bool,

    // 1 if this project has been marked as deleted
    pub is_deleted: isize,

    // 1 if this project has been marked as archived
    pub is_archived: isize,

    // 1 if this project has been marked as a favorite
    pub is_favorite: isize,

    /// True if this project is in the user's inbox
    pub inbox: bool,

    /// True if this project is in the team's inbox
    pub inbox_team: bool,
}

impl Project {
    pub fn add() -> command::project::Add {
        command::project::Add::default()
    }

    pub fn update(&self) -> command::project::Update {
        command::project::Update {
            id: self.id,
            item_order: self.item_order,
            is_favorite: self.is_favorite,
            name: self.name.clone(),
            color: self.color.clone(),
            indent: self.indent,
            collapsed: self.collapsed,
        }
    }

    pub fn delete(&self) -> command::project::Delete {
        command::project::Delete { ids: vec![self.id] }
    }
}

#[cfg(test)]
mod test {
    use super::Project;
    use serde_json;
    #[test]
    pub fn deserialize_user() {
        let _user =
            serde_json::from_str::<Project>(include_str!("../../test/data/resources/project.json"))
                .unwrap();
    }
}
