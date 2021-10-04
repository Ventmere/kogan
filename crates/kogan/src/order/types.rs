use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "Currency")]
    currency: OrderCurrency,
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Items")]
    items: Vec<Item>,
    #[serde(rename = "OrderDateUtc")]
    order_date_utc: DateTime<Utc>,
    #[serde(rename = "OrderStatus")]
    order_status: OrderStatus,
    #[serde(rename = "RequestedShippingMethod")]
    requested_shipping_method: String,
    #[serde(rename = "TotalGiftOptionPrice")]
    total_gift_option_price: f64,
    #[serde(rename = "TotalGiftOptionTaxPrice")]
    total_gift_option_tax_price: f64,
    #[serde(rename = "TotalPrice")]
    total_price: f64,
    #[serde(rename = "TotalShippingPrice")]
    total_shipping_price: f64,
    #[serde(rename = "TotalShippingTaxPrice")]
    total_shipping_tax_price: f64,
    #[serde(rename = "TotalTaxPrice")]
    total_tax_price: f64,
    #[serde(rename = "VatInclusive")]
    vat_inclusive: bool,
    #[serde(rename = "BuyerAddress")]
    buyer_address: Address,
    #[serde(rename = "DeliverByDateUtc")]
    deliver_by_date_utc: DateTime<Utc>,
    #[serde(rename = "OtherFees")]
    other_fees: f64,
    #[serde(rename = "PaymentMethod")]
    payment_method: Option<String>,
    #[serde(rename = "PaymentTransactionID")]
    payment_transaction_id: Option<String>,
    #[serde(rename = "PrivateNotes")]
    private_notes: Option<String>,
    #[serde(rename = "ShippingAddress")]
    shipping_address: Address,
    #[serde(rename = "ShippingLabelURL")]
    shipping_label_url: Option<String>,
    #[serde(rename = "SpecialInstructions")]
    special_instructions: Option<String>,
    #[serde(rename = "TotalOrderDiscount")]
    total_order_discount: f64,
    #[serde(rename = "TotalShippingDiscount")]
    total_shipping_discount: f64,
    #[serde(rename = "OrderLabel")]
    order_label: String,
    #[serde(rename = "DispatchedItems")]
    dispatched_items: Option<Vec<DispatchedItem>>,
    #[serde(rename = "CancelledItems")]
    cancelled_items: Option<Vec<Item>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "AddressLine1")]
    address_line1: String,
    #[serde(rename = "AddressLine2")]
    address_line2: String,
    #[serde(rename = "City")]
    city: String,
    #[serde(rename = "CompanyName")]
    company_name: String,
    #[serde(rename = "Country")]
    country: String,
    #[serde(rename = "DaytimePhone")]
    daytime_phone: String,
    #[serde(rename = "EveningPhone")]
    evening_phone: String,
    #[serde(rename = "EmailAddress")]
    email_address: String,
    #[serde(rename = "FirstName")]
    first_name: String,
    #[serde(rename = "LastName")]
    last_name: String,
    #[serde(rename = "NameSuffix")]
    name_suffix: String,
    #[serde(rename = "PostalCode")]
    postal_code: String,
    #[serde(rename = "StateOrProvince")]
    state_or_province: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "SellerSku")]
    seller_sku: String,
    #[serde(rename = "Quantity")]
    quantity: i64,
    #[serde(rename = "Reason")]
    reason: Option<String>,
    #[serde(rename = "UnitPrice")]
    unit_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DispatchedItem {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Quantity")]
    quantity: i64,
    #[serde(rename = "SellerSku")]
    seller_sku: String,
    #[serde(rename = "DispatchDateUtc")]
    dispatch_date_utc: String,
    #[serde(rename = "Carrier")]
    carrier: String,
    #[serde(rename = "TrackingNumber")]
    tracking_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OrderStatus {
    ReleasedForShipment,
    Pending,
    AcknowledgedBySeller,
    PartiallyShipped,
    Shipped,
    Canceled,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OrderRequestedShippingMethod {
    Standard
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderCurrency {
    AUD,
    GBP,
    NZD,
    USD
}