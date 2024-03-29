use crate::client::{KoganClient, KoganRequestBuilderExt, Method};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl KoganClient {
    pub async fn get_task_results(&self, task_id: &str) -> Result<GetTaskResultsResponse> {
        self.request(Method::GET, &format!("/task/{}/", task_id)).send_json().await
    }

    pub async fn get_task_results_url(&self, url: &str) -> Result<GetTaskResultsResponse> {
        self.request(Method::GET, url).send_json().await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTaskResultsResponse {
    pub status: GetTaskResultsStatus,
    pub pending_url: Option<String>,
    pub body: Value,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GetTaskResultsStatus {
    AsyncResponsePending,
    Complete,
    CompleteWithErrors,
    Failed,
}
