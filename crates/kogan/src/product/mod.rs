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

    pub async fn get_products(&self, filter: Option<GetProductsFilters>) -> Result<GetProductsResponse> {
        if let Some(filter) = filter {
            self
                .request(Method::GET, "/products")
                .json(&filter)
                .send_json()
                .await
        } else {
            self
                .request(Method::GET, "/products")
                .send_json()
                .await
        }
    }

    pub async fn get_products_next(&self, next: String) -> Result<GetProductsResponse> {
        self
        .request(Method::GET, &next)
        .send_json()
        .await
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GetProductsFilters {
    pub search: Option<String>,
    pub sku: Option<String>,
    pub enabled: Option<bool>,
    pub category: Option<String>,
    pub brand: Option<String>,
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub detail: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetProductsResponse {
    pub body: GetProductsBody,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProductsBody {
    pub next: Option<String>,
    pub results: Vec<Product>,
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
