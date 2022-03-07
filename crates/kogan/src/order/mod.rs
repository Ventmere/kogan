mod types;
pub use self::types::*;

use crate::client::{KoganClient, KoganRequestBuilderExt, Method};
use crate::error::{KoganError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

impl KoganClient {
    pub async fn get_orders(&self, params: GetOrdersParams) -> Result<GetOrdersResponse> {
        self.request(Method::GET, "/orders/")
            .query(&params)
            .send_json()
            .await
    }

    pub async fn get_order(&self, kogan_order_ref: &str) -> Result<GetOrderResponse> {
        self.request(Method::GET, &format!("/orders/{}/", kogan_order_ref))
            .send_json()
            .await
    }

    pub async fn post_order_dispatch_info(
        &self,
        params: Vec<PostOrderDispatchInfoParams>,
    ) -> Result<PostOrderDispatchInfoResponse> {
        self.request(Method::POST, "/orders/fulfill/")
            .json(&params)
            .send_json()
            .await
    }

    pub async fn post_order_cancellation_request(
        &self,
        params: PostOrderCancellationParams,
    ) -> Result<PostOrderCancellationResponse> {
        self.request(
            Method::POST,
            &format!("/orders/orders/{}/cancel/", params.order_id),
        )
        .json(&params)
        .send_json()
        .await
    }

    pub async fn post_order_refund_request(
        &self,
        params: PostOrderRefundParams,
    ) -> Result<PostOrderRefundResponse> {
        self.request(
            Method::POST,
            &format!("/orders/orders/{}/refund/", params.order_id),
        )
        .json(&params)
        .send_json()
        .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOrdersParams {
    pub status: OrderStatus,
    pub limit: Option<i64>,
    pub start_date_utc: Option<DateTime<Utc>>,
    pub end_date_utc: Option<DateTime<Utc>>,
}

impl Default for GetOrdersParams {
    fn default() -> Self {
        Self {
            status: OrderStatus::ReleasedForShipment,
            limit: None,
            start_date_utc: None,
            end_date_utc: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOrdersResponse {
    pub status: OrderResponseStatus,
    pub pending_url: Option<String>,
    pub body: Vec<Order>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OrderResponseStatus {
    Complete,
    CompleteWithErrors,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOrderResponse {
    pub status: OrderResponseStatus,
    pub pending_url: Option<String>,
    pub body: Order,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOrderDispatchInfoParams {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Items")]
    pub items: Vec<OrderDispatchInfoItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDispatchInfoItem {
    #[serde(rename = "OrderItemID")]
    pub order_item_id: String,
    #[serde(rename = "SellerSku")]
    pub seller_sku: String,
    #[serde(rename = "Quantity")]
    pub quantity: i64,
    #[serde(rename = "ShippedDateUtc")]
    pub shipped_date_utc: DateTime<Utc>,
    #[serde(rename = "TrackingNumber")]
    pub tracking_number: String,
    #[serde(rename = "ShippingCarrier")]
    pub shipping_carrier: OrderShippingCarrier,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderShippingCarrier {
    /// Australia Post
    AUP,
    /// Star Track
    ST,
    /// Toll IPEC
    TIPC,
    /// DHL
    DHL,
    /// FedEX
    FDX,
    /// UPS
    UPS,
    /// Couriers Please
    CP,
    /// Direct Freight Express
    DFE,
    /// Aramex
    ARAMEX,
    /// Fastway
    FW,
    /// Kings Transport
    KINGS,
    /// Hunter Express
    HX,
    /// New Zealand Post
    NZP,
    /// XL Express
    XLE,
    /// TNT Express
    TNT,
    /// Allied Express
    R,
    /// Border Express
    BEX,
    /// Blue Star Logistics
    BLUESTAR,
    /// Northline
    NTH,
    /// ADSOne
    ADSONE,
    /// Sendle
    SENDLE,
    /// COPE Sensitive Freight
    COPE,
    /// Bpost
    BPOST,
    /// Shippit
    SHIPPIT,
    /// Megasave
    MEGASAVE,
    /// QLS Standard
    QLSSTD,
    /// QLS Premium
    QLSPRE,
    /// Omni Parcel
    OMNI,
}

impl FromStr for OrderShippingCarrier {
    type Err = KoganError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str().trim() {
            "AUP" | "AUSTRALIA POST" => Ok(Self::AUP),
            "ST" | "STAR TRACK" => Ok(Self::ST),
            "TIPC" | "TOLL IPEC" => Ok(Self::TIPC),
            "DHL" => Ok(Self::DHL),
            "FDX" | "FEDEX" => Ok(Self::FDX),
            "UPS" => Ok(Self::UPS),
            "CP" | "COURIERS PLEASE" => Ok(Self::CP),
            "DFE" | "DIRECT FREIGHT EXPRESS" => Ok(Self::DFE),
            "ARAMEX" => Ok(Self::ARAMEX),
            "FW" | "FASTWAY" => Ok(Self::FW),
            "KINGS" | "KINGS TRANSPORT" => Ok(Self::KINGS),
            "HX" | "HUNTER EXPRESS" => Ok(Self::HX),
            "NZP" | "NEW ZEALAND POST" => Ok(Self::NZP),
            "XLE" | "XL EXPRESS" => Ok(Self::XLE),
            "TNT" | "TNT EXPRESS" => Ok(Self::TNT),
            "R" | "ALLIED EXPRESS" => Ok(Self::R),
            "BEX" | "BORDER EXPRESS" => Ok(Self::BEX),
            "BLUESTAR" | "BLUE STAR LOGISTICS" => Ok(Self::BLUESTAR),
            "NTH" | "NORTHLINE" => Ok(Self::NTH),
            "ADSONE" => Ok(Self::ADSONE),
            "SENDLE" => Ok(Self::SENDLE),
            "COPE" | "COPE SENSITIVE FREIGHT" => Ok(Self::COPE),
            "BPOST" => Ok(Self::BPOST),
            "SHIPPIT" => Ok(Self::SHIPPIT),
            "MEGASAVE" => Ok(Self::MEGASAVE),
            "QLSSTD" | "QLS STANDARD" => Ok(Self::QLSSTD),
            "QLSPRE" | "QLS PREMIUM" => Ok(Self::QLSPRE),
            "OMNI" | "OMNI PARCEL" => Ok(Self::OMNI),
            other => Err(KoganError::InvalidCarriorCode(other.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOrderDispatchInfoResponse {
    pub status: OrderResponseStatus,
    pub pending_url: Option<String>,
    pub body: Vec<OrderItemFulfillmentResult>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemFulfillmentResult {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Result")]
    pub result: String,
    #[serde(rename = "Errors")]
    pub errors: Vec<OrderItemFulfillmentError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemFulfillmentError {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ErrorCode")]
    pub error_code: String,
    #[serde(rename = "Message")]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOrderCancellationParams {
    #[serde(rename = "OrderID")]
    pub order_id: String,
    #[serde(rename = "Items")]
    pub items: Vec<OrderCancellationItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCancellationItem {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "SellerSku")]
    seller_sku: String,
    #[serde(rename = "Quantity")]
    quantity: i64,
    #[serde(rename = "Reason")]
    reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OrderCancellationReason {
    Other,
    GeneralAdjustment,
    ItemNotAvailable,
    CustomerReturnedItem,
    CouldNotShip,
    AlternateItemProvided,
    BuyerCanceled,
    CustomerExchange,
    MerchandiseNotReceived,
    ShippingAddressUndeliverable,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOrderCancellationResponse {
    pub status: OrderResponseStatus,
    pub pending_url: Option<String>,
    pub body: Value,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOrderRefundParams {
    #[serde(rename = "OrderID")]
    pub order_id: String,
    #[serde(rename = "Items")]
    pub items: Vec<OrderRefundItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRefundItem {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "SellerSku")]
    seller_sku: String,
    #[serde(rename = "Quantity")]
    quantity: i64,
    #[serde(rename = "Reason")]
    reason: OrderRefundReason,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OrderRefundReason {
    Other,
    GeneralAdjustment,
    ItemNotAvailable,
    CustomerReturnedItem,
    CouldNotShip,
    AlternateItemProvided,
    BuyerCanceled,
    CustomerExchange,
    MerchandiseNotReceived,
    ShippingAddressUndeliverable,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOrderRefundResponse {
    pub status: OrderResponseStatus,
    pub pending_url: Option<String>,
    pub body: Value,
    pub error: Option<String>,
}
