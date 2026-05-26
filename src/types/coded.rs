use serde::{Deserialize, Serialize};

/// EMV 3DS specification version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageVersion {
    #[serde(rename = "2.1.0")]
    V210,
    #[serde(rename = "2.2.0")]
    V220,
    #[serde(rename = "2.3.0")]
    V230,
}

impl MessageVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V210 => "2.1.0",
            Self::V220 => "2.2.0",
            Self::V230 => "2.3.0",
        }
    }
}

impl std::fmt::Display for MessageVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Device channel through which the transaction originates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceChannel {
    /// 01 — App-based (native SDK)
    #[serde(rename = "01")]
    App,
    /// 02 — Browser
    #[serde(rename = "02")]
    Browser,
    /// 03 — 3DS Requestor Initiated (3RI)
    #[serde(rename = "03")]
    ThreeDsRequestorInitiated,
}

/// Category of authentication being requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageCategory {
    /// 01 — Payment Authentication
    #[serde(rename = "01")]
    PaymentAuthentication,
    /// 02 — Non-Payment Authentication
    #[serde(rename = "02")]
    NonPaymentAuthentication,
}

/// Outcome of the authentication transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransStatus {
    /// Y — Authentication / Account Verification Successful
    #[serde(rename = "Y")]
    Success,
    /// N — Not Authenticated / Account Not Verified
    #[serde(rename = "N")]
    Failure,
    /// U — Authentication / Account Verification Could Not Be Performed
    #[serde(rename = "U")]
    Unable,
    /// A — Attempts Processing Performed
    #[serde(rename = "A")]
    Attempted,
    /// C — Challenge Required; additional authentication needed
    #[serde(rename = "C")]
    ChallengeRequired,
    /// D — Decoupled Authentication Confirmed
    #[serde(rename = "D")]
    DecoupledRequired,
    /// I — Informational Only; 3DS Requestor challenge preference acknowledged
    #[serde(rename = "I")]
    InformationalOnly,
    /// R — Authentication / Account Verification Rejected
    #[serde(rename = "R")]
    Rejected,
}

impl TransStatus {
    /// True if the authentication was successful (frictionless Y or attempted A).
    pub fn is_authenticated(self) -> bool {
        matches!(self, Self::Success | Self::Attempted)
    }

    /// True if a challenge must be presented to the cardholder.
    pub fn requires_challenge(self) -> bool {
        matches!(self, Self::ChallengeRequired | Self::DecoupledRequired)
    }
}

/// Reason code accompanying a non-successful TransStatus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransStatusReason {
    #[serde(rename = "01")]
    CardAuthenticationFailed,
    #[serde(rename = "02")]
    UnknownDevice,
    #[serde(rename = "03")]
    UnsupportedDevice,
    #[serde(rename = "04")]
    ExceedsAuthenticationFrequencyLimit,
    #[serde(rename = "05")]
    ExpiredCard,
    #[serde(rename = "06")]
    InvalidCardNumber,
    #[serde(rename = "07")]
    InvalidTransaction,
    #[serde(rename = "08")]
    NoCardRecord,
    #[serde(rename = "09")]
    SecurityFailure,
    #[serde(rename = "10")]
    StolenCard,
    #[serde(rename = "11")]
    SuspectedFraud,
    #[serde(rename = "12")]
    TransactionNotPermittedToCardholder,
    #[serde(rename = "13")]
    CardholderNotEnrolledInService,
    #[serde(rename = "14")]
    TransactionTimedOutAtAcs,
    #[serde(rename = "15")]
    LowConfidence,
    #[serde(rename = "16")]
    MediumConfidence,
    #[serde(rename = "17")]
    HighConfidence,
    #[serde(rename = "18")]
    VeryHighConfidence,
    #[serde(rename = "19")]
    ExceedsMaxChallengesPerTransaction,
    #[serde(rename = "20")]
    NonPaymentNotSupported,
    #[serde(rename = "21")]
    ThreeDsRequestorChallengeIndicator3NotAccepted,
}

/// Electronic Commerce Indicator — liability shift outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Eci {
    /// Visa: fully authenticated
    #[serde(rename = "05")]
    VisaFullyAuthenticated,
    /// Visa: attempted
    #[serde(rename = "06")]
    VisaAttempted,
    /// Visa: not authenticated / no liability shift
    #[serde(rename = "07")]
    VisaNotAuthenticated,
    /// Mastercard: fully authenticated
    #[serde(rename = "02")]
    MastercardFullyAuthenticated,
    /// Mastercard: attempted
    #[serde(rename = "01")]
    MastercardAttempted,
    /// Mastercard: not authenticated
    #[serde(rename = "00")]
    MastercardNotAuthenticated,
}

