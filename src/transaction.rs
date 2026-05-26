use crate::error::{Error, Result};
use crate::message::{
    AuthenticationRequest, AuthenticationResponse, ChallengeRequest, ChallengeResponse,
    ErrorMessage, ResultsRequest,
};
use crate::types::{Eci, TransStatus};

/// The lifecycle of a single 3DS transaction as seen by the 3DS Server.
///
/// Transitions are driven by sending an AReq and receiving subsequent messages.
/// Terminal states are `Authenticated`, `NotAuthenticated`, and `Failed`.
///
/// ```text
///  Created
///    │  send_areq()
///    ▼
///  AwaitingARes
///    │  receive_ares()
///    ├─(Y/A)──────────────────────► Authenticated
///    ├─(N/U/R)────────────────────► NotAuthenticated
///    ├─(C)
///    │    │
///    │    ▼
///    │  AwaitingCRes
///    │    │  receive_cres()
///    │    ├─(Y/A)────────────────► Authenticated
///    │    └─(N/U)────────────────► NotAuthenticated
///    └─(D)
///         │
///         ▼
///       AwaitingRReq
///         │  receive_rreq()
///         ├─(Y/A)──────────────────► Authenticated
///         └─(N/U/R)────────────────► NotAuthenticated
///
///  Any state + receive_error() ──► Failed
/// ```
#[derive(Debug, Clone)]
pub enum TransactionState {
    /// AReq has been constructed and is ready to send.
    Created { areq: Box<AuthenticationRequest> },
    /// AReq sent; waiting for the ARes from the DS.
    AwaitingARes { three_ds_server_trans_id: String },
    /// ARes received with transStatus=C; challenge must be presented.
    AwaitingCRes {
        three_ds_server_trans_id: String,
        acs_trans_id: String,
        /// ACS URL to redirect/embed (browser channel), or None for app/decoupled.
        acs_url: Option<String>,
    },
    /// ARes received with transStatus=D; waiting for the ACS to send an RReq
    /// after completing decoupled out-of-band authentication.
    AwaitingRReq {
        three_ds_server_trans_id: String,
        acs_trans_id: String,
    },
    /// Authentication completed successfully (frictionless Y/A or challenge Y).
    Authenticated {
        three_ds_server_trans_id: String,
        acs_trans_id: String,
        ds_trans_id: Option<String>,
        eci: Option<Eci>,
        /// CAVV / AAV value to include in the authorization request.
        authentication_value: Option<String>,
    },
    /// Authentication was rejected or failed (N, U, R, or challenge N).
    NotAuthenticated {
        three_ds_server_trans_id: String,
        trans_status: TransStatus,
        reason_code: Option<String>,
    },
    /// A protocol error was received; the transaction cannot continue.
    Failed { error: String },
}

impl TransactionState {
    /// Create a new transaction from a constructed AReq.
    pub fn new(areq: AuthenticationRequest) -> Self {
        Self::Created {
            areq: Box::new(areq),
        }
    }

    /// Mark the AReq as sent and transition to awaiting the ARes.
    pub fn areq_sent(self) -> Result<(Self, AuthenticationRequest)> {
        match self {
            Self::Created { areq } => {
                let id = areq.three_ds_server_trans_id.clone();
                let next = Self::AwaitingARes {
                    three_ds_server_trans_id: id,
                };
                Ok((next, *areq))
            }
            other => Err(Error::InvalidTransition {
                from: other.name().to_owned(),
                to: "AwaitingARes".to_owned(),
            }),
        }
    }

    /// Process an incoming ARes and advance the state.
    pub fn receive_ares(self, ares: AuthenticationResponse) -> Result<Self> {
        let Self::AwaitingARes {
            three_ds_server_trans_id,
        } = self
        else {
            return Err(Error::InvalidTransition {
                from: self.name().to_owned(),
                to: "post-ARes".to_owned(),
            });
        };

        if ares.three_ds_server_trans_id != three_ds_server_trans_id {
            return Err(Error::InvalidField {
                field: "threeDSServerTransID",
                reason: "ARes trans ID does not match AReq".to_owned(),
            });
        }

        let next = match ares.trans_status {
            TransStatus::Success | TransStatus::Attempted => Self::Authenticated {
                three_ds_server_trans_id,
                acs_trans_id: ares.acs_trans_id,
                ds_trans_id: Some(ares.ds_trans_id),
                eci: ares.eci,
                authentication_value: ares.authentication_value,
            },
            TransStatus::ChallengeRequired => Self::AwaitingCRes {
                three_ds_server_trans_id,
                acs_trans_id: ares.acs_trans_id,
                acs_url: ares.acs_url,
            },
            TransStatus::DecoupledRequired => Self::AwaitingRReq {
                three_ds_server_trans_id,
                acs_trans_id: ares.acs_trans_id,
            },
            status => Self::NotAuthenticated {
                three_ds_server_trans_id,
                trans_status: status,
                reason_code: ares.trans_status_reason.map(|r| format!("{r:?}")),
            },
        };

        Ok(next)
    }

    /// Build the CReq that should be submitted to the ACS challenge URL.
    pub fn build_creq(
        &self,
        window_size: Option<crate::types::ChallengeWindowSize>,
    ) -> Result<ChallengeRequest> {
        let Self::AwaitingCRes {
            three_ds_server_trans_id,
            acs_trans_id,
            ..
        } = self
        else {
            return Err(Error::InvalidTransition {
                from: self.name().to_owned(),
                to: "CReq".to_owned(),
            });
        };

        Ok(ChallengeRequest {
            message_type: crate::message::creq::MessageType::CReq,
            message_version: crate::types::MessageVersion::V220,
            three_ds_server_trans_id: three_ds_server_trans_id.clone(),
            acs_trans_id: acs_trans_id.clone(),
            challenge_data_entry: None,
            challenge_window_size: window_size,
            challenge_completion_ind: None,
            sdk_trans_id: None,
            resend_challenge: None,
            whitelist_status_source: None,
        })
    }

    /// Process an incoming CRes and advance to a terminal state.
    pub fn receive_cres(self, cres: ChallengeResponse) -> Result<Self> {
        let Self::AwaitingCRes {
            three_ds_server_trans_id,
            acs_trans_id,
            ..
        } = self
        else {
            return Err(Error::InvalidTransition {
                from: self.name().to_owned(),
                to: "post-CRes".to_owned(),
            });
        };

        let next = if cres.trans_status.is_authenticated() {
            Self::Authenticated {
                three_ds_server_trans_id,
                acs_trans_id,
                ds_trans_id: None,
                eci: None,
                authentication_value: None,
            }
        } else {
            Self::NotAuthenticated {
                three_ds_server_trans_id,
                trans_status: cres.trans_status,
                reason_code: None,
            }
        };

        Ok(next)
    }

    /// Process an incoming RReq and advance to a terminal state.
    /// The caller must send `ResultsResponse::acknowledge(&rreq)` to the ACS.
    pub fn receive_rreq(self, rreq: ResultsRequest) -> Result<Self> {
        let Self::AwaitingRReq {
            three_ds_server_trans_id,
            acs_trans_id,
        } = self
        else {
            return Err(Error::InvalidTransition {
                from: self.name().to_owned(),
                to: "post-RReq".to_owned(),
            });
        };

        if rreq.three_ds_server_trans_id != three_ds_server_trans_id {
            return Err(Error::InvalidField {
                field: "threeDSServerTransID",
                reason: "RReq trans ID does not match AReq".to_owned(),
            });
        }

        let next = if rreq.trans_status.is_authenticated() {
            Self::Authenticated {
                three_ds_server_trans_id,
                acs_trans_id,
                ds_trans_id: rreq.ds_trans_id,
                eci: rreq.eci,
                authentication_value: rreq.authentication_value,
            }
        } else {
            Self::NotAuthenticated {
                three_ds_server_trans_id,
                trans_status: rreq.trans_status,
                reason_code: rreq.trans_status_reason.map(|r| format!("{r:?}")),
            }
        };

        Ok(next)
    }

    /// Handle a protocol error; always transitions to `Failed`.
    pub fn receive_error(self, err: &ErrorMessage) -> Self {
        Self::Failed {
            error: format!(
                "[{}] {}: {}",
                err.error_code, err.error_description, err.error_detail
            ),
        }
    }

