use serde;

use std::fmt;
use types::*;
use command;
use uuid::Uuid;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeSeq;

#[derive(Serialize, Deserialize, Debug)]
pub enum UploadState {
    #[serde(rename = "pending")]
    Pending,

    #[serde(rename = "completed")]
    Completed,
}


#[derive(Debug)]
pub struct Thumbnail {
    link : String,
    width : usize,
    height : usize,
}


#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]

/// A Todoist note
pub struct Note {
    /// The note's unique ID
    pub id : ID,

    /// The ID of the note's poster
    pub user_id : ID,

    /// The ID of the item the note is attached to
    pub item_id : ID,

    /// The ID of the project this note is a part of
    pub project_id : ID,

    /// The note's text
    pub content : String,

    /// the file attached to this note
    pub file_attachment : Attachment,

    /// List of user ids to notify
    pub uids_to_notify : Vec<ID>,

    /// whether this note is marked as deleted
    pub is_deleted : IntBool,

    /// whether this note has been marked as archived
    pub is_archived : IntBool,

    /// the date that this note was posted
    pub posting : Date,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]

/// A file attachment
pub struct Attachment {
    /// The attachment's name
    pub file_name : String,

    /// The file's size
    pub file_size : usize,

    /// the files MIME type
    pub file_type : String,

    /// the url where this file can be found
    pub file_url : String,

    /// the file's upload state (pending or complete)
    pub upload_state : UploadState,

    /// small thumbnail
    pub tn_s : Option<Thumbnail>,

    /// medium thumbnail
    pub tn_m : Option<Thumbnail>,

    /// large thumbnail
    pub tn_l : Option<Thumbnail>,
}


pub struct ProjectNote(Note);

impl Note {
    fn project(self) -> ProjectNote {
        ProjectNote(self)
    }
}

impl<'de> Deserialize<'de> for Thumbnail {
    fn deserialize<D : Deserializer<'de>>(deserializer: D) -> Result<Thumbnail, D::Error> {
        struct ThumbnailVisitor;

        impl<'de> serde::de::Visitor<'de> for ThumbnailVisitor {
            type Value = Thumbnail;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array in the form [string, usize, usize]")
            }

            fn visit_seq<A : serde::de::SeqAccess<'de> >(self, mut value: A) -> Result<Self::Value, A::Error>    {
                let mut x = Thumbnail{
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

        deserializer.deserialize_seq(ThumbnailVisitor{})
    }
}

impl Default for UploadState {
    fn default() -> UploadState {
        UploadState::Completed
    }
}

impl Serialize for Thumbnail {
    fn serialize<S : Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.link)?;
        seq.serialize_element(&self.width)?;
        seq.serialize_element(&self.height)?;
        seq.end()
    }
}

impl command::Create for ProjectNote {
    fn create(self) -> command::Command {
        command::Command {
            typ: "note_add".to_string(),
            args: Box::new(
                command::note::Create {
                    project_id      : Some(self.0.project_id),
                    item_id         : None,
                    content         : self.0.content,
                    file_attachment : Some(self.0.file_attachment),
                    uids_to_notify  : None,
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}

impl command::Create for Note {
    fn create(self) -> command::Command {
        command::Command {
            typ: "note_add".to_string(),
            args: Box::new(
                command::note::Create {
                    item_id         : Some(self.project_id),
                    project_id      : None,
                    content         : self.content,
                    file_attachment : Some(self.file_attachment),
                    uids_to_notify  : Some(self.uids_to_notify),
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}

impl command::Update for Note {
    fn update(self) -> command::Command {
        command::Command {
            typ: "note_update".to_string(),
            args: Box::new(
                command::note::Update {
                    content         : self.content,
                    id              : self.id,
                    file_attachment : Some(self.file_attachment),
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}

impl command::Delete for Note {
    fn delete(self) -> command::Command {
        command::Command {
            typ: "note_delete".to_string(),
            args: Box::new(
                command::Identity {
                    ids: vec![self.id],
                }
            ),
            uuid:    Uuid::new_v4(),
            temp_id: None,
        }
    }
}