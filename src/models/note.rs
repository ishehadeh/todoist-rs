use serde;

use std::collections::BTreeMap;
use std::fmt;
use types::*;

use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UploadState {
    #[serde(rename = "pending")]
    Pending,

    #[serde(rename = "completed")]
    Completed,
}

#[derive(Debug, Clone)]
pub struct Thumbnail {
    link: String,
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
/// A Todoist note
pub struct Note {
    /// The note's unique ID
    pub id: String,

    pub posted_uid: String,

    /// The ID of the item this note is attached to
    pub item_id: String,

    /// The note's text, may be formatted as markdown
    pub content: String,

    /// the file attached to this note
    pub file_attachment: Attachment,

    /// List of user ids to notify
    pub uids_to_notify: Option<Vec<String>>,

    /// true if this note is marked as deleted
    pub is_deleted: bool,

    /// the date that this note was posted
    pub posted_at: chrono::DateTime<chrono::Utc>,

    /// Map of emoji reactions to the user ID who reacted with that emoji
    pub reactions: Option<BTreeMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
/// A Todoist note, attached to a project instead of an item
pub struct ProjectNote {
    /// The note's unique ID
    pub id: String,

    pub posted_uid: String,

    /// The ID of the project the note is attached to
    pub project_id: String,

    /// The note's text, may be formatted as markdown
    pub content: String,

    /// the file attached to this note
    pub file_attachment: Attachment,

    /// List of user ids to notify
    pub uids_to_notify: Option<Vec<String>>,

    /// true if this note is marked as deleted
    pub is_deleted: bool,

    /// the date that this note was posted
    pub posted_at: chrono::DateTime<chrono::Utc>,

    /// Map of emoji reactions to the user ID who reacted with that emoji
    pub reactions: Option<BTreeMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]

/// A file attachment
// TODO: move Attachment into its own module
pub struct Attachment {
    /// The attachment's name
    pub file_name: String,

    /// The file's size
    pub file_size: usize,

    /// the files MIME type
    pub file_type: String,

    /// the url where this file can be found
    pub file_url: String,

    /// the file's upload state (pending or complete)
    pub upload_state: UploadState,

    /// small thumbnail
    pub tn_s: Option<Thumbnail>,

    /// medium thumbnail
    pub tn_m: Option<Thumbnail>,

    /// large thumbnail
    pub tn_l: Option<Thumbnail>,
}

impl<'de> Deserialize<'de> for Thumbnail {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Thumbnail, D::Error> {
        struct ThumbnailVisitor;

        impl<'de> serde::de::Visitor<'de> for ThumbnailVisitor {
            type Value = Thumbnail;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array in the form [string, usize, usize]")
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut value: A,
            ) -> Result<Self::Value, A::Error> {
                let mut x = Thumbnail {
                    link: "".to_string(),
                    width: 0,
                    height: 0,
                };

                x.link = value.next_element()?.unwrap_or("".to_string());
                x.width = value.next_element()?.unwrap_or(0);
                x.height = value.next_element()?.unwrap_or(0);
                Ok(x)
            }
        }

        deserializer.deserialize_seq(ThumbnailVisitor {})
    }
}

impl Serialize for Thumbnail {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.link)?;
        seq.serialize_element(&self.width)?;
        seq.serialize_element(&self.height)?;
        seq.end()
    }
}

#[cfg(test)]
mod test {
    use super::{Note, ProjectNote};
    use serde_json;

    #[test]
    pub fn deserialize_note() {
        let _user =
            serde_json::from_str::<Note>(include_str!("../../test/data/resources/note.json"))
                .unwrap();
    }

    #[test]
    pub fn deserialize_project_note() {
        let _user = serde_json::from_str::<ProjectNote>(include_str!(
            "../../test/data/resources/project_note.json"
        ))
        .unwrap();
    }
}
