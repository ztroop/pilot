pub mod api;
pub mod structs;

use async_trait::async_trait;

use crate::{PilotComment, PilotHandle, PilotRequest};

use self::{
    api::GitLabWebAPI,
    structs::{note::Note, request::MergeRequest},
};

pub struct GitLab {
    api: GitLabWebAPI,
}

impl GitLab {
    pub fn new(url: &str) -> Self {
        GitLab {
            api: GitLabWebAPI::new(url).unwrap(),
        }
    }
}

#[async_trait]
impl PilotHandle for GitLab {
    async fn get_requests(&self) -> Vec<PilotRequest> {
        let requests = self.api.get_merge_requests().await;
        match requests {
            Err(error) => panic!("Error: {:?}", error),
            Ok(data) => {
                let mut reviews = data
                    .into_iter()
                    .filter(|x| x.state == "opened")
                    .map(to_pilot_review)
                    .collect::<Vec<PilotRequest>>();
                reviews.sort_by_key(|d| d.created_at.clone());

                reviews
            }
        }
    }

    async fn get_comments(&self, request: &PilotRequest) -> Vec<PilotComment> {
        let notes = self
            .api
            .get_merge_request_notes(request.project_id, request.request_id)
            .await;
        match notes {
            Err(error) => panic!("Error: {:?}", error),
            Ok(data) => data
                .into_iter()
                .map(to_pilot_notes)
                .collect::<Vec<PilotComment>>(),
        }
    }
}

fn to_pilot_review(r: MergeRequest) -> PilotRequest {
    PilotRequest {
        request_id: r.id,
        request_title: r.title,
        project_id: r.project_id,
        project_name: r.references.full.split('!').next().unwrap().to_string(),
        conflicts: r.has_conflicts,
        web_url: r.web_url,
        author: r.author.username,
        avatar: r.author.avatar_url,
        created_at: r.created_at,
        commit_hash: r.sha,
    }
}

fn to_pilot_notes(n: Note) -> PilotComment {
    PilotComment {
        body: n.body,
        author: n.author.username,
        avatar: n.author.avatar_url,
        created_at: n.created_at,
    }
}
