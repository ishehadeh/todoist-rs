use types::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Create {
    pub project_id : ID,
    pub content : Option<String>,
    pub date_string : Option<String>,
    pub date_lang : Option<Language>,
    pub due_date_utc : Option<Date>,
    pub priority : Priority,
    pub indent : u8,
    pub item_order : isize,
    pub day_order : isize,
    pub collapsed : isize,
    pub labels : Vec<ID>,
    pub assigned_by_uid : Option<ID>,
    pub auto_reminders : Option<bool>,
    pub auto_parse_labels : Option<bool>,

}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Update {
    pub id : ID,
    pub content : Option<String>,
    pub date_string : Option<String>,
    pub date_lang : Option<Language>,
    pub due_date_utc : Option<Date>,
    pub priority : Priority,
    pub indent : u8,
    pub item_order : isize,
    pub day_order : isize,
    pub collapsed : isize,
    pub labels : Vec<ID>,
    pub assigned_by_uid : Option<ID>,
    pub responsible_uid : Option<ID>,
}


#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Move {
    pub project_items : HashMap<ID, ID>,
    pub to_project    : ID,
}
