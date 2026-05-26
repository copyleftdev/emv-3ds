use crate::types::{MessageVersion, TransStatus};
use serde::{Deserialize, Serialize};

/// EMV 3DS Challenge Response (CRes).
///
/// Sent by the ACS back to the 3DS Server (via the browser/SDK) to report
/// the outcome of the challenge. A `transStatus` of `Y` means the challenge
/// was passed; `N` means it was failed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeResponse {
    pub message_type: MessageType,
    pub message_version: MessageVersion,
    #[serde(rename = "threeDSServerTransID")]
    pub three_ds_server_trans_id: String,
    #[serde(rename = "acsTransID")]
    pub acs_trans_id: String,
    /// Final authentication outcome.
    pub trans_status: TransStatus,
    /// "Y" when the challenge is complete (final CRes).
    pub challenge_completion_ind: CompletionIndicator,
    /// Additional UI content from the ACS (HTML string, v2.2+).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acs_ui: Option<String>,
    /// UI type of the ACS content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acs_ui_type: Option<AcsUiType>,
    /// ACS-rendered challenge HTML (native UI, when sdk_ui_type = 05).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acs_html: Option<String>,
    /// Whitelist status after challenge (v2.2+).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_status: Option<super::ares::WhitelistStatus>,
}

impl ChallengeResponse {
    /// True if the cardholder successfully passed the challenge.
    pub fn is_authenticated(&self) -> bool {
        self.trans_status.is_authenticated()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    CRes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionIndicator {
    #[serde(rename = "Y")]
    Complete,
    #[serde(rename = "N")]
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcsUiType {
    #[serde(rename = "01")]
    Text,
    #[serde(rename = "02")]
    SingleSelect,
    #[serde(rename = "03")]
    MultiSelect,
    #[serde(rename = "04")]
    Oob,
    #[serde(rename = "05")]
    HtmlOther,
}
