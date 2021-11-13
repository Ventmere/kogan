use crate::client::{KoganClient, KoganRequestBuilderExt, Method};
use crate::error::Result;
use serde::{Deserialize, Serialize};

impl KoganClient {
    pub async fn get_category_list(&self, next: Option<String>) -> Result<GetCategoryListResponse> {
        self.request(Method::GET, next.as_ref().map(|v| v.as_str()).unwrap_or("/category")).send_json().await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCategoryListResponse {
    pub status: GetCategoryListStatus,
    pub pending_url: Option<String>,
    pub body: GetCategoryListResponseBody,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCategoryListResponseBody {
    pub count: i64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub display: String,
    pub departments: Vec<String>,
    pub url: String,
    pub parent_company: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GetCategoryListStatus {
    Complete,
    CompleteWithErrors,
    Failed,
}
