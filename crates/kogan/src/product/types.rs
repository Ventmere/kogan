use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    product_title: String,
    product_sku: String,
    product_subtitle: String,
    product_gtin: String,
    images: Vec<String>,
    brand: String,
    category: String,
    offer_data: HashMap<String, OfferData>,
    stock: i64,
    enabled: bool,
    created: String,
    store_urls: Vec<StoreUrl>,
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
