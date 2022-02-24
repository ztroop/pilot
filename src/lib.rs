pub mod github;
pub mod gitlab;

use async_trait::async_trait;

use self::gitlab::GitLab;

pub enum VCS {
    GitLab,
    GitHub,
}

pub struct Pilot {
    vcs: Box<dyn PilotHandle>,
}

#[async_trait]
pub trait PilotHandle {
    async fn get_requests(&self) -> Vec<PilotRequest>;
    async fn get_comments(&self, request: &PilotRequest) -> Vec<PilotComment>;
}

impl Pilot {
    pub fn new(url: &str, vcs: VCS) -> Self {
        match vcs {
            VCS::GitLab => Self {
                vcs: Box::new(GitLab::new(url)),
            },
            _ => unimplemented!("Not supported yet!"),
        }
    }

    pub async fn requests(&self) -> Vec<PilotRequest> {
        self.vcs.get_requests().await
    }

    pub async fn comments(&self, pilot_request: &PilotRequest) -> Vec<PilotComment> {
        self.vcs.get_comments(pilot_request).await
    }
}

#[derive(Debug)]
pub struct PilotRequest {
    pub request_id: i64,
    pub project_id: i64,
    pub project_name: String,
    pub request_title: String,
    pub created_at: String,
    pub author: String,
    pub avatar: String,
    pub web_url: String,
    pub conflicts: bool,
    pub commit_hash: String,
}

#[derive(Debug)]
pub struct PilotComment {
    pub body: String,
    pub author: String,
    pub avatar: String,
    pub created_at: String,
}
