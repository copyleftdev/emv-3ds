# emv-3ds

**EMV 3-D Secure 2.x protocol implementation in Rust.**

A zero-dependency\* Rust crate providing the complete message layer and transaction
state machine for the EMV 3DS 2.x specification — the global standard used by Visa
(3DS 2.0/2.2), Mastercard (Identity Check), American Express (SafeKey 2.0),
and all major card networks to perform strong customer authentication (SCA) during
card-not-present payments.

[![Crates.io](https://img.shields.io/crates/v/emv-3ds)](https://crates.io/crates/emv-3ds)
[![docs.rs](https://img.shields.io/docsrs/emv-3ds)](https://docs.rs/emv-3ds)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust 1.75+](https://img.shields.io/badge/rust-1.75%2B-orange)](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)

\*Runtime dependencies are `serde`, `serde_json`, `uuid`, `thiserror`, and `chrono` only.

---

## What is EMV 3-D Secure?

EMV 3DS (3-D Secure version 2.x) is the payment authentication protocol defined by
[EMVCo](https://www.emvco.com/emv-technologies/3d-secure/) that enables issuers to
verify cardholder identity during e-commerce transactions without redirecting to a
static password page.

The protocol involves three parties:

| Role | Abbreviation | Responsibility |
|------|-------------|----------------|
| 3DS Server | 3DSS | Merchant-side component; sends AReq, receives ARes |
| Directory Server | DS | Card-scheme routing layer (Visa, Mastercard) |
| Access Control Server | ACS | Issuer-side component; authenticates cardholder |

A transaction follows one of two paths:

```
Frictionless:  3DSS ──AReq──► DS ──► ACS ──ARes(Y/A)──► 3DSS
Challenge:     3DSS ──AReq──► DS ──► ACS ──ARes(C)───► 3DSS
                        browser ──CReq──► ACS ──CRes(Y/N)──► 3DSS
```

---

## Features

- **All five protocol messages** — `AReq`, `ARes`, `CReq`, `CRes`, `Erro` with
  every field from the EMVCo 3DS Core Specification 2.3.
- **Correct wire format** — `#[serde(rename)]` for every acronym field
  (`threeDSServerTransID`, `acsTransID`, `dsTransID`, `acsURL`) that
  `rename_all = "camelCase"` would mangle.
- **Transaction state machine** — type-safe lifecycle from `Created` through
  `AwaitingARes` → `AwaitingCRes` → `Authenticated` / `NotAuthenticated` / `Failed`,
  with invalid-transition errors.
- **Coded value enums** — `TransStatus`, `Eci`, `ChallengeIndicator`,
  `MessageVersion`, `TransStatusReason` (21 codes), and all other spec enumerations.
- **ECI / liability shift helpers** — `Eci::has_liability_shift()`,
  `TransStatus::is_authenticated()`, `AuthenticationResponse::requires_challenge()`.
- **ISO 4217 currency** — `Currency` newtype with zero-padded spec string,
  `Amount` with `spec_amount` / `spec_currency` / `spec_exponent` EMVCo field getters.
- **Message envelope** — `Message` enum with `from_json` / `to_json` that peeks at
  `messageType` for dispatch without duplicating the field on the wire.
- **Quality-gated** — 47 tests (unit + integration + proptest), 0 `cargo-mutants`
  survivors, `clippy -D warnings` clean.

---

## Quick start

```toml
[dependencies]
emv-3ds = "0.1"
```

### Build and send an AReq

```rust
use emv_3ds::message::areq::{AuthenticationRequest, MessageType};
use emv_3ds::types::{DeviceChannel, MessageCategory, MessageVersion};

let areq = AuthenticationRequest {
    message_type: MessageType::AReq,
    message_version: MessageVersion::V220,
    three_ds_server_trans_id: uuid::Uuid::new_v4().to_string(),
    device_channel: DeviceChannel::Browser,
    message_category: MessageCategory::PaymentAuthentication,
    three_ds_requestor_id: "your-requestor-id".into(),
    three_ds_requestor_name: "Acme Payments".into(),
    three_ds_requestor_url: "https://acme.example.com".into(),
    acct_number: "4111111111111111".into(),
    card_expiry_date: "2812".into(),
    notification_url: Some("https://acme.example.com/3ds/notify".into()),
    // ...optional fields omitted from JSON via skip_serializing_if
    ..Default::default()
};

let json = serde_json::to_string(&areq)?;
// POST json to the Directory Server endpoint
```

### Drive the state machine

```rust
use emv_3ds::transaction::TransactionState;
use emv_3ds::types::ChallengeWindowSize;

// 1. Create transaction
let state = TransactionState::new(areq);

// 2. Send the AReq — state machine gives you back the serialized message
let (state, outbound_areq) = state.areq_sent()?;
// → POST serde_json::to_string(&outbound_areq) to the DS

// 3. Receive the ARes
let ares: emv_3ds::message::AuthenticationResponse = serde_json::from_str(&ds_response)?;
let state = state.receive_ares(ares)?;

match &state {
    TransactionState::Authenticated { eci, authentication_value, .. } => {
        // Frictionless success — attach ECI + CAVV to auth request
    }
    TransactionState::AwaitingCRes { acs_url, .. } => {
        // Redirect browser to acs_url for challenge
        let creq = state.build_creq(Some(ChallengeWindowSize::W500x600))?;
        // POST serde_json::to_string(&creq) to acs_url
    }
    TransactionState::NotAuthenticated { .. } => {
        // Decline or soft-decline
    }
    _ => {}
}

// 4. After challenge: receive CRes
let cres: emv_3ds::message::ChallengeResponse = serde_json::from_str(&acs_response)?;
let state = state.receive_cres(cres)?;
```

### Parse any incoming message

```rust
use emv_3ds::message::Message;

let msg = Message::from_json(&raw_json)?;
match msg {
    Message::ARes(ares) => { /* handle */ }
    Message::Erro(err)  => { /* abort transaction */ }
    _ => {}
}
```

---

## Message types

| Struct | Wire name | Direction | Purpose |
|--------|-----------|-----------|---------|
| `AuthenticationRequest` | `AReq` | 3DSS → DS → ACS | Initiate authentication |
| `AuthenticationResponse` | `ARes` | ACS → DS → 3DSS | Outcome or challenge redirect |
| `ChallengeRequest` | `CReq` | Browser/SDK → ACS | Submit challenge data |
| `ChallengeResponse` | `CRes` | ACS → 3DSS | Challenge outcome |
| `ErrorMessage` | `Erro` | Any → Any | Protocol error |

---

## Transaction state machine

```
Created
  │  areq_sent()
  ▼
AwaitingARes
  │  receive_ares()
  ├─ Y/A ──────────────────────► Authenticated  (terminal)
  ├─ N/U/I/R ──────────────────► NotAuthenticated  (terminal)
  └─ C/D
       │
       ▼
     AwaitingCRes
       │  receive_cres()
       ├─ Y ────────────────────► Authenticated  (terminal)
       └─ N/U ──────────────────► NotAuthenticated  (terminal)

Any state + receive_error() ──► Failed  (terminal)
```

---

## ECI values

| Constant | Wire | Network | Meaning | Liability shift |
|----------|------|---------|---------|----------------|
| `Eci::VisaFullyAuthenticated` | `05` | Visa | Full 3DS auth | ✓ |
| `Eci::VisaAttempted` | `06` | Visa | Attempted processing | ✓ |
| `Eci::VisaNotAuthenticated` | `07` | Visa | Failed / not enrolled | ✗ |
| `Eci::MastercardFullyAuthenticated` | `02` | Mastercard | Full 3DS auth | ✓ |
| `Eci::MastercardAttempted` | `01` | Mastercard | Attempted processing | ✓ |
| `Eci::MastercardNotAuthenticated` | `00` | Mastercard | Failed / not enrolled | ✗ |

---

## Roadmap

- [ ] **PReq / PRes** — Directory Server preparation request (card range negotiation)
- [ ] **RReq / RRes** — Results request for decoupled and app-based authentication
- [ ] **JWE envelope** — EMVCo-mandated end-to-end encryption for `acctNumber` and
  `sdkEncData` fields
- [ ] **DS certificate management** — JWK / PKCS#12 signing for AReq integrity

---

## Spec conformance

This crate targets the **EMVCo 3DS Core Specification v2.2.0 and v2.3.0**.
The spec is available (registration required) at
[emvco.com](https://www.emvco.com/emv-technologies/3d-secure/).

EMVCo 3DS is an open standard with no royalty requirements on implementations.

---

## License

MIT — see [LICENSE](LICENSE).
