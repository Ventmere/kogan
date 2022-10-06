use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub product_title: String,
    pub product_sku: String,
    pub product_subtitle: String,
    pub product_gtin: String,
    pub images: Vec<String>,
    pub brand: String,
    pub category: String,
    pub offer_data: HashMap<String, OfferData>,
    pub stock: i64,
    pub enabled: bool,
    pub created: String,
    pub store_urls: Vec<StoreUrl>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferData {
    pub price: String,
    pub kogan_first_price: Option<String>,
    pub tax_exempt: bool,
    pub shipping: String,
    pub handling_days: i64,
    pub rrp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDimensions {
    pub length: i64,
    pub width: i64,
    pub height: i64,
    pub weight: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreUrl {
    pub url: String,
    pub store_name: String,
    pub organisation: String,
}
