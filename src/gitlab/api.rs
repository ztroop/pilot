use std::env;
use std::{error, time::Duration};

use surf::{Client, Config, Url};

use super::structs::note::{Note, Position};
use super::structs::request::MergeRequest;

const API_VERSION: &str = "api/v4";

pub struct GitLabWebAPI {
    pub client: Client,
    pub base_url: String,
    token: String,
}

impl GitLabWebAPI {
    pub fn new(url: &str) -> Result<Self, Box<dyn error::Error>> {
        let base_url = format!("{}/{}/", url, API_VERSION);
        let client = Config::new()
            .set_base_url(Url::parse(&base_url)?)
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()?;

        Ok(GitLabWebAPI {
            client,
            token: env::var("GITLAB_TOKEN").unwrap(),
            base_url,
        })
    }

    pub async fn get_merge_requests(&self) -> Result<Vec<MergeRequest>, Box<dyn error::Error>> {
        let mut res = self
            .client
            .get(format!("{}/merge_requests", &self.base_url))
            .header("Content-Type", "application/json")
            .header("PRIVATE-TOKEN", &self.token)
            .await?;

        let result: Vec<MergeRequest> = res.body_json().await?;
        Ok(result)
    }

    pub async fn get_merge_request_notes(
        &self,
        project_id: i64,
        merge_request_id: i64,
    ) -> Result<Vec<Note>, Box<dyn error::Error>> {
        let mut res = self
            .client
            .get(format!(
                "{}/projects/{}/merge_requests/{}/notes",
                &self.base_url, project_id, merge_request_id
            ))
            .header("Content-Type", "application/json")
            .header("PRIVATE-TOKEN", &self.token)
            .await?;

        let result: Vec<Note> = res.body_json().await?;
        Ok(result)
    }

    pub async fn get_raw_file(
        &self,
        project_id: i64,
        position: Position,
    ) -> Result<String, Box<dyn error::Error>> {
        let mut res = self
            .client
            .get(format!(
                "{}/projects/{}/repository/files/{}/raw?ref={}",
                &self.base_url, project_id, position.new_path, position.head_sha
            ))
            .header("Content-Type", "application/json")
            .header("PRIVATE-TOKEN", &self.token)
            .await?;

        let result: String = res.body_string().await?;
        Ok(result)
    }
}
