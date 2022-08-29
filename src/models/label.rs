use super::Color;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]

/// A Todoist label (premium users only)
pub struct Label {
    /// The label's ID
    pub id: String,

    /// The label's name
    pub name: String,

    /// The label's Color
    pub color: Color,

    /// This label's position in the label list, the smallest number should be at the top
    pub item_order: isize,

    // true if this label has been marked as deleted
    pub is_deleted: bool,

    // true if this label has been marked as a favorite
    pub is_favorite: bool,
}

#[cfg(test)]
mod test {
    use super::Label;
    use serde_json;

    #[test]
    pub fn deserialize_label() {
        let _user =
            serde_json::from_str::<Label>(include_str!("../../test/data/resources/label.json"))
                .unwrap();
    }
}
