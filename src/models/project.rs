//! Project related structures
use super::Color;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViewStyle {
    #[serde(rename = "list")]
    List,
    #[serde(rename = "board")]
    Board,
}

const fn get_false() -> bool {
    false
}

#[derive(Deserialize, Serialize, Debug, Clone)]

/// A Todoist Project
pub struct Project {
    /// The project's unique ID
    pub id: String,

    /// The project's name
    pub name: String,

    /// The project's Color
    pub color: Color,

    pub parent_id: Option<String>,

    /// Defines the position of this project within the parent project
    pub child_order: i32,

    /// Whether this project's child's project's children are visible
    pub collapsed: bool,

    pub shared: bool,

    // 1 if this project has been marked as deleted
    pub is_deleted: bool,

    // 1 if this project has been marked as archived
    pub is_archived: bool,

    // 1 if this project has been marked as a favorite
    pub is_favorite: bool,

    /// Shared projects get a unique ID for each user, `sync_id` is the same across all instances of the project.
    /// `None` if the project is not shared.
    pub sync_id: Option<String>,

    /// True if this project is in the user's inbox
    #[serde(default = "get_false")]
    pub inbox_project: bool,

    /// True if this project is in the team's inbox
    #[serde(default = "get_false")]
    pub team_inbox: bool,

    pub view_style: ViewStyle,
}

#[cfg(test)]
mod test {
    use super::Project;
    use serde_json;
    #[test]
    pub fn deserialize_project() {
        let _user =
            serde_json::from_str::<Project>(include_str!("../../test/data/resources/project.json"))
                .unwrap();
    }
}