    /// Returns true if the transaction reached a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Authenticated { .. } | Self::NotAuthenticated { .. } | Self::Failed { .. }
        )
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Created { .. } => "Created",
            Self::AwaitingARes { .. } => "AwaitingARes",
            Self::AwaitingCRes { .. } => "AwaitingCRes",
            Self::AwaitingRReq { .. } => "AwaitingRReq",
            Self::Authenticated { .. } => "Authenticated",
            Self::NotAuthenticated { .. } => "NotAuthenticated",
            Self::Failed { .. } => "Failed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::areq;
    use crate::message::ares;
    use crate::message::cres;
    use crate::types::{DeviceChannel, MessageCategory, MessageVersion, TransStatus};

    fn minimal_areq() -> AuthenticationRequest {
        AuthenticationRequest {
            message_type: areq::MessageType::AReq,
            message_version: MessageVersion::V220,
            three_ds_server_trans_id: "test-txn-id".to_owned(),
            device_channel: DeviceChannel::Browser,
            message_category: MessageCategory::PaymentAuthentication,
            three_ds_requestor_id: "req-001".to_owned(),
            three_ds_requestor_name: "Test Merchant".to_owned(),
            three_ds_requestor_url: "https://merchant.example.com".to_owned(),
            acct_number: "4111111111111111".to_owned(),
            card_expiry_date: "2612".to_owned(),
            three_ds_requestor_authentication_ind: None,
            three_ds_requestor_authentication_info: None,
            three_ds_requestor_challenge_ind: None,
            three_ds_requestor_prior_authentication_info: None,
            acct_type: None,
            acct_info: None,
            acct_id: None,
            purchase_amount: Some("1000".to_owned()),
            purchase_currency: Some("826".to_owned()),
            purchase_exponent: Some("2".to_owned()),
            purchase_date: Some("20261225120000".to_owned()),
            trans_type: Some("01".to_owned()),
            recurring_expiry: None,
            recurring_frequency: None,
            purchase_instal_data: None,
            merchant_id: None,
            mcc: None,
            merchant_name: None,
            merchant_country_code: None,
            merchant_risk_indicator: None,
            cardholder_name: None,
            email: None,
            home_phone: None,
            mobile_phone: None,
            work_phone: None,
            bill_addr_city: None,
            bill_addr_country: None,
            bill_addr_line1: None,
            bill_addr_line2: None,
            bill_addr_line3: None,
            bill_addr_post_code: None,
            bill_addr_state: None,
            ship_addr_city: None,
            ship_addr_country: None,
            ship_addr_line1: None,
            ship_addr_line2: None,
            ship_addr_line3: None,
            ship_addr_post_code: None,
            ship_addr_state: None,
            addr_match: None,
            three_ds_comp_ind: None,
            notification_url: Some("https://merchant.example.com/3ds/notify".to_owned()),
            browser_info: None,
            sdk_info: None,
            device_render_options: None,
        }
    }

    fn frictionless_ares(trans_id: &str, status: TransStatus) -> AuthenticationResponse {
        AuthenticationResponse {
            message_type: ares::MessageType::ARes,
            message_version: MessageVersion::V220,
            three_ds_server_trans_id: trans_id.to_owned(),
            acs_trans_id: "acs-001".to_owned(),
            ds_trans_id: "ds-001".to_owned(),
            trans_status: status,
            trans_status_reason: None,
            acs_challenge_mandated: None,
            eci: Some(Eci::VisaFullyAuthenticated),
            authentication_value: Some("abc123==".to_owned()),
            acs_url: None,
            acs_signed_content: None,
            acs_dec_con_ind: None,
            acs_reference_number: None,
            ds_reference_number: None,
            cardholder_info: None,
            whitelist_status: None,
            whitelist_status_source: None,
        }
    }

    fn challenge_ares(trans_id: &str) -> AuthenticationResponse {
        AuthenticationResponse {
            message_type: ares::MessageType::ARes,
            message_version: MessageVersion::V220,
            three_ds_server_trans_id: trans_id.to_owned(),
            acs_trans_id: "acs-001".to_owned(),
            ds_trans_id: "ds-001".to_owned(),
            trans_status: TransStatus::ChallengeRequired,
            trans_status_reason: None,
            acs_challenge_mandated: Some(ares::AcsMandated::Yes),
            eci: None,
            authentication_value: None,
            acs_url: Some("https://acs.bank.com/challenge".to_owned()),
            acs_signed_content: None,
            acs_dec_con_ind: None,
            acs_reference_number: None,
            ds_reference_number: None,
            cardholder_info: None,
            whitelist_status: None,
            whitelist_status_source: None,
        }
    }

    #[test]
    fn frictionless_success_flow() {
        let state = TransactionState::new(minimal_areq());
        let (state, _areq) = state.areq_sent().unwrap();
        let state = state
            .receive_ares(frictionless_ares("test-txn-id", TransStatus::Success))
            .unwrap();
        assert!(matches!(state, TransactionState::Authenticated { .. }));
        assert!(state.is_terminal());
    }

    #[test]
    fn frictionless_attempted_flow() {
        let state = TransactionState::new(minimal_areq());
        let (state, _) = state.areq_sent().unwrap();
        let state = state
            .receive_ares(frictionless_ares("test-txn-id", TransStatus::Attempted))
            .unwrap();
        assert!(matches!(state, TransactionState::Authenticated { .. }));
    }

    #[test]
    fn frictionless_failure_flow() {
        let state = TransactionState::new(minimal_areq());
        let (state, _) = state.areq_sent().unwrap();
        let state = state
            .receive_ares(frictionless_ares("test-txn-id", TransStatus::Failure))
            .unwrap();
        assert!(matches!(state, TransactionState::NotAuthenticated { .. }));
        assert!(state.is_terminal());
    }

    #[test]
    fn challenge_success_flow() {
        let state = TransactionState::new(minimal_areq());
        let (state, _) = state.areq_sent().unwrap();
        let state = state.receive_ares(challenge_ares("test-txn-id")).unwrap();
        assert!(matches!(state, TransactionState::AwaitingCRes { .. }));

        // Build and check the CReq
        let creq = state.build_creq(None).unwrap();
        assert_eq!(creq.three_ds_server_trans_id, "test-txn-id");
        assert_eq!(creq.acs_trans_id, "acs-001");

        let cres = ChallengeResponse {
            message_type: cres::MessageType::CRes,
            message_version: MessageVersion::V220,
            three_ds_server_trans_id: "test-txn-id".to_owned(),
            acs_trans_id: "acs-001".to_owned(),
            trans_status: TransStatus::Success,
            challenge_completion_ind: cres::CompletionIndicator::Complete,
            acs_ui: None,
            acs_ui_type: None,
            acs_html: None,
            whitelist_status: None,
        };

        let state = state.receive_cres(cres).unwrap();
        assert!(matches!(state, TransactionState::Authenticated { .. }));
    }

    #[test]
    fn trans_id_mismatch_is_error() {
        let state = TransactionState::new(minimal_areq());
        let (state, _) = state.areq_sent().unwrap();
        let result = state.receive_ares(frictionless_ares("wrong-id", TransStatus::Success));
        assert!(result.is_err());
    }

    #[test]
    fn invalid_transition_from_created() {
        let state = TransactionState::new(minimal_areq());
        let result = state.receive_ares(frictionless_ares("test-txn-id", TransStatus::Success));
        assert!(result.is_err());
    }

    #[test]
    fn error_message_transitions_to_failed() {
        let state = TransactionState::new(minimal_areq());
        let (state, _) = state.areq_sent().unwrap();
        let err = ErrorMessage {
            message_type: crate::message::error_msg::MessageType::Erro,
            message_version: MessageVersion::V220,
            error_code: "202".to_owned(),
            error_description: "Critical element missing".to_owned(),
            error_detail: "acctNumber".to_owned(),
            error_message_type: "AReq".to_owned(),
            three_ds_server_trans_id: None,
            acs_trans_id: None,
            ds_trans_id: None,
            sdk_trans_id: None,
        };
        let state = state.receive_error(&err);
        assert!(matches!(state, TransactionState::Failed { .. }));
        assert!(state.is_terminal());
    }
}
