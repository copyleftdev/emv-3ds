use crate::types::{Eci, MessageVersion, TransStatus, TransStatusReason};
use serde::{Deserialize, Serialize};

/// EMV 3DS Authentication Response (ARes).
///
/// Returned by the Directory Server (relaying from the ACS) in response to an AReq.
/// A `transStatus` of `C` means the ACS requires a challenge; `Y` or `A` means
/// authentication succeeded frictionlessly.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResponse {
    pub message_type: MessageType,
    pub message_version: MessageVersion,

    // ── Transaction IDs ───────────────────────────────────────────────────
    #[serde(rename = "threeDSServerTransID")]
    pub three_ds_server_trans_id: String,
    #[serde(rename = "acsTransID")]
    pub acs_trans_id: String,
    #[serde(rename = "dsTransID")]
    pub ds_trans_id: String,

    // ── Authentication outcome ────────────────────────────────────────────
    pub trans_status: TransStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status_reason: Option<TransStatusReason>,
    /// Whether the ACS mandates a challenge regardless of requestor preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acs_challenge_mandated: Option<AcsMandated>,
    /// ECI value to include in the authorization request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eci: Option<Eci>,
    /// CAVV / AAV — base64url encoded; present only on Y or A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_value: Option<String>,

    // ── Challenge parameters (only when transStatus = C) ─────────────────
    /// URL of the ACS challenge page (browser channel).
    #[serde(rename = "acsURL", skip_serializing_if = "Option::is_none")]
    pub acs_url: Option<String>,
    /// JWS-signed content for app-based challenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acs_signed_content: Option<String>,
    /// Decoupled authentication maximum timeout (minutes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acs_dec_con_ind: Option<String>,

    // ── DS reference info ─────────────────────────────────────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acs_reference_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds_reference_number: Option<String>,

    // ── Cardholder info (may be returned for informational NPA) ──────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_info: Option<String>,

    // ── Whitelisting (v2.2+) ──────────────────────────────────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_status: Option<WhitelistStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_status_source: Option<WhitelistStatusSource>,
}

impl AuthenticationResponse {
    /// True if authentication completed without a challenge.
    pub fn is_frictionless(&self) -> bool {
        self.trans_status.is_authenticated()
    }

    /// True if the requestor must present a challenge UI to the cardholder.
    pub fn requires_challenge(&self) -> bool {
        self.trans_status.requires_challenge()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    ARes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcsMandated {
    #[serde(rename = "Y")]
    Yes,
    #[serde(rename = "N")]
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhitelistStatus {
    #[serde(rename = "Y")]
    Whitelisted,
    #[serde(rename = "N")]
    NotWhitelisted,
    #[serde(rename = "E")]
    NotEligible,
    #[serde(rename = "P")]
    Pending,
    #[serde(rename = "R")]
    RequestedByRequestor,
    #[serde(rename = "U")]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhitelistStatusSource {
    #[serde(rename = "01")]
    ThreeDsServerCached,
    #[serde(rename = "02")]
    Ds,
    #[serde(rename = "03")]
    Acs,
}
