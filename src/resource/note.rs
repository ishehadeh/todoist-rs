use serde;

use command;
use std::fmt;
use types::*;
use uuid::Uuid;

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

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]

/// A Todoist note
pub struct Note {
    /// The note's unique ID
    pub id: ID,

    /// The ID of the note's poster
    pub user_id: ID,

    /// The ID of the note the note is attached to
    pub item_id: ID,

    /// The ID of the project this note is a part of
    pub project_id: ID,

    /// The note's text
    pub content: String,

    /// the file attached to this note
    pub file_attachment: Attachment,

    /// List of user ids to notify
    pub uids_to_notify: Vec<ID>,

    /// whether this note is marked as deleted
    pub is_deleted: isize,

    /// whether this note has been marked as archived
    pub is_archived: isize,

    /// the date that this note was posted
    pub posting: Date,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]

/// A file attachment
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

pub struct ProjectNote(Note);

impl Note {
    fn project(self) -> ProjectNote {
        ProjectNote(self)
    }
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

impl Default for UploadState {
    fn default() -> UploadState {
        UploadState::Completed
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

impl Note {
    pub fn add() -> command::note::Add {
        command::note::Add::default()
    }

    pub fn update(&self) -> command::note::Update {
        command::note::Update {
            id: self.id,
            content: self.content.clone(),
            file_attachment: Some(self.file_attachment.clone()),
        }
    }

    pub fn delete(&self) -> command::note::Delete {
        command::note::Delete { ids: vec![self.id] }
    }
}
