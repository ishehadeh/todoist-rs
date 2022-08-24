#[derive(Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Section {}

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
