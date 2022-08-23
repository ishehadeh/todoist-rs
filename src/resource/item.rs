use command;
use types::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]

/// A Todoist task item
pub struct Item {
    /// The item's unique ID
    pub id: ID,

    /// The ID of the item's owner
    pub user_id: ID,

    /// The ID of the project this item belongs to
    pub project_id: ID,

    /// This item's text, not (e.g. "Do the dishes")
    pub content: Option<String>,

    /// The date for this TODO
    /// examples: "every other day", "tomorrow", "today at 9am"
    pub date_string: Option<String>,

    /// The date_string's language
    pub date_lang: Option<Language>,

    /// The date this TODO is due, or none
    pub due_date_utc: Option<Date>,

    /// this item's importance
    pub priority: Priority,

    /// this item's indent
    pub indent: u8,

    /// This item's position in the item list, the smallest number should be at the top
    pub item_order: isize,

    /// This item's position in the "Today" or "Next 7 Days" list, the smallest number should be at the top
    pub day_order: isize,

    /// 0 if this item's children should be hidden
    pub collapsed: isize,

    /// a list of label id's for the labels attached to this item
    pub labels: Vec<ID>,

    /// The user ID of the user who added this item
    pub assigned_by_uid: Option<ID>,

    /// The user ID of the user who is assigned this task
    pub responsible_uid: Option<ID>,

    /// 1 if this task has been completed
    pub checked: isize,

    // 1 if this item has been marked as as completely completed (all child tasks have also been completed)
    pub in_history: isize,

    // 1 if this item has been marked as deleted
    pub is_deleted: isize,

    // 1 if this item has been marked as archived
    pub is_archived: isize,

    // 1 if this item has been marked as a favorite
    pub is_favorite: isize,

    /// used internally by Todoist, here for completeness
    pub sync_id: Option<isize>,

    /// when this item was added
    pub date_added: Option<Date>,
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
