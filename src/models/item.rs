use chrono::Utc;
use types::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
/// A Todoist task item
pub struct Item {
    /// The item's unique ID
    pub id: String,

    /// The ID of the item's owner
    pub user_id: String,

    /// The ID of the project this item belongs to
    pub project_id: String,

    /// A short description of the task (e.g. "Do the dishes")
    pub content: String,

    /// A longer description of the task
    pub description: String,

    /// A string describing when this task is due
    /// See [Due Dates](https://web.archive.org/web/20220805200101/https://developer.todoist.com/sync/v9/#clear-locations) reference for details.
    pub due: Option<String>,

    /// this item's importance
    pub priority: Priority,

    /// the parent item ID, if this item is a child
    pub parent_id: Option<String>,

    /// The position of this child among its siblings
    pub child_order: Option<isize>,

    /// The section this item belongs to if any
    pub section_id: Option<String>,

    /// if this item's children should be hidden
    pub collapsed: bool,

    /// This item's position in the "Today" or "Next 7 Days" list, the smallest number should be at the top
    pub day_order: isize,

    /// a list of label names assigned to this task
    pub labels: Vec<String>,

    /// The user ID of the user who added this item
    pub added_by_uid: Option<String>,

    /// The user ID of the user who assigned the task
    pub assigned_by_uid: Option<String>,

    /// The user ID of the user who is assigned this task
    pub responsible_uid: Option<String>,

    /// true if this task has been completed
    pub checked: bool,

    // true if this item has been marked as deleted
    pub is_deleted: bool,

    /// Universal ID for tasks within shared projects, `Item::id` is per-user
    pub sync_id: Option<String>,

    pub completed_at: Option<chrono::DateTime<Utc>>,

    pub added_at: Option<chrono::DateTime<Utc>>,
}

#[cfg(test)]
mod test {
    use super::Item;
    use serde_json;

    #[test]
    pub fn deserialize_item() {
        let _user =
            serde_json::from_str::<Item>(include_str!("../../test/data/resources/item.json"))
                .unwrap();
    }
}
