extern crate serde;
extern crate reqwest;
extern crate uuid;
extern crate chrono;
extern crate serde_json;
extern crate erased_serde;

pub mod command;
pub mod resource;

mod types;
#[cfg(test)] mod tests;

pub use types::*;
pub use resource::*;

use std::collections::HashMap;
use std::fmt;
use std::error::Error;

#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub enum Resource {
    Label(resource::Label),
    Project(resource::Project),
    Item(resource::Item),
    Note(resource::Note),
    Filter(resource::Filter),
    User(resource::User),
    Collaborator(resource::Collaborator),
    LiveNotification(resource::LiveNotification),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceType {
    #[serde(rename = "all")]
    All,

    #[serde(rename = "labels")]
    Labels,

    #[serde(rename = "projects")]
    Projects,

    #[serde(rename = "items")]
    Items,

    #[serde(rename = "notes")]
    Notes,

    #[serde(rename = "filters")]
    Filters,

    #[serde(rename = "reminders")]
    Reminders,

    #[serde(rename = "location")]
    Locations, // TODO

    #[serde(rename = "user")]
    User,

    #[serde(rename = "live_notifications")]
    LiveNotifications,

    #[serde(rename = "collaborators")]
    Collaborators,

    #[serde(rename = "notification_settings")]
    NotificationSettings, // TODO
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct CommandError {
    pub error_code : isize,
    pub error : String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CommandStatus {
    Ok(String),
    Error(CommandError),
} 

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct CommandResponse {
    pub sync_status      : HashMap<uuid::Uuid, CommandStatus>,
    pub temp_id_mappings : HashMap<uuid::Uuid, ID>,
}

#[derive(Default, Debug)]
pub struct CommandErrors {
    errors        : HashMap<uuid::Uuid, CommandError>,
    command_count : usize, 
}


#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SyncResponse {
    pub sync_token : String,
    pub full_sync : bool,
    pub items : Option<Vec<resource::Item>>,
    pub labels : Option<Vec<resource::Label>>,
    pub projects : Option<Vec<resource::Project>>,
    pub collaborators : Option<Vec<resource::Collaborator>>,
    pub notes : Option<Vec<resource::Note>>,
    pub filters : Option<Vec<resource::Filter>>,
    pub live_notifications : Option<Vec<resource::LiveNotification>>,
    pub reminders : Option<Vec<resource::Reminder>>,
    pub user : Option<resource::User>,
}

/// Client to make request to the todoist API
pub struct Client {
    token: String,
    client: reqwest::Client,
    last_sync: String,
}

/// A transactions is a batch of commands that can be sent to Todoist in a single request
/// 
/// A transaction can be initiated with Client::begin(), to update the 
pub struct Transaction<'a> {
    commands: Vec<command::Command>,
    client: &'a mut Client,
}

impl<'a> Transaction<'a> {
    pub fn create<T : command::Create>(mut self, v : T) -> Self {
        self.commands.push(v.create());
        self
    }

    pub fn delete<T : command::Delete>(mut self, v : T) -> Self {
        self.commands.push(v.delete());
        self
    }

    pub fn archive<T : command::Archive>(mut self, v : T) -> Self {
        self.commands.push(v.archive());
        self
    }

    pub fn unarchive<T : command::Archive>(mut self, v : T) -> Self {
        self.commands.push(v.unarchive());
        self
    }

    pub fn update<T : command::Update>(mut self, v : T) -> Self {
        self.commands.push(v.update());
        self
    }

    pub fn close<T : command::Close>(mut self, v : T) -> Self {
        self.commands.push(v.close());
        self
    }

    pub fn complete<T : command::Complete>(mut self, v : T) -> Self {
        self.commands.push(v.complete());
        self
    }

    pub fn uncomplete<T : command::Complete>(mut self, v : T) -> Self {
        self.commands.push(v.uncomplete());
        self
    }

    pub fn commit(self) -> Result<CommandResponse, types::Error> {
        self.client.send(self.commands.as_slice())
    }
}

impl<'a> Client {

    /// Create a new client with a Todoist API key
    pub fn new(tok: &str) -> Client {
        Client::new_with_sync(tok, "*")
    }
    
    /// create a new client with a sync token and API key
    pub fn new_with_sync(tok: &str, sync_tok: &str) -> Client {
        Client {
            client: reqwest::Client::new(),
            token: String::from(tok),
            last_sync: String::from(sync_tok),

        }
    }

    /// Request resources from todoist
    pub fn sync(&mut self, what: &[ResourceType]) -> Result<SyncResponse, types::Error> {
        let res : SyncResponse = self.client.post("http://todoist.com/api/v7/sync")
            .form(&[("token", self.token.clone()), 
                    ("sync_token", self.last_sync.clone()),
                    ("resource_types", serde_json::to_string(what)?)])
            .send()?
            .json()?;

        self.last_sync = res.sync_token.clone();
        Ok(res)
    }

    /// Send a series of commands to todoist
    /// 
    /// It is generally prettier and safer to use a transaction, instead of this command.
    /// See Client::begin()
    pub fn send(&mut self, cmd: &[command::Command]) -> Result<CommandResponse, types::Error> {
        let res : CommandResponse = self.client.post("http://todoist.com/api/v7/sync")
            .form(&[("token", self.token.clone()), 
                    ("commands", serde_json::to_string(cmd)?)])
            .send()?
            .json()?;
        CommandErrors::check_response(&res)?;
        Ok(res)
    }


    /// Begin the transaction to send a series of commands to Todoist.
    pub fn begin(&'a mut self) -> Transaction<'a> {
        Transaction {
            client: self,
            commands: Vec::new(),
        }
    }
}

impl CommandErrors {
    pub fn check_response(resp : &CommandResponse) -> Result<(), CommandErrors> {
        let errs = CommandErrors {
            command_count: resp.sync_status.len(),
            errors: resp.sync_status.iter()
                .filter(|(_, y)| 
                    match y {
                        CommandStatus::Ok(_) => false,
                        CommandStatus::Error(_) => true,
                    })
                .map(|(x, y)| match y {
                    CommandStatus::Ok(_) => unreachable!(),
                    CommandStatus::Error(e) => (x.clone(), (*e).clone()),
                })
                .collect(),
        };
        if errs.errors.len() > 0 {
            Err(errs)
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for CommandErrors {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{} commands failed: \n", self.errors.len(), self.command_count)?;
        for x in self.errors.iter() {
            write!(f, " - {}: {}", x.0, x.1)?;
        }
        Ok(())
    }
}

impl Error for CommandErrors {
    fn description(&self) -> &'static str {
        "One or more commands failed"
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "command failed (error {}): {}", self.error_code, self.error)
    }
}

impl Error for CommandError {
    fn description(&self) -> &'static str {
        "Command failed"
    }
}