use std::fmt;
use types::*;
use command;
use uuid::Uuid;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeSeq;

#[derive(Serialize, Deserialize, Default, Debug)]
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
    pub is_deleted : IntBool,

    /// whether this filter is marked as a favorite
    pub is_favorite : IntBool,
}

impl command::Create for Filter {
    fn create(self) -> command::Command {
        command::Command {
            typ: "note_add".to_string(),
            args: Box::new(
                command::filter::Create {
                    name: self.name,
                    query: self.query,
                    color: self.color,
                    item_order: self.order,
                    is_favorite: self.is_favorite,
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}

impl command::Update for Filter {
    fn update(self) -> command::Command {
        command::Command {
            typ: "note_update".to_string(),
            args: Box::new(
                command::filter::Update {
                    id: self.id,
                    name: self.name,
                    query: self.query,
                    color: self.color,
                    item_order: self.order,
                    is_favorite: self.is_favorite,
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}

impl command::Delete for Filter {
    fn delete(self) -> command::Command {
        command::Command {
            typ: "note_delete".to_string(),
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