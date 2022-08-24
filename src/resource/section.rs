use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Section {
    pub id: String,
    pub name: String,
    pub project_id: String,

    /// The place of this section among its siblings
    pub section_order: i32,

    /// Undocumented, I assume its the ID of the user who created the section.
    pub user_id: String,

    /// True if the sections child tasks are hidden
    pub collapsed: bool,

    /// ID used to identify this section between users in shared proejcts, `id` is per-user.
    pub sync_id: Option<String>,

    pub is_deleted: bool,

    pub is_archived: bool,

    pub archived_at: Option<DateTime<Utc>>,

    pub added_at: DateTime<Utc>,
}

#[cfg(test)]
mod test {
    use super::Section;
    use serde_json;
    #[test]
    pub fn deserialize_section() {
        let _ =
            serde_json::from_str::<Section>(include_str!("../../test/data/resources/section.json"))
                .unwrap();
    }
}
