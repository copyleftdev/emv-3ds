use crate::types::{ActionIndicator, MessageVersion};
use serde::{Deserialize, Serialize};

/// EMV 3DS Preparation Response (PRes).
///
/// Returned by the Directory Server with card range data and supported protocol
/// versions. The 3DS Server uses `cardRangeData` to look up the ACS URL and
/// the protocol version range for a given PAN.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparationResponse {
    pub message_type: MessageType,
    pub message_version: MessageVersion,
    #[serde(rename = "threeDSServerTransID")]
    pub three_ds_server_trans_id: String,
    #[serde(rename = "dsTransID", skip_serializing_if = "Option::is_none")]
    pub ds_trans_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_num: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds_reference_number: Option<String>,
    pub ds_start_protocol_version: String,
    pub ds_end_protocol_version: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub card_range_data: Vec<CardRangeData>,
}

impl PreparationResponse {
    /// Return the card range entry that covers `pan`, if any.
    pub fn range_for_pan(&self, pan: &str) -> Option<&CardRangeData> {
        self.card_range_data
            .iter()
            .find(|r| pan >= r.start_range.as_str() && pan <= r.end_range.as_str())
    }
}

/// A single BIN-range entry returned in a PRes payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRangeData {
    pub start_range: String,
    pub end_range: String,
    pub action_ind: ActionIndicator,
    pub acs_start_protocol_version: String,
    pub acs_end_protocol_version: String,
    #[serde(rename = "threeDSMethodURL", skip_serializing_if = "Option::is_none")]
    pub three_ds_method_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds_start_protocol_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds_end_protocol_version: Option<String>,
    #[serde(rename = "dsTransID", skip_serializing_if = "Option::is_none")]
    pub ds_trans_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acs_info_ind: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    PRes,
}
