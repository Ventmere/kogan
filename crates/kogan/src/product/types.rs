use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    product_title: String,
    product_sku: String,
    product_subtitle: String,
    product_description: String,
    product_inbox: String,
    product_gtin: String,
    product_dimensions: ProductDimensions,
    product_location: String,
    product_condition: String,
    product_multipack: i64,
    images: Vec<String>,
    brand: String,
    category: String,
    category_slug: String,
    offer_data: OfferData,
    stock: i64,
    facets: Vec<Facet>,
    variant: Variant,
    enabled: bool,
    created: String,
    store_urls: Vec<StoreUrl>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Facet {
    group: String,
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "type")]
    item_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferData {
    property1: Property,
    property2: Property,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    price: String,
    kogan_first_price: String,
    tax_exempt: bool,
    shipping: String,
    handling_days: i64,
    rrp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDimensions {
    length: i64,
    width: i64,
    height: i64,
    weight: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreUrl {
    url: String,
    store_name: String,
    organisation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
    group_title: String,
    group_id: String,
    vary_on: VaryOn,
    vary_on_2: VaryOn,
    vary_on_3: VaryOn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaryOn {
    group: String,
    #[serde(rename = "type")]
    vary_on_type: String,
}
