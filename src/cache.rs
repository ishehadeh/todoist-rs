use super::{Client, Collaborator, Project, Item, Label, User, ID, ResourceType};
use super::types::Error;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Cache {
    /// The token returned on the last sync request
    sync_token : String,

    pub user : User,
    pub labels : HashMap<ID, Label>,
    pub projects : HashMap<ID, Project>,
    pub items : HashMap<ID, Item>,
    pub collaborators : HashMap<ID, Collaborator>,
}

impl Cache {
    pub fn sync(&mut self, client : &mut Client) -> Result<(), Error> {
        let resp = client.sync(&[ResourceType::Projects,
                                 ResourceType::Items,
                                 ResourceType::User,
                                 ResourceType::Collaborators,
                                 ResourceType::Labels])?;
        match resp.user {
            Some(v) => self.user = v,
            None => (),
        };
        for project in resp.projects.unwrap() {
            self.projects.insert(project.id, project);
        }

        for item in resp.items.unwrap() {
            self.items.insert(item.id, item);
        }

        for collaborator in resp.collaborators.unwrap() {
            self.collaborators.insert(collaborator.id, collaborator);
        }

        for label in resp.labels.unwrap() {
            self.labels.insert(label.id, label);
        }
        self.sync_token = resp.sync_token.clone();
        Ok(())
    }
}
