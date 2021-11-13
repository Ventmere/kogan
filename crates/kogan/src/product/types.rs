use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub product_title: String,
    pub product_sku: String,
    pub product_subtitle: String,
    pub product_description: String,
    pub product_inbox: String,
    pub product_gtin: String,
    pub product_dimensions: ProductDimensions,
    pub product_location: String,
    pub product_condition: String,
    pub product_multipack: i64,
    pub images: Vec<String>,
    pub brand: String,
    pub category: String,
    pub category_slug: String,
    pub offer_data: OfferData,
    pub stock: i64,
    pub facets: Vec<Facet>,
    pub variant: Variant,
    pub enabled: bool,
    pub created: String,
    pub store_urls: Vec<StoreUrl>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Facet {
    pub group: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "type")]
    pub item_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferData {
    pub property1: Property,
    pub property2: Property,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    pub price: String,
    pub kogan_first_price: String,
    pub tax_exempt: bool,
    pub shipping: String,
    pub handling_days: i64,
    pub rrp: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
    pub group_title: String,
    pub group_id: String,
    pub vary_on: VaryOn,
    pub vary_on_2: VaryOn,
    pub vary_on_3: VaryOn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaryOn {
    pub group: String,
    #[serde(rename = "type")]
    pub vary_on_type: String,
}
