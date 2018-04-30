pub mod project;
pub mod item;
pub mod label;
pub mod filter;
pub mod note;

use uuid::Uuid;
use types::ID;
use erased_serde;

pub enum MoveTargetType {
    Project,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
/// Identity is used to identify Todoist objects, 
/// it's typically used as arguments to a command, where a single action
/// is performed on an object. (e.g. delete)
pub struct Identity {
    pub ids : Vec<ID>,
}


#[derive(Serialize)]
pub struct Command {
    #[serde(rename = "type")]
    pub typ     : String,
    pub args    : Box<erased_serde::Serialize>,
    pub uuid    : Uuid,
    pub temp_id : Option<Uuid>, 
}

pub trait Update {
    fn update(self) -> Command;
}

pub trait Complete {
    fn complete(self) -> Command;
    fn uncomplete(self) -> Command;
}

pub trait Delete {
    fn delete(self) -> Command;
}

pub trait Create {
    fn create(self) -> Command;
}


pub trait Close {
    fn close(self) -> Command;
}

pub trait MoveObject {
    fn move_object(self, &MoveTarget) -> Command;
}

pub trait MoveTarget {
    fn target_type(&self) -> MoveTargetType;
    fn target_id(&self) -> ID;
}

pub trait Archive {
    fn archive(self) -> Command;
    fn unarchive(self) -> Command;
}