impl Eci {
    /// True if this ECI value represents a successful liability shift.
    pub fn has_liability_shift(self) -> bool {
        matches!(
            self,
            Self::VisaFullyAuthenticated
                | Self::VisaAttempted
                | Self::MastercardFullyAuthenticated
                | Self::MastercardAttempted
        )
    }
}

/// Requestor's preference for whether a challenge should be performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChallengeIndicator {
    /// 01 — No preference
    #[serde(rename = "01")]
    NoPreference,
    /// 02 — No challenge requested
    #[serde(rename = "02")]
    NoChallengeRequested,
    /// 03 — Challenge requested (requestor preference)
    #[serde(rename = "03")]
    ChallengeRequested,
    /// 04 — Challenge requested (mandate)
    #[serde(rename = "04")]
    ChallengeRequestedMandate,
    /// 05 — No challenge (transactional risk analysis already performed)
    #[serde(rename = "05")]
    NoChallengeRiskAnalysisPerformed,
    /// 06 — No challenge (Data Share Only)
    #[serde(rename = "06")]
    NoChallengeDataShareOnly,
    /// 07 — No challenge (SCA already performed)
    #[serde(rename = "07")]
    NoChallengeSCAAlreadyPerformed,
    /// 08 — No challenge (whitelisted)
    #[serde(rename = "08")]
    NoChallengeWhitelisted,
    /// 09 — Challenge requested (whitelisting prompt requested)
    #[serde(rename = "09")]
    ChallengeRequestedWhitelistPrompt,
}

/// Dimensions of the challenge window presented in the browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChallengeWindowSize {
    #[serde(rename = "01")]
    W250x400,
    #[serde(rename = "02")]
    W390x400,
    #[serde(rename = "03")]
    W500x600,
    #[serde(rename = "04")]
    W600x400,
    #[serde(rename = "05")]
    FullScreen,
}

/// How the 3DS Method was used to collect device fingerprint data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreeDsMethod {
    /// 00 — 3DS Method Not Available / Requestor chose not to use
    #[serde(rename = "00")]
    NotAvailableOrNotUsed,
    /// 01 — 3DS Method URL was present and successfully invoked
    #[serde(rename = "01")]
    Successful,
    /// 02 — 3DS Method URL was present but failed or timed out
    #[serde(rename = "02")]
    Failed,
}

/// How the cardholder authenticates to the 3DS Requestor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    #[serde(rename = "01")]
    NoThreeDsRequestorAuthentication,
    #[serde(rename = "02")]
    LoginToRequestorAccount,
    #[serde(rename = "03")]
    FederatedId,
    #[serde(rename = "04")]
    IssuerCredentials,
    #[serde(rename = "05")]
    ThirdPartyAuthentication,
    #[serde(rename = "06")]
    FidoAuthenticator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeliveryTimeframe {
    #[serde(rename = "01")]
    ElectronicDelivery,
    #[serde(rename = "02")]
    SameDayShipping,
    #[serde(rename = "03")]
    OvernightShipping,
    #[serde(rename = "04")]
    TwoDayOrMoreShipping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShipIndicator {
    #[serde(rename = "01")]
    ShipToCardholdersBillingAddress,
    #[serde(rename = "02")]
    ShipToAnotherVerifiedAddress,
    #[serde(rename = "03")]
    ShipToDifferentAddress,
    #[serde(rename = "04")]
    ShipToStore,
    #[serde(rename = "05")]
    DigitalGoods,
    #[serde(rename = "06")]
    NotShipped,
    #[serde(rename = "07")]
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReorderItemsIndicator {
    #[serde(rename = "01")]
    FirstTimeOrdered,
    #[serde(rename = "02")]
    Reordered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PreOrderPurchaseIndicator {
    #[serde(rename = "01")]
    MerchandiseAvailable,
    #[serde(rename = "02")]
    FutureAvailability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SuspiciousAccActivity {
    #[serde(rename = "01")]
    NoSuspiciousActivity,
    #[serde(rename = "02")]
    SuspiciousActivityObserved,
}
