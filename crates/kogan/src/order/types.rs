use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "Currency")]
    pub currency: OrderCurrency,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "OrderDateUtc")]
    pub order_date_utc: DateTime<Utc>,
    #[serde(rename = "OrderStatus")]
    pub order_status: OrderStatus,
    #[serde(rename = "RequestedShippingMethod")]
    pub requested_shipping_method: String,
    #[serde(rename = "TotalGiftOptionPrice")]
    pub total_gift_option_price: f64,
    #[serde(rename = "TotalGiftOptionTaxPrice")]
    pub total_gift_option_tax_price: f64,
    #[serde(rename = "TotalPrice")]
    pub total_price: f64,
    #[serde(rename = "TotalShippingPrice")]
    pub total_shipping_price: f64,
    #[serde(rename = "TotalShippingTaxPrice")]
    pub total_shipping_tax_price: f64,
    #[serde(rename = "TotalTaxPrice")]
    pub total_tax_price: f64,
    #[serde(rename = "VatInclusive")]
    pub vat_inclusive: bool,
    #[serde(rename = "BuyerAddress")]
    pub buyer_address: Address,
    #[serde(rename = "DeliverByDateUtc")]
    pub deliver_by_date_utc: DateTime<Utc>,
    #[serde(rename = "OtherFees")]
    pub other_fees: f64,
    #[serde(rename = "PaymentMethod")]
    pub payment_method: Option<String>,
    #[serde(rename = "PaymentTransactionID")]
    pub payment_transaction_id: Option<String>,
    #[serde(rename = "PrivateNotes")]
    pub private_notes: Option<String>,
    #[serde(rename = "ShippingAddress")]
    pub shipping_address: Address,
    #[serde(rename = "ShippingLabelURL")]
    pub shipping_label_url: Option<String>,
    #[serde(rename = "SpecialInstructions")]
    pub special_instructions: Option<String>,
    #[serde(rename = "TotalOrderDiscount")]
    pub total_order_discount: f64,
    #[serde(rename = "TotalShippingDiscount")]
    pub total_shipping_discount: f64,
    #[serde(rename = "OrderLabel")]
    pub order_label: String,
    #[serde(rename = "DispatchedItems")]
    pub dispatched_items: Option<Vec<DispatchedItem>>,
    #[serde(rename = "CancelledItems")]
    pub cancelled_items: Option<Vec<Item>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "AddressLine1")]
    pub address_line1: String,
    #[serde(rename = "AddressLine2")]
    pub address_line2: String,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "CompanyName")]
    pub company_name: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "DaytimePhone")]
    pub daytime_phone: String,
    #[serde(rename = "EveningPhone")]
    pub evening_phone: String,
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "NameSuffix")]
    pub name_suffix: String,
    #[serde(rename = "PostalCode")]
    pub postal_code: String,
    #[serde(rename = "StateOrProvince")]
    pub state_or_province: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "SellerSku")]
    pub seller_sku: String,
    #[serde(rename = "Quantity")]
    pub quantity: i64,
    #[serde(rename = "Reason")]
    pub reason: Option<String>,
    #[serde(rename = "UnitPrice")]
    pub unit_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DispatchedItem {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Quantity")]
    pub quantity: i64,
    #[serde(rename = "SellerSku")]
    pub seller_sku: String,
    #[serde(rename = "DispatchDateUtc")]
    pub dispatch_date_utc: String,
    #[serde(rename = "Carrier")]
    pub carrier: String,
    #[serde(rename = "TrackingNumber")]
    pub tracking_number: String,
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

impl OrderCurrency {
    pub fn as_str(&self) -> &'static str {
        match *self {
            OrderCurrency::AUD => "AUD",
            OrderCurrency::GBP => "GBP",
            OrderCurrency::NZD => "NZD",
            OrderCurrency::USD => "USD",
        }
    }
}