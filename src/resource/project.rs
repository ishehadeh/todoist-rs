//! Project related structures
use types::*;

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
