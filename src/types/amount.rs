use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// ISO 4217 currency code (numeric, zero-padded to 3 digits).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Currency(u16);

impl Currency {
    pub const GBP: Self = Self(826);
    pub const USD: Self = Self(840);
    pub const EUR: Self = Self(978);
    pub const NGN: Self = Self(566);
    pub const ETB: Self = Self(230);
    pub const KES: Self = Self(404);
    pub const GHS: Self = Self(936);
    pub const INR: Self = Self(356);
    pub const PHP: Self = Self(608);
    pub const MXN: Self = Self(484);
    pub const MAD: Self = Self(504);
    pub const AED: Self = Self(784);
    pub const SAR: Self = Self(682);

    pub fn new(code: u16) -> Result<Self> {
        if code > 999 {
            return Err(Error::InvalidField {
                field: "currency",
                reason: format!("ISO 4217 numeric code must be 0–999, got {code}"),
            });
        }
        Ok(Self(code))
    }

    pub fn code(self) -> u16 {
        self.0
    }

    /// Returns the zero-padded 3-digit string as required by the EMVCo spec.
    pub fn as_spec_string(self) -> String {
        format!("{:03}", self.0)
    }
}

/// A monetary amount expressed in minor units (e.g. pence, cents) with
/// a currency and the number of minor-unit decimal places.
///
/// The EMVCo spec requires `purchaseAmount` as a numeric string (max 48 digits)
/// and `purchaseExponent` as the number of digits after the decimal point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Amount {
    /// Minor-unit value (e.g. 1099 for £10.99)
    pub minor_units: u64,
    pub currency: Currency,
    /// Decimal exponent per ISO 4217 (e.g. 2 for GBP → £X.XX)
    pub exponent: u8,
}

impl Amount {
    pub fn new(minor_units: u64, currency: Currency, exponent: u8) -> Self {
        Self {
            minor_units,
            currency,
            exponent,
        }
    }

    /// Convenience: construct from major units with a 2-digit exponent.
    pub fn from_major(major: u64, currency: Currency) -> Self {
        Self {
            minor_units: major * 100,
            currency,
            exponent: 2,
        }
    }

    /// EMVCo `purchaseAmount` field value.
    pub fn spec_amount(&self) -> String {
        self.minor_units.to_string()
    }

    /// EMVCo `purchaseCurrency` field value.
    pub fn spec_currency(&self) -> String {
        self.currency.as_spec_string()
    }

    /// EMVCo `purchaseExponent` field value.
    pub fn spec_exponent(&self) -> String {
        self.exponent.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_spec_string_zero_pads() {
        assert_eq!(Currency::new(826).unwrap().as_spec_string(), "826");
        assert_eq!(Currency::new(8).unwrap().as_spec_string(), "008");
        assert_eq!(Currency::new(78).unwrap().as_spec_string(), "078");
    }

    #[test]
    fn currency_rejects_out_of_range() {
        assert!(Currency::new(1000).is_err());
    }

    #[test]
    fn amount_spec_fields() {
        let a = Amount::from_major(10, Currency::GBP);
        assert_eq!(a.spec_amount(), "1000");
        assert_eq!(a.spec_currency(), "826");
        assert_eq!(a.spec_exponent(), "2");
    }
}
