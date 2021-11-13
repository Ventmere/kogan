mod types;
pub use self::types::*;

use crate::client::{KoganClient, KoganRequestBuilderExt, Method};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl KoganClient {
    pub async fn create_products(&self, products: &[Product]) -> Result<CreateProductsResponse> {
        self
            .request(Method::POST, "/products")
            .json(products)
            .send_json()
            .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductsResponse {
    pub status: CreateProductsStatus,
    pub pending_url: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductsResponseBody {
    pub errors: Vec<ProductError>,
    pub warnings: Vec<ProductWarning>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CreateProductsStatus {
    AsyncResponsePending,
    Complete,
    CompleteWithErrors,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductError {
    pub product_sku: String,
    pub errors: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductWarning {
    pub product_sku: String,
    pub warnings: HashMap<String, Vec<String>>,
}
