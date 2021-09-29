mod types;

use crate::client::{KoganClient, KoganRequestBuilderExt, Method};
use crate::error::Result;
use serde::Deserialize;
use std::collections::HashMap;
use types::Product;

impl KoganClient {
    pub async fn create_products(&self, products: &[Product]) -> Result<CreateProductsResponse> {
        self
            .request(Method::POST, "/products")
            .json(products)
            .send_json()
            .await
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateProductsResponse {
    pub status: CreateProductsStatus,
    pub pending_url: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductsResponseBody {
    pub errors: Vec<ProductError>,
    pub warnings: Vec<ProductWarning>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CreateProductsStatus {
    AsyncResponsePending,
    Complete,
    CompleteWithErrors,
    Failed,
}

#[derive(Debug, Deserialize)]
pub struct ProductError {
    pub product_sku: String,
    pub errors: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ProductWarning {
    pub product_sku: String,
    pub warnings: HashMap<String, Vec<String>>,
}
