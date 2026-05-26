use emv_3ds::message::{
    AuthenticationRequest, AuthenticationResponse, ChallengeRequest, ChallengeResponse,
    ErrorMessage,
};
use emv_3ds::types::{
    AcsAuthMethod, ActionIndicator, AuthenticationType, ChallengeWindowSize, DeviceChannel, Eci,
    MessageCategory, MessageVersion, ResultsStatus, TransStatus,
};

// ── AReq roundtrip ────────────────────────────────────────────────────────────

#[test]
fn areq_serializes_required_fields() {
    let areq = AuthenticationRequest {
        message_type: emv_3ds::message::areq::MessageType::AReq,
        message_version: MessageVersion::V220,
        three_ds_server_trans_id: "550e8400-e29b-41d4-a716-446655440000".to_owned(),
        device_channel: DeviceChannel::Browser,
        message_category: MessageCategory::PaymentAuthentication,
        three_ds_requestor_id: "REQ12345".to_owned(),
        three_ds_requestor_name: "Acme Remittance".to_owned(),
        three_ds_requestor_url: "https://acme.example.com".to_owned(),
        acct_number: "4111111111111111".to_owned(),
        card_expiry_date: "2812".to_owned(),
        purchase_amount: Some("5000".to_owned()),
        purchase_currency: Some("826".to_owned()),
        purchase_exponent: Some("2".to_owned()),
        purchase_date: Some("20261201120000".to_owned()),
        trans_type: Some("01".to_owned()),
        notification_url: Some("https://acme.example.com/3ds/notify".to_owned()),
        // all optional fields None
        three_ds_requestor_authentication_ind: None,
        three_ds_requestor_authentication_info: None,
        three_ds_requestor_challenge_ind: None,
        three_ds_requestor_prior_authentication_info: None,
        acct_type: None,
        acct_info: None,
        acct_id: None,
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
        browser_info: None,
        sdk_info: None,
        device_render_options: None,
    };

    let json = serde_json::to_string(&areq).unwrap();
    assert!(json.contains("\"messageType\":\"AReq\""));
    assert!(json.contains("\"messageVersion\":\"2.2.0\""));
    assert!(json.contains("\"deviceChannel\":\"02\""));
    assert!(json.contains("\"messageCategory\":\"01\""));

    // None fields must be omitted
    assert!(!json.contains("browserInfo"));
    assert!(!json.contains("sdkInfo"));

    // Roundtrip
    let decoded: AuthenticationRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(
        decoded.three_ds_server_trans_id,
        areq.three_ds_server_trans_id
    );
    assert_eq!(decoded.purchase_currency, Some("826".to_owned()));
}

// ── ARes roundtrip ────────────────────────────────────────────────────────────

#[test]
fn ares_frictionless_success_roundtrip() {
    let json = r#"{
        "messageType": "ARes",
        "messageVersion": "2.2.0",
        "threeDSServerTransID": "550e8400-e29b-41d4-a716-446655440000",
        "acsTransID":  "acs-trans-001",
        "dsTransID":   "ds-trans-001",
        "transStatus": "Y",
        "eci":         "05",
        "authenticationValue": "dGVzdA=="
    }"#;

    let ares: AuthenticationResponse = serde_json::from_str(json).unwrap();
    assert_eq!(ares.trans_status, TransStatus::Success);
    assert!(ares.is_frictionless());
    assert!(!ares.requires_challenge());
    assert_eq!(ares.eci, Some(Eci::VisaFullyAuthenticated));
    assert!(Eci::VisaFullyAuthenticated.has_liability_shift());

    let re_json = serde_json::to_string(&ares).unwrap();
    let re_ares: AuthenticationResponse = serde_json::from_str(&re_json).unwrap();
    assert_eq!(re_ares.authentication_value, ares.authentication_value);
}

#[test]
fn ares_challenge_required_roundtrip() {
    let json = r#"{
        "messageType": "ARes",
        "messageVersion": "2.2.0",
        "threeDSServerTransID": "txn-abc",
        "acsTransID":  "acs-abc",
        "dsTransID":   "ds-abc",
        "transStatus": "C",
        "acsMandated": "Y",
        "acsURL":      "https://acs.bank.example.com/challenge"
    }"#;

    let ares: AuthenticationResponse = serde_json::from_str(json).unwrap();
    assert!(ares.requires_challenge());
    assert_eq!(
        ares.acs_url.as_deref(),
        Some("https://acs.bank.example.com/challenge")
    );
}

// ── CReq roundtrip ────────────────────────────────────────────────────────────

