use types::*;
use command;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]

/// A Todoist label (premium users only)
pub struct Label {
    /// The label's ID
    pub id : ID,

    /// The label's name
    pub name : String,

    /// The label's Color
    pub color : Color,

    /// This label's position in the label list, the smallest number should be at the top
    pub item_order : isize,

    // 1 if this label has been marked as deleted
    pub is_deleted : IntBool,
    // 1 if this label has been marked as a favorite
    pub is_favorite : IntBool,
}


impl command::Create for Label {
    fn create(self) -> command::Command {
        command::Command {
            typ: "label_add".to_string(),
            args: Box::new(
                command::label::Create {
                    name:        self.name,
                    color:       self.color,
                    item_order:  self.item_order,
                    is_favorite: self.is_favorite,
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: Some(Uuid::new_v4()),
        }
    }
}

impl command::Update for Label {
    fn update(self) -> command::Command {
        command::Command {
            typ: "label_update".to_string(),
            args: Box::new(
                command::label::Update {
                    name:        self.name,
                    id:          self.id,
                    color:       self.color,
                    item_order:  self.item_order,
                    is_favorite: self.is_favorite,
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}

impl command::Delete for Label {
    fn delete(self) -> command::Command {
        command::Command {
            typ: "label_delete".to_string(),
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