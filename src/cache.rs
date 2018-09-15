use super::{Client, Collaborator, Project, Item, Label, User, ID, ResourceType, Transaction};
use super::types::Error;

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
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

    pub fn Add_client(&self) -> Result<Client, Error> {
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

    /// Get a project from the cache using a file path, the path is made up of project names separated with a `/`
    /// 
    /// The path can have up to four elements (the maximum indent for a project is 4).
    /// A leading slash is not necessary, but legal, however a windows drive identifier
    /// (for example `C:\\`) is not allowed.
    pub fn get_project<'a, P : Into<PathBuf>>(&'a self, path: P) -> Option<&'a Project> {
        let mut sorted : Vec<&Project> = self.projects.iter()
                                .map(|(_, x)| x)
                                .collect();
        sorted.sort_by(|a, b| a.item_order.cmp(&b.item_order));
        
        let mut proj : Option<&Project> = None;
        for (i, proj_name) in path.into().iter().enumerate() {
            let order : isize = match proj {
                Some(v) => v.item_order,
                None => isize::min_value(),
            };

            while sorted.len() != 0 {
                if sorted[0].name != proj_name.to_string_lossy() || sorted[0].indent != i as u8 + 1 || sorted[0].item_order < order {
                    sorted.remove(0);
                } else {
                    break;
                }
            }

            if sorted.len() == 0 {
                return None;
            }

            proj = Some(sorted[0]);
        }

        proj
    }

}