#[test]
fn creq_serializes_window_size() {
    let creq = ChallengeRequest {
        message_type: emv_3ds::message::creq::MessageType::CReq,
        message_version: MessageVersion::V220,
        three_ds_server_trans_id: "txn-1".to_owned(),
        acs_trans_id: "acs-1".to_owned(),
        challenge_window_size: Some(ChallengeWindowSize::W500x600),
        challenge_data_entry: None,
        challenge_completion_ind: None,
        sdk_trans_id: None,
        resend_challenge: None,
        whitelist_status_source: None,
    };

    let json = serde_json::to_string(&creq).unwrap();
    assert!(json.contains("\"challengeWindowSize\":\"03\""));

    let decoded: ChallengeRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(
        decoded.challenge_window_size,
        Some(ChallengeWindowSize::W500x600)
    );
}

// ── CRes roundtrip ────────────────────────────────────────────────────────────

#[test]
fn cres_authenticated_roundtrip() {
    let json = r#"{
        "messageType": "CRes",
        "messageVersion": "2.2.0",
        "threeDSServerTransID": "txn-1",
        "acsTransID": "acs-1",
        "transStatus": "Y",
        "challengeCompletionInd": "Y"
    }"#;

    let cres: ChallengeResponse = serde_json::from_str(json).unwrap();
    assert!(cres.is_authenticated());
    assert_eq!(cres.trans_status, TransStatus::Success);
}

// ── Error message roundtrip ───────────────────────────────────────────────────

#[test]
fn error_message_roundtrip() {
    let json = r#"{
        "messageType": "Erro",
        "messageVersion": "2.2.0",
        "errorCode": "202",
        "errorDescription": "Critical element missing",
        "errorDetail": "acctNumber",
        "errorMessageType": "AReq"
    }"#;

    let err: ErrorMessage = serde_json::from_str(json).unwrap();
    assert_eq!(
        err.error_code,
        emv_3ds::message::error_msg::error_codes::CRITICAL_ELEMENT_MISSING
    );

    let re_json = serde_json::to_string(&err).unwrap();
    let re_err: ErrorMessage = serde_json::from_str(&re_json).unwrap();
    assert_eq!(re_err.error_description, err.error_description);
}

// ── MessageVersion display ────────────────────────────────────────────────────

#[test]
fn message_version_display() {
    assert_eq!(MessageVersion::V210.to_string(), "2.1.0");
    assert_eq!(MessageVersion::V220.to_string(), "2.2.0");
    assert_eq!(MessageVersion::V230.to_string(), "2.3.0");
}

// ── ECI liability shift ───────────────────────────────────────────────────────

#[test]
fn eci_liability_shift() {
    assert!(Eci::VisaFullyAuthenticated.has_liability_shift());
    assert!(Eci::VisaAttempted.has_liability_shift());
    assert!(!Eci::VisaNotAuthenticated.has_liability_shift());
    assert!(Eci::MastercardFullyAuthenticated.has_liability_shift());
    assert!(!Eci::MastercardNotAuthenticated.has_liability_shift());
}

// ── Amount helpers ────────────────────────────────────────────────────────────

#[test]
fn amount_spec_fields() {
    use emv_3ds::types::{Amount, Currency};
    let amount = Amount::from_major(50, Currency::GBP);
    assert_eq!(amount.spec_amount(), "5000");
    assert_eq!(amount.spec_currency(), "826");
    assert_eq!(amount.spec_exponent(), "2");
}

// ── PReq roundtrip ────────────────────────────────────────────────────────────

#[test]
fn preq_serializes_correctly() {
    let preq = emv_3ds::message::PreparationRequest {
        message_type: emv_3ds::message::preq::MessageType::PReq,
        message_version: MessageVersion::V220,
        three_ds_server_trans_id: "prep-txn-001".to_owned(),
        ds_trans_id: None,
        serial_num: Some("00001".to_owned()),
        three_ds_server_ref_number: None,
    };
    let json = serde_json::to_string(&preq).unwrap();
    assert!(json.contains("\"messageType\":\"PReq\""));
    assert!(json.contains("\"threeDSServerTransID\":\"prep-txn-001\""));
    assert!(json.contains("\"serialNum\":\"00001\""));
    assert!(!json.contains("dsTransID"));

    let decoded: emv_3ds::message::PreparationRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(
        decoded.three_ds_server_trans_id,
        preq.three_ds_server_trans_id
    );
    assert_eq!(decoded.serial_num, Some("00001".to_owned()));
}

// ── PRes roundtrip ────────────────────────────────────────────────────────────

