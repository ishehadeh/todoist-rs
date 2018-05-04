use super::{Client, Collaborator, Project, Item, Label, User, ID, ResourceType, Transaction};
use super::types::Error;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Cache {
    /// The user's api token
    pub token : Option<String>,

    /// The token returned on the last sync request
    pub sync_token : Option<String>,

    pub user : User,
    pub labels : HashMap<ID, Label>,
    pub projects : HashMap<ID, Project>,
    pub items : HashMap<ID, Item>,
    pub collaborators : HashMap<ID, Collaborator>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            token: None,
            sync_token: None,
            user: User::default(),
            labels: HashMap::new(),
            projects: HashMap::new(),
            items: HashMap::new(),
            collaborators: HashMap::new(),
        }
    }

    pub fn create_client(&self) -> Result<Client, Error> {
        match self.token {
            Some(ref v) => Ok(Client::new(v)),
            None => Err(Error::InvalidApiToken("<None>".to_string()))
        }
    }

    pub fn sync(&mut self, client: &Client) -> Result<(), Error> {
        let sync_tok = self.sync_token.clone().unwrap_or("*".to_string());
        let resp = client.sync(&sync_tok,
                                  &[ResourceType::Projects,
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
        self.sync_token = Some(resp.sync_token.clone());
        Ok(())
    }
}
