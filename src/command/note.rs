use types::*;
use resource::Attachment;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Create {
    pub item_id         : Option<ID>,
    pub project_id      : Option<ID>,
    pub content         : String,
    pub file_attachment : Option<Attachment>,
    pub uids_to_notify  : Option<Vec<ID>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Update {
    pub id         : ID,
    pub content         : String,
    pub file_attachment : Option<Attachment>,
}