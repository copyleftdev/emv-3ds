use crate::types::MessageVersion;
use serde::{Deserialize, Serialize};

/// EMV 3DS Error Message.
///
/// Sent by any component (3DS Server, DS, ACS) to indicate a protocol error.
/// A received ErrorMessage should abort the current transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub message_type: MessageType,
    pub message_version: MessageVersion,
    /// Three-digit code; see EMVCo spec Table A.5.
    pub error_code: String,
    /// Human-readable description of the error.
    pub error_description: String,
    /// Detail about the specific field that caused the error.
    pub error_detail: String,
    /// The messageType of the message that caused this error.
    pub error_message_type: String,
    /// Transaction ID of the erroring message.
    #[serde(
        rename = "threeDSServerTransID",
        skip_serializing_if = "Option::is_none"
    )]
    pub three_ds_server_trans_id: Option<String>,
    #[serde(rename = "acsTransID", skip_serializing_if = "Option::is_none")]
    pub acs_trans_id: Option<String>,
    #[serde(rename = "dsTransID", skip_serializing_if = "Option::is_none")]
    pub ds_trans_id: Option<String>,
    #[serde(rename = "sdkTransID", skip_serializing_if = "Option::is_none")]
    pub sdk_trans_id: Option<String>,
}

/// Well-known EMVCo error codes.
pub mod error_codes {
    pub const INVALID_FORMAT: &str = "101";
    pub const INVALID_FORMAT_CRITICAL: &str = "102";
    pub const REQUIRED_ELEMENT_MISSING: &str = "201";
    pub const CRITICAL_ELEMENT_MISSING: &str = "202";
    pub const FORMAT_VIOLATION: &str = "203";
    pub const DUPLICATE_ELEMENT: &str = "301";
    pub const TRANSACTION_TIMED_OUT: &str = "402";
    pub const TRANSACTION_TIMED_OUT_DECOUPLED: &str = "403";
    pub const ACCESS_DENIED: &str = "404";
    pub const UNKNOWN_PROTOCOL_ELEMENT: &str = "405";
    pub const SYSTEM_ERROR: &str = "500";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Erro,
}
