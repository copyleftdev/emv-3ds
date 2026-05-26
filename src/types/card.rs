use serde::{Deserialize, Serialize};

/// Type of account being used for the transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "01")]
    NotApplicable,
    #[serde(rename = "02")]
    Credit,
    #[serde(rename = "03")]
    Debit,
}

/// Cardholder account information used for risk assessment.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// Length of time that the cardholder has had the account (01–04).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_acc_age_ind: Option<String>,
    /// Date that the cardholder opened the account (YYYYMMDD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_acc_date: Option<String>,
    /// Length of time since the last change to the account (01–04).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_acc_change_ind: Option<String>,
    /// Date the last account change occurred (YYYYMMDD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_acc_change: Option<String>,
    /// Length of time since the password last changed (01–04).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_acc_pw_change_ind: Option<String>,
    /// Number of transactions in the past 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_purchase_account: Option<String>,
    /// Number of add-card attempts in the past 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_attempts_day: Option<String>,
    /// Number of transactions in the past year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn_activity_year: Option<String>,
    /// Number of transactions in the previous 6 months.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn_activity_day: Option<String>,
    /// Date that the shipping address was first used (YYYYMMDD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_address_usage: Option<String>,
    /// Indicator for when the shipping address was first used (01–04).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_address_usage_ind: Option<ShippingAddressUsage>,
    /// Name on account vs shipping name match indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_name_indicator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_acc_activity: Option<super::coded::SuspiciousAccActivity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShippingAddressUsage {
    #[serde(rename = "01")]
    ThisTransaction,
    #[serde(rename = "02")]
    LessThan30Days,
    #[serde(rename = "03")]
    ThirtyTo60Days,
    #[serde(rename = "04")]
    MoreThan60Days,
}

/// BIN range for card identification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardRange {
    pub start_acct_range: String,
    pub end_acct_range: String,
}
