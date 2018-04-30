use serde;

use std::fmt;
use types::*;
use command;
use uuid::Uuid;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeSeq;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]

/// A Todoist task item
pub struct Item {
    /// The item's unique ID
    pub id : ID,

    /// The ID of the item's owner
    pub user_id : ID,

    /// The ID of the project this item belongs to 
    pub project_id : ID,

    /// This item's text, not (e.g. "Do the dishes")
    pub content : Option<String>,

    /// The date for this TODO
    /// examples: "every other day", "tomorrow", "today at 9am"
    pub date_string : Option<String>,

    /// The date_string's language
    pub date_lang : Option<Language>,

    /// The date this TODO is due, or none 
    pub due_date_utc : Option<Date>,

    /// this item's importance
    pub priority : Priority,

    /// this item's indent
    pub indent : u8,

    /// This item's position in the item list, the smallest number should be at the top
    pub item_order : isize,

    /// This item's position in the "Today" or "Next 7 Days" list, the smallest number should be at the top
    pub day_order : isize,

    /// 0 if this item's children should be hidden
    pub collapsed : isize,

    /// a list of label id's for the labels attached to this item
    pub labels : Vec<ID>,

    /// The user ID of the user who added this item
    pub assigned_by_uid : Option<ID>,

    /// The user ID of the user who is assigned this task
    pub responsible_uid : Option<ID>,

    /// 1 if this task has been completed
    pub checked : IntBool,

    // 1 if this item has been marked as as completely completed (all child tasks have also been completed)
    pub in_history : IntBool,

    // 1 if this item has been marked as deleted
    pub is_deleted : IntBool,

    // 1 if this item has been marked as archived
    pub is_archived : IntBool,

    // 1 if this item has been marked as a favorite
    pub is_favorite : IntBool,

    /// used internally by Todoist, here for completeness 
    pub sync_id : Option<isize>,

    /// when this item was added
    pub date_added : Option<Date>,

    /// if true this item will use the user's default reminder (this field is CREATE only)
    pub auto_reminders : Option<bool>,

    /// if true this item's labels will be parsed from the content field (this field is CREATE only)
    pub auto_parse_labels : Option<bool>,
}



impl command::Create for Item {
    fn create(self) -> command::Command {
        command::Command {
            typ: "item_add".to_string(),
            args: Box::new(
                command::item::Create {
                    content : self.content,
                    project_id : self.project_id,
                    date_string : self.date_string,
                    date_lang : self.date_lang,
                    due_date_utc : self.due_date_utc,
                    priority : self.priority,
                    indent : self.indent,
                    item_order : self.item_order,
                    day_order : self.day_order,
                    collapsed : self.collapsed,
                    labels : self.labels,
                    assigned_by_uid : self.assigned_by_uid,
                    auto_parse_labels : self.auto_parse_labels,
                    auto_reminders : self.auto_reminders,
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: Some(Uuid::new_v4()),
        }
    }
}

impl command::Update for Item {
    fn update(self) -> command::Command {
        command::Command {
            typ: "item_update".to_string(),
            args: Box::new(
                command::item::Update {
                    id : self.id,
                    content : self.content,
                    date_string : self.date_string,
                    date_lang : self.date_lang,
                    due_date_utc : self.due_date_utc,
                    priority : self.priority,
                    indent : self.indent,
                    item_order : self.item_order,
                    day_order : self.day_order,
                    collapsed : self.collapsed,
                    labels : self.labels,
                    assigned_by_uid : self.assigned_by_uid,
                    responsible_uid : self.responsible_uid,
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}

impl command::Delete for Item {
    fn delete(self) -> command::Command {
        command::Command {
            typ: "item_delete".to_string(),
            args: Box::new(
                command::Identity {
                    ids: vec![self.id],
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}


impl command::Archive for Item {
    fn archive(self) -> command::Command {
        command::Command {
            typ: "item_archive".to_string(),
            args: Box::new(
                command::Identity {
                    ids: vec![self.id],
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }

    fn unarchive(self) -> command::Command {
        command::Command {
            typ: "item_unarchive".to_string(),
            args: Box::new(
                command::Identity {
                    ids: vec![self.id],
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}

impl command::Close for Item {
    fn close(self) -> command::Command {
        command::Command {
            typ: "item_close".to_string(),
            args: Box::new(
                command::Identity {
                    ids: vec![self.id],
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}

impl command::Complete for Item {
    fn complete(self) -> command::Command {
        command::Command {
            typ: "item_complete".to_string(),
            args: Box::new(
                command::Identity {
                    ids: vec![self.id],
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }

    fn uncomplete(self) -> command::Command {
        command::Command {
            typ: "item_uncomplete".to_string(),
            args: Box::new(
                command::Identity {
                    ids: vec![self.id],
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}