use crate::types::{
    AcsAuthMethod, AuthenticationType, Eci, MessageCategory, MessageVersion, TransStatus,
    TransStatusReason,
};
use serde::{Deserialize, Serialize};

/// EMV 3DS Results Request (RReq).
///
/// Sent by the ACS to the 3DS Server (via the DS) after a decoupled or
/// app-based challenge completes. The 3DS Server must respond with an RRes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsRequest {
    pub message_type: MessageType,
    pub message_version: MessageVersion,
    #[serde(rename = "threeDSServerTransID")]
    pub three_ds_server_trans_id: String,
    #[serde(rename = "acsTransID")]
    pub acs_trans_id: String,
    #[serde(rename = "dsTransID", skip_serializing_if = "Option::is_none")]
    pub ds_trans_id: Option<String>,
    #[serde(rename = "sdkTransID", skip_serializing_if = "Option::is_none")]
    pub sdk_trans_id: Option<String>,
    pub message_category: MessageCategory,
    pub trans_status: TransStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status_reason: Option<TransStatusReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<AcsAuthMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<AuthenticationType>,
    /// CAVV / AAV — base64url encoded; present when transStatus is Y or A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eci: Option<Eci>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_counter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acs_rendering_type: Option<AcsRenderingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_extension: Option<Vec<MessageExtension>>,
}

/// ACS UI rendering information for app-based flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcsRenderingType {
    pub acs_interface: String,
    pub acs_ui_template: String,
}

/// Generic protocol extension element — carried in RReq and RRes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageExtension {
    pub name: String,
    pub id: String,
    pub critical_indicator: bool,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    RReq,
}
