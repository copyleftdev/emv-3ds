use crate::types::MessageVersion;
use serde::{Deserialize, Serialize};

/// EMV 3DS Preparation Request (PReq).
///
/// Sent by the 3DS Server to the Directory Server before the first AReq to
/// negotiate protocol versions and retrieve card range data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparationRequest {
    pub message_type: MessageType,
    pub message_version: MessageVersion,
    #[serde(rename = "threeDSServerTransID")]
    pub three_ds_server_trans_id: String,
    #[serde(rename = "dsTransID", skip_serializing_if = "Option::is_none")]
    pub ds_trans_id: Option<String>,
    /// Serial number of the last card range data received; omit on first request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_num: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_server_ref_number: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    PReq,
}
