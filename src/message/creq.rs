use crate::types::{ChallengeWindowSize, MessageVersion};
use serde::{Deserialize, Serialize};

/// EMV 3DS Challenge Request (CReq).
///
/// Sent by the browser (or SDK) to the ACS to initiate or continue the
/// challenge flow. For browser channel this is typically submitted as a
/// hidden form POST; for app channel it is sent directly from the SDK.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeRequest {
    pub message_type: MessageType,
    pub message_version: MessageVersion,
    #[serde(rename = "threeDSServerTransID")]
    pub three_ds_server_trans_id: String,
    #[serde(rename = "acsTransID")]
    pub acs_trans_id: String,
    /// Cardholder-provided challenge data (OTP, text, selected option, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_data_entry: Option<String>,
    /// Preferred challenge window dimensions (browser channel only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_window_size: Option<ChallengeWindowSize>,
    /// "Y" once the cardholder has completed the challenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_completion_ind: Option<CompletionIndicator>,
    /// SDK transaction ID (app channel only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk_trans_id: Option<String>,
    /// HTML cancel requested by cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resend_challenge: Option<ResendChallenge>,
    /// Merchant whitelist opt-in preference (v2.2+).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_status_source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    CReq,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionIndicator {
    #[serde(rename = "Y")]
    Completed,
    #[serde(rename = "N")]
    NotCompleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResendChallenge {
    #[serde(rename = "Y")]
    Yes,
}
