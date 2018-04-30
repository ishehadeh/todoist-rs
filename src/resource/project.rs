//! Project related structures
use types::*;
use uuid::Uuid;

use command;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
#[serde(default)]

/// A Todoist Project
pub struct Project {
    /// The project's unique ID
    pub id    : ID,

    /// The project's name
    pub name  : String,

    /// The project's Color
    pub color : Color,

    /// The project's indent (hierarchy level) is a number between from 1-4
    pub indent : u8,

    /// This project's position in the project list, the smallest number should be at the top
    pub item_order : isize,

    /// Whether this project's child's project's children are visible
    pub collapsed : IntBool,

    pub shared : bool,

    // 1 if this project has been marked as deleted
    pub is_deleted : IntBool,

    // 1 if this project has been marked as archived
    pub is_archived :IntBool,

    // 1 if this project has been marked as a favorite
    pub is_favorite : IntBool,

    /// True if this project is in the user's inbox
    pub inbox : bool,

    /// True if this project is in the team's inbox
    pub inbox_team : bool,
}

impl command::Create for Project {
    fn create(self) -> command::Command {
        command::Command {
            typ: "project_add".to_string(),
            args: Box::new(
                command::project::Create {
                    name:        self.name,
                    color:       self.color,
                    indent:      self.indent,
                    item_order:  self.item_order,
                    is_favorite: self.is_favorite,
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: Some(Uuid::new_v4()),
        }
    }
}

impl command::Update for Project {
    fn update(self) -> command::Command {
        command::Command {
            typ: "project_update".to_string(),
            args: Box::new(
                command::project::Update {
                    name:        self.name,
                    id:          self.id,
                    color:       self.color,
                    indent:      self.indent,
                    item_order:  self.item_order,
                    collapsed:   self.collapsed,
                    is_favorite: self.is_favorite,
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}

impl command::Delete for Project {
    fn delete(self) -> command::Command {
        command::Command {
            typ: "project_delete".to_string(),
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


impl command::Archive for Project {
    fn archive(self) -> command::Command {
        command::Command {
            typ: "project_archive".to_string(),
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
            typ: "project_unarchive".to_string(),
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
