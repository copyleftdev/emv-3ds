use emv_3ds::message::{
    areq, ares, cres, AuthenticationRequest, AuthenticationResponse, ChallengeResponse,
    ErrorMessage,
};
use emv_3ds::transaction::TransactionState;
use emv_3ds::types::{
    Amount, ChallengeWindowSize, Currency, DeviceChannel, Eci, MessageCategory, MessageVersion,
    TransStatus,
};
use proptest::prelude::*;

// ── Strategies ────────────────────────────────────────────────────────────────

fn arb_trans_status() -> impl Strategy<Value = TransStatus> {
    prop_oneof![
        Just(TransStatus::Success),
        Just(TransStatus::Failure),
        Just(TransStatus::Unable),
        Just(TransStatus::Attempted),
        Just(TransStatus::ChallengeRequired),
        Just(TransStatus::DecoupledRequired),
        Just(TransStatus::InformationalOnly),
        Just(TransStatus::Rejected),
    ]
}

fn arb_auth_status() -> impl Strategy<Value = TransStatus> {
    prop_oneof![Just(TransStatus::Success), Just(TransStatus::Attempted)]
}

fn arb_challenge_status() -> impl Strategy<Value = TransStatus> {
    prop_oneof![
        Just(TransStatus::ChallengeRequired),
        Just(TransStatus::DecoupledRequired),
    ]
}

fn arb_non_auth_non_challenge_status() -> impl Strategy<Value = TransStatus> {
    prop_oneof![
        Just(TransStatus::Failure),
        Just(TransStatus::Unable),
        Just(TransStatus::InformationalOnly),
        Just(TransStatus::Rejected),
    ]
}

fn arb_eci() -> impl Strategy<Value = Eci> {
    prop_oneof![
        Just(Eci::VisaFullyAuthenticated),
        Just(Eci::VisaAttempted),
        Just(Eci::VisaNotAuthenticated),
        Just(Eci::MastercardFullyAuthenticated),
        Just(Eci::MastercardAttempted),
        Just(Eci::MastercardNotAuthenticated),
    ]
}

fn arb_message_version() -> impl Strategy<Value = MessageVersion> {
    prop_oneof![
        Just(MessageVersion::V210),
        Just(MessageVersion::V220),
        Just(MessageVersion::V230),
    ]
}

fn arb_device_channel() -> impl Strategy<Value = DeviceChannel> {
    prop_oneof![
        Just(DeviceChannel::App),
        Just(DeviceChannel::Browser),
        Just(DeviceChannel::ThreeDsRequestorInitiated),
    ]
}

fn arb_message_category() -> impl Strategy<Value = MessageCategory> {
    prop_oneof![
        Just(MessageCategory::PaymentAuthentication),
        Just(MessageCategory::NonPaymentAuthentication),
    ]
}

fn arb_window_size() -> impl Strategy<Value = ChallengeWindowSize> {
    prop_oneof![
        Just(ChallengeWindowSize::W250x400),
        Just(ChallengeWindowSize::W390x400),
        Just(ChallengeWindowSize::W500x600),
        Just(ChallengeWindowSize::W600x400),
        Just(ChallengeWindowSize::FullScreen),
    ]
}

// ── Fixtures ──────────────────────────────────────────────────────────────────

