use crate::types::{MessageVersion, ResultsStatus};
use serde::{Deserialize, Serialize};

/// EMV 3DS Results Response (RRes).
///
/// Sent by the 3DS Server to the ACS to acknowledge receipt of an RReq.
/// Build with `ResultsResponse::acknowledge(&rreq)` immediately after receiving
/// an RReq; send before or concurrently with calling `TransactionState::receive_rreq`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsResponse {
    pub message_type: MessageType,
    pub message_version: MessageVersion,
    #[serde(rename = "threeDSServerTransID")]
    pub three_ds_server_trans_id: String,
    #[serde(rename = "acsTransID")]
    pub acs_trans_id: String,
    #[serde(rename = "dsTransID", skip_serializing_if = "Option::is_none")]
    pub ds_trans_id: Option<String>,
    pub results_status: ResultsStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_extension: Option<Vec<super::rreq::MessageExtension>>,
}

impl ResultsResponse {
    /// Build the acknowledgment RRes for a received RReq.
    pub fn acknowledge(rreq: &super::rreq::ResultsRequest) -> Self {
        Self {
            message_type: MessageType::RRes,
            message_version: rreq.message_version,
            three_ds_server_trans_id: rreq.three_ds_server_trans_id.clone(),
            acs_trans_id: rreq.acs_trans_id.clone(),
            ds_trans_id: rreq.ds_trans_id.clone(),
            results_status: ResultsStatus::Received,
            message_extension: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    RRes,
}
