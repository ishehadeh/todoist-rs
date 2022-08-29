use super::Color;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]

/// A Todoist filter
pub struct Filter {
    /// The filter's unique ID
    pub id: String,

    /// The filter name
    pub name: String,

    /// The filter query
    pub query: String,

    /// The filter's color
    pub color: Color,

    /// The filter's place in the filter list (lowest is first)
    pub item_order: isize,

    /// whether this filter is marked as deleted
    pub is_deleted: bool,

    /// whether this filter is marked as a favorite
    pub is_favorite: bool,
}

#[cfg(test)]
mod test {
    use super::Filter;
    use serde_json;
    #[test]
    pub fn deserialize_filter() {
        let _ =
            serde_json::from_str::<Filter>(include_str!("../../test/data/resources/filter.json"))
                .unwrap();
    }
}