#[test]
fn pres_roundtrip_with_card_ranges() {
    let json = r#"{
        "messageType": "PRes",
        "messageVersion": "2.2.0",
        "threeDSServerTransID": "prep-txn-001",
        "dsStartProtocolVersion": "2.1.0",
        "dsEndProtocolVersion": "2.2.0",
        "serialNum": "00002",
        "cardRangeData": [
            {
                "startRange": "4000000000000000",
                "endRange":   "4999999999999999",
                "actionInd":  "A",
                "acsStartProtocolVersion": "2.1.0",
                "acsEndProtocolVersion":   "2.2.0",
                "threeDSMethodURL": "https://acs.bank.example.com/3ds/method"
            }
        ]
    }"#;

    let pres: emv_3ds::message::PreparationResponse = serde_json::from_str(json).unwrap();
    assert_eq!(pres.card_range_data.len(), 1);
    assert_eq!(pres.card_range_data[0].action_ind, ActionIndicator::Add);
    assert_eq!(
        pres.card_range_data[0].three_ds_method_url.as_deref(),
        Some("https://acs.bank.example.com/3ds/method")
    );
    assert!(pres.range_for_pan("4111111111111111").is_some());
    assert!(pres.range_for_pan("5111111111111111").is_none());

    let re_json = serde_json::to_string(&pres).unwrap();
    let re_pres: emv_3ds::message::PreparationResponse = serde_json::from_str(&re_json).unwrap();
    assert_eq!(re_pres.serial_num, pres.serial_num);
}

// ── RReq roundtrip ────────────────────────────────────────────────────────────

#[test]
fn rreq_roundtrip() {
    let json = r#"{
        "messageType": "RReq",
        "messageVersion": "2.2.0",
        "threeDSServerTransID": "txn-001",
        "acsTransID": "acs-001",
        "dsTransID": "ds-001",
        "messageCategory": "01",
        "transStatus": "Y",
        "authenticationMethod": "02",
        "authenticationType": "01",
        "authenticationValue": "dGVzdA==",
        "eci": "05",
        "interactionCounter": "01"
    }"#;

    let rreq: emv_3ds::message::ResultsRequest = serde_json::from_str(json).unwrap();
    assert_eq!(rreq.trans_status, TransStatus::Success);
    assert_eq!(rreq.eci, Some(Eci::VisaFullyAuthenticated));
    assert_eq!(rreq.authentication_method, Some(AcsAuthMethod::SmsOtp));
    assert_eq!(
        rreq.authentication_type,
        Some(AuthenticationType::Frictionless)
    );

    let re_json = serde_json::to_string(&rreq).unwrap();
    let re_rreq: emv_3ds::message::ResultsRequest = serde_json::from_str(&re_json).unwrap();
    assert_eq!(re_rreq.authentication_value, rreq.authentication_value);
}

// ── RRes roundtrip + acknowledge builder ─────────────────────────────────────

#[test]
fn rres_acknowledge_and_roundtrip() {
    let rreq = emv_3ds::message::ResultsRequest {
        message_type: emv_3ds::message::rreq::MessageType::RReq,
        message_version: MessageVersion::V220,
        three_ds_server_trans_id: "txn-001".to_owned(),
        acs_trans_id: "acs-001".to_owned(),
        ds_trans_id: Some("ds-001".to_owned()),
        sdk_trans_id: None,
        message_category: MessageCategory::PaymentAuthentication,
        trans_status: TransStatus::Success,
        trans_status_reason: None,
        authentication_method: Some(AcsAuthMethod::PushNotification),
        authentication_type: Some(AuthenticationType::ChallengeOob),
        authentication_value: Some("abc==".to_owned()),
        eci: Some(Eci::VisaFullyAuthenticated),
        interaction_counter: Some("01".to_owned()),
        acs_rendering_type: None,
        message_extension: None,
    };

    let rres = emv_3ds::message::ResultsResponse::acknowledge(&rreq);
    assert_eq!(rres.three_ds_server_trans_id, "txn-001");
    assert_eq!(rres.acs_trans_id, "acs-001");
    assert_eq!(rres.ds_trans_id.as_deref(), Some("ds-001"));
    assert_eq!(rres.results_status, ResultsStatus::Received);

    let json = serde_json::to_string(&rres).unwrap();
    assert!(json.contains("\"messageType\":\"RRes\""));
    assert!(json.contains("\"resultsStatus\":\"01\""));

    let decoded: emv_3ds::message::ResultsResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.results_status, ResultsStatus::Received);
}
