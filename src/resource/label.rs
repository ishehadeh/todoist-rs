use types::*;

#[derive(Serialize, Deserialize, Debug, Clone)]

/// A Todoist label (premium users only)
pub struct Label {
    /// The label's ID
    pub id: ID,

    /// The label's name
    pub name: String,

    /// The label's Color
    pub color: Color,

    /// This label's position in the label list, the smallest number should be at the top
    pub item_order: isize,

    // 1 if this label has been marked as deleted
    pub is_deleted: isize,
    // 1 if this label has been marked as a favorite
    pub is_favorite: isize,
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
