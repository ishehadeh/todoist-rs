use types::*;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Create {
    pub name : String,
    pub color : Color,
    pub item_order : isize,
    pub is_favorite : IntBool,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Update {
    pub id   : ID,
    pub name : String,
    pub color : Color,
    pub item_order : isize,
    pub is_favorite : IntBool,
}