use crate::types::device::{BrowserInfo, DeviceRenderOptions, SdkInfo};
use crate::types::{
    AccountInfo, AccountType, AuthenticationMethod, ChallengeIndicator, DeliveryTimeframe,
    DeviceChannel, MessageCategory, MessageVersion, PreOrderPurchaseIndicator,
    ReorderItemsIndicator, ShipIndicator, ThreeDsMethod,
};
use serde::{Deserialize, Serialize};

/// EMV 3DS Authentication Request (AReq).
///
/// Sent by the 3DS Server to the Directory Server to initiate authentication.
/// Fields marked `Option` are conditional or optional per the EMVCo spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationRequest {
    // ── Message envelope ──────────────────────────────────────────────────
    pub message_type: MessageType,
    pub message_version: MessageVersion,
    /// UUIDv4 assigned by the 3DS Server for this transaction.
    #[serde(rename = "threeDSServerTransID")]
    pub three_ds_server_trans_id: String,
    pub device_channel: DeviceChannel,
    pub message_category: MessageCategory,

    // ── Requestor identity ────────────────────────────────────────────────
    pub three_ds_requestor_id: String,
    pub three_ds_requestor_name: String,
    pub three_ds_requestor_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_requestor_authentication_ind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_requestor_authentication_info: Option<AuthenticationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_requestor_challenge_ind: Option<ChallengeIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_requestor_prior_authentication_info: Option<PriorAuthenticationInfo>,

    // ── Card / account ────────────────────────────────────────────────────
    /// PAN — must be encrypted when transmitted outside the 3DS Server.
    pub acct_number: String,
    /// YYMM
    pub card_expiry_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_type: Option<AccountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_info: Option<AccountInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<String>,

    // ── Purchase ──────────────────────────────────────────────────────────
    /// Minor-unit purchase amount as a numeric string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_amount: Option<String>,
    /// ISO 4217 numeric currency code, zero-padded to 3 digits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_currency: Option<String>,
    /// Decimal exponent (e.g. "2" for cents).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_exponent: Option<String>,
    /// UTC datetime, format: YYYYMMDDHHMMSS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<String>,
    /// Type of transaction: "01" goods/service, "03" cash-advance, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_type: Option<String>,
    /// Recurring/instalment expiry date (YYYYMMDD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_expiry: Option<String>,
    /// Days between recurring charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_frequency: Option<String>,
    /// Instalments count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_instal_data: Option<String>,

    // ── Merchant ──────────────────────────────────────────────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_risk_indicator: Option<MerchantRiskIndicator>,

    // ── Cardholder ────────────────────────────────────────────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_phone: Option<PhoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<PhoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_phone: Option<PhoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_addr_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_addr_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_addr_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_addr_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_addr_line3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_addr_post_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_addr_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_addr_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_addr_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_addr_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_addr_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_addr_line3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_addr_post_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_addr_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr_match: Option<AddrMatch>,

    // ── 3DS Method ────────────────────────────────────────────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_comp_ind: Option<ThreeDsMethod>,
    /// Notification URL where the ACS posts the CRes (browser channel).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,

    // ── Browser channel ───────────────────────────────────────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_info: Option<BrowserInfo>,

    // ── App channel ───────────────────────────────────────────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk_info: Option<SdkInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_render_options: Option<DeviceRenderOptions>,
}

impl AuthenticationRequest {
    pub fn message_type_value() -> &'static str {
        "AReq"
    }
}

/// Constant "AReq" literal for the messageType field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    #[default]
    AReq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationInfo {
    pub three_ds_req_auth_method: AuthenticationMethod,
    pub three_ds_req_auth_timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_req_auth_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorAuthenticationInfo {
    pub three_ds_req_prior_ref: String,
    pub three_ds_req_prior_auth_method: String,
    pub three_ds_req_prior_auth_timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_req_prior_auth_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantRiskIndicator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_timeframe: Option<DeliveryTimeframe>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_curr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_order_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_order_purchase_ind: Option<PreOrderPurchaseIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reorder_items_ind: Option<ReorderItemsIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_indicator: Option<ShipIndicator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneNumber {
    pub cc: String,
    pub subscriber: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddrMatch {
    #[serde(rename = "Y")]
    Match,
    #[serde(rename = "N")]
    NoMatch,
}