fn minimal_areq() -> AuthenticationRequest {
    AuthenticationRequest {
        message_type: areq::MessageType::AReq,
        message_version: MessageVersion::V220,
        three_ds_server_trans_id: "txn-prop".to_owned(),
        device_channel: DeviceChannel::Browser,
        message_category: MessageCategory::PaymentAuthentication,
        three_ds_requestor_id: "req-001".to_owned(),
        three_ds_requestor_name: "PropTest Merchant".to_owned(),
        three_ds_requestor_url: "https://merchant.example.com".to_owned(),
        acct_number: "4111111111111111".to_owned(),
        card_expiry_date: "2812".to_owned(),
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
        purchase_date: Some("20261201120000".to_owned()),
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

fn make_ares(status: TransStatus) -> AuthenticationResponse {
    AuthenticationResponse {
        message_type: ares::MessageType::ARes,
        message_version: MessageVersion::V220,
        three_ds_server_trans_id: "txn-prop".to_owned(),
        acs_trans_id: "acs-prop".to_owned(),
        ds_trans_id: "ds-prop".to_owned(),
        trans_status: status,
        trans_status_reason: None,
        acs_challenge_mandated: None,
        eci: None,
        authentication_value: None,
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

fn make_failed_cres() -> ChallengeResponse {
    ChallengeResponse {
        message_type: cres::MessageType::CRes,
        message_version: MessageVersion::V220,
        three_ds_server_trans_id: "txn-prop".to_owned(),
        acs_trans_id: "acs-prop".to_owned(),
        trans_status: TransStatus::Failure,
        challenge_completion_ind: cres::CompletionIndicator::Complete,
        acs_ui: None,
        acs_ui_type: None,
        acs_html: None,
        whitelist_status: None,
    }
}

// ── TransStatus semantics ─────────────────────────────────────────────────────

proptest! {
    /// Kills: `is_authenticated -> true/false` and any single-variant omission.
    #[test]
    fn is_authenticated_spec(s in arb_trans_status()) {
        let want = matches!(s, TransStatus::Success | TransStatus::Attempted);
        prop_assert_eq!(s.is_authenticated(), want);
    }

    /// Kills: `requires_challenge -> true/false` and `DecoupledRequired` omission.
    #[test]
    fn requires_challenge_spec(s in arb_trans_status()) {
        let want = matches!(s, TransStatus::ChallengeRequired | TransStatus::DecoupledRequired);
        prop_assert_eq!(s.requires_challenge(), want);
    }

    /// is_authenticated and requires_challenge are mutually exclusive.
    #[test]
    fn authenticated_and_challenge_are_disjoint(s in arb_trans_status()) {
        prop_assert!(!(s.is_authenticated() && s.requires_challenge()));
    }

    /// Kills: `has_liability_shift -> true/false` and `MastercardAttempted` omission.
    #[test]
    fn has_liability_shift_spec(e in arb_eci()) {
        let want = matches!(
            e,
            Eci::VisaFullyAuthenticated
                | Eci::VisaAttempted
                | Eci::MastercardFullyAuthenticated
                | Eci::MastercardAttempted
        );
        prop_assert_eq!(e.has_liability_shift(), want);
    }
}

// ── ARes helper semantics ─────────────────────────────────────────────────────

proptest! {
    /// Kills: `AuthenticationResponse::is_frictionless -> true`.
    #[test]
    fn ares_is_frictionless_spec(s in arb_trans_status()) {
        let ares = make_ares(s);
        prop_assert_eq!(ares.is_frictionless(), s.is_authenticated());
    }

    /// Kills: `AuthenticationResponse::requires_challenge -> true/false`.
    #[test]
    fn ares_requires_challenge_spec(s in arb_trans_status()) {
        let ares = make_ares(s);
        prop_assert_eq!(ares.requires_challenge(), s.requires_challenge());
    }
}

// ── CRes helper semantics ─────────────────────────────────────────────────────

/// Kills: `ChallengeResponse::is_authenticated -> true` (no-failure case was already covered).
#[test]
fn cres_failed_challenge_is_not_authenticated() {
    assert!(!make_failed_cres().is_authenticated());
}

// ── Serde roundtrips ──────────────────────────────────────────────────────────

proptest! {
    #[test]
    fn trans_status_serde_roundtrip(s in arb_trans_status()) {
        let json = serde_json::to_string(&s).unwrap();
        let back: TransStatus = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(s, back);
    }

    #[test]
    fn eci_serde_roundtrip(e in arb_eci()) {
        let json = serde_json::to_string(&e).unwrap();
        let back: Eci = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(e, back);
    }

    #[test]
    fn message_version_serde_roundtrip(v in arb_message_version()) {
        let json = serde_json::to_string(&v).unwrap();
        let back: MessageVersion = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(v, back);
    }

    /// Kills: `MessageVersion::as_str -> ""/xyzzy` and the `Display` fmt mutant.
    #[test]
    fn message_version_display_equals_as_str(v in arb_message_version()) {
        prop_assert_eq!(v.to_string(), v.as_str());
    }

    #[test]
    fn device_channel_serde_roundtrip(c in arb_device_channel()) {
        let json = serde_json::to_string(&c).unwrap();
        let back: DeviceChannel = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(c, back);
    }

    #[test]
    fn message_category_serde_roundtrip(c in arb_message_category()) {
        let json = serde_json::to_string(&c).unwrap();
        let back: MessageCategory = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(c, back);
    }

    #[test]
    fn challenge_window_size_serde_roundtrip(w in arb_window_size()) {
        let json = serde_json::to_string(&w).unwrap();
        let back: ChallengeWindowSize = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(w, back);
    }
}

// ── Message envelope to_json/from_json ───────────────────────────────────────

/// Kills: `Message::to_json -> Ok(String::new())` and `Ok("xyzzy".into())`.
#[test]
fn message_to_json_from_json_roundtrip() {
    use emv_3ds::message::Message;

    let ares = make_ares(TransStatus::Success);
    let msg = Message::ARes(ares.clone());

    let json = msg.to_json().expect("to_json failed");
    assert!(
        json.contains("ARes"),
        "to_json must produce real JSON, got: {json}"
    );

    let back = Message::from_json(&json).expect("from_json failed");
    match back {
        Message::ARes(r) => assert_eq!(r.three_ds_server_trans_id, ares.three_ds_server_trans_id),
        other => panic!("expected ARes, got {other:?}"),
    }
}

// ── AuthenticationRequest::message_type_value ─────────────────────────────────

/// Kills: `message_type_value -> ""/xyzzy`.
#[test]
fn message_type_value_is_areq() {
    assert_eq!(AuthenticationRequest::message_type_value(), "AReq");
}

// ── Currency::code ────────────────────────────────────────────────────────────

proptest! {
    /// Kills: `Currency::code -> 0` and `-> 1`.
    #[test]
    fn currency_code_roundtrips(raw in 0u16..=999u16) {
        let c = Currency::new(raw).unwrap();
        prop_assert_eq!(c.code(), raw);
    }

    #[test]
    fn currency_spec_string_is_3_chars(raw in 0u16..=999u16) {
        let s = Currency::new(raw).unwrap().as_spec_string();
        prop_assert_eq!(s.len(), 3);
    }

    /// Kills: `Currency::as_spec_string -> ""/xyzzy` (complements existing unit test).
    #[test]
    fn currency_spec_string_parses_back_to_same_code(raw in 0u16..=999u16) {
        let c = Currency::new(raw).unwrap();
        let s = c.as_spec_string();
        let parsed: u16 = s.parse().unwrap();
        prop_assert_eq!(parsed, raw);
    }

    #[test]
    fn currency_rejects_values_above_999(raw in 1000u16..=u16::MAX) {
        prop_assert!(Currency::new(raw).is_err());
    }

    /// Kills: `Amount::from_major * → +` and `* → /`.
    #[test]
    fn amount_from_major_minor_units(major in 0u64..=100_000u64) {
        let a = Amount::from_major(major, Currency::GBP);
        prop_assert_eq!(a.minor_units, major * 100);
        prop_assert_eq!(a.exponent, 2);
    }

    /// Kills: `Amount::spec_amount -> ""/xyzzy`.
    #[test]
    fn amount_spec_amount_is_minor_units_string(minor in 0u64..=10_000_000u64) {
        let a = Amount::new(minor, Currency::GBP, 2);
        prop_assert_eq!(a.spec_amount(), minor.to_string());
    }

    /// Kills: `Amount::spec_exponent -> ""/xyzzy`.
    #[test]
    fn amount_spec_exponent_matches_field(exp in 0u8..=9u8) {
        let a = Amount::new(100, Currency::GBP, exp);
        prop_assert_eq!(a.spec_exponent(), exp.to_string());
    }
}

// ── State machine: all ARes transition paths ──────────────────────────────────

proptest! {
    /// Kills: any mutation that misroutes authenticated statuses.
    #[test]
    fn ares_auth_status_gives_authenticated(status in arb_auth_status()) {
        let state = TransactionState::new(minimal_areq());
        let (state, _) = state.areq_sent().unwrap();
        let next = state.receive_ares(make_ares(status)).unwrap();
        let is_auth = matches!(next, TransactionState::Authenticated { .. });
        prop_assert!(is_auth);
        prop_assert!(next.is_terminal());
    }

    /// Kills: `DecoupledRequired` omission in the challenge arm of receive_ares.
    #[test]
    fn ares_challenge_status_gives_awaiting_cres(status in arb_challenge_status()) {
        let state = TransactionState::new(minimal_areq());
        let (state, _) = state.areq_sent().unwrap();
        let next = state.receive_ares(make_ares(status)).unwrap();
        let is_cres = matches!(next, TransactionState::AwaitingCRes { .. });
        prop_assert!(is_cres);
        prop_assert!(!next.is_terminal());
    }

    /// Kills: mutations that misroute failure/unable/rejected/informational statuses.
    #[test]
    fn ares_failure_status_gives_not_authenticated(status in arb_non_auth_non_challenge_status()) {
        let state = TransactionState::new(minimal_areq());
        let (state, _) = state.areq_sent().unwrap();
        let next = state.receive_ares(make_ares(status)).unwrap();
        let is_not_auth = matches!(next, TransactionState::NotAuthenticated { .. });
        prop_assert!(is_not_auth);
        prop_assert!(next.is_terminal());
    }
}

// ── State machine: is_terminal consistency ────────────────────────────────────

/// Kills: `is_terminal -> true` and `is_terminal -> false`.
#[test]
fn is_terminal_for_each_state() {
    let state = TransactionState::new(minimal_areq());
    assert!(!state.is_terminal(), "Created must not be terminal");

    let (awaiting, _) = state.areq_sent().unwrap();
    assert!(!awaiting.is_terminal(), "AwaitingARes must not be terminal");

    let challenge = awaiting
        .receive_ares(make_ares(TransStatus::ChallengeRequired))
        .unwrap();
    assert!(
        !challenge.is_terminal(),
        "AwaitingCRes must not be terminal"
    );

    let auth = TransactionState::new(minimal_areq())
        .areq_sent()
        .unwrap()
        .0
        .receive_ares(make_ares(TransStatus::Success))
        .unwrap();
    assert!(auth.is_terminal(), "Authenticated must be terminal");

    let not_auth = TransactionState::new(minimal_areq())
        .areq_sent()
        .unwrap()
        .0
        .receive_ares(make_ares(TransStatus::Failure))
        .unwrap();
    assert!(not_auth.is_terminal(), "NotAuthenticated must be terminal");

    let err_msg = ErrorMessage {
        message_type: emv_3ds::message::error_msg::MessageType::Erro,
        message_version: MessageVersion::V220,
        error_code: "500".to_owned(),
        error_description: "System error".to_owned(),
        error_detail: "internal".to_owned(),
        error_message_type: "AReq".to_owned(),
        three_ds_server_trans_id: None,
        acs_trans_id: None,
        ds_trans_id: None,
        sdk_trans_id: None,
    };
    let failed = TransactionState::new(minimal_areq())
        .areq_sent()
        .unwrap()
        .0
        .receive_error(&err_msg);
    assert!(failed.is_terminal(), "Failed must be terminal");
}

// ── State machine: InvalidTransition carries the correct state name ───────────

/// Kills: `TransactionState::name -> ""/xyzzy`.
#[test]
fn invalid_transition_error_names_current_state() {
    let created = TransactionState::new(minimal_areq());
    let err = created
        .receive_ares(make_ares(TransStatus::Success))
        .unwrap_err();
    match err {
        emv_3ds::Error::InvalidTransition { from, .. } => {
            assert_eq!(from, "Created", "error must name the actual current state");
        }
        other => panic!("expected InvalidTransition, got {other:?}"),
    }
}
