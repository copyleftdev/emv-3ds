pub mod areq;
pub mod ares;
pub mod creq;
pub mod cres;
pub mod error_msg;

pub use areq::AuthenticationRequest;
pub use ares::AuthenticationResponse;
pub use creq::ChallengeRequest;
pub use cres::ChallengeResponse;
pub use error_msg::ErrorMessage;

use serde::{Deserialize, Serialize};

/// A type-erased EMV 3DS message, dispatched by the `messageType` field.
///
/// Each inner struct already carries `messageType` per the spec.  The `Message`
/// envelope must NOT duplicate that field, so serialization delegates directly to
/// the inner struct and deserialization peeks at `messageType` before dispatching.
#[derive(Debug, Clone)]
pub enum Message {
    AReq(Box<AuthenticationRequest>),
    ARes(AuthenticationResponse),
    CReq(ChallengeRequest),
    CRes(ChallengeResponse),
    Erro(ErrorMessage),
}

impl Message {
    /// Parse an incoming JSON payload into the appropriate message variant.
    ///
    /// Peeks at the `messageType` field first, then deserializes the full object
    /// into the matching concrete type.
    pub fn from_json(json: &str) -> crate::error::Result<Self> {
        let peek: serde_json::Value = serde_json::from_str(json)?;
        let tag = peek
            .get("messageType")
            .and_then(|v| v.as_str())
            .ok_or(crate::error::Error::MissingField("messageType"))?;

        match tag {
            "AReq" => {
                let m: AuthenticationRequest = serde_json::from_value(peek)?;
                Ok(Self::AReq(Box::new(m)))
            }
            "ARes" => Ok(Self::ARes(serde_json::from_value(peek)?)),
            "CReq" => Ok(Self::CReq(serde_json::from_value(peek)?)),
            "CRes" => Ok(Self::CRes(serde_json::from_value(peek)?)),
            "Erro" => Ok(Self::Erro(serde_json::from_value(peek)?)),
            other => Err(crate::error::Error::Protocol {
                code: "405".to_owned(),
                description: format!("unknown messageType: {other}"),
            }),
        }
    }

    /// Serialize this message to a JSON string.
    ///
    /// Each inner struct already includes `messageType` in its serialized form, so
    /// this simply delegates to the inner struct's `Serialize` implementation.
    pub fn to_json(&self) -> crate::error::Result<String> {
        match self {
            Self::AReq(m) => serde_json::to_string(m.as_ref()),
            Self::ARes(m) => serde_json::to_string(m),
            Self::CReq(m) => serde_json::to_string(m),
            Self::CRes(m) => serde_json::to_string(m),
            Self::Erro(m) => serde_json::to_string(m),
        }
        .map_err(Into::into)
    }
}

// Manual Serialize/Deserialize: delegate to the inner struct so that
// `messageType` appears exactly once in the wire format.
impl Serialize for Message {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AReq(m) => m.serialize(s),
            Self::ARes(m) => m.serialize(s),
            Self::CReq(m) => m.serialize(s),
            Self::CRes(m) => m.serialize(s),
            Self::Erro(m) => m.serialize(s),
        }
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        // Buffer the whole object, then dispatch on messageType.
        let v = serde_json::Value::deserialize(d)?;
        let tag = v
            .get("messageType")
            .and_then(|t| t.as_str())
            .ok_or_else(|| serde::de::Error::missing_field("messageType"))?;

        match tag {
            "AReq" => serde_json::from_value::<AuthenticationRequest>(v)
                .map(|m| Self::AReq(Box::new(m)))
                .map_err(serde::de::Error::custom),
            "ARes" => serde_json::from_value(v)
                .map(Self::ARes)
                .map_err(serde::de::Error::custom),
            "CReq" => serde_json::from_value(v)
                .map(Self::CReq)
                .map_err(serde::de::Error::custom),
            "CRes" => serde_json::from_value(v)
                .map(Self::CRes)
                .map_err(serde::de::Error::custom),
            "Erro" => serde_json::from_value(v)
                .map(Self::Erro)
                .map_err(serde::de::Error::custom),
            other => Err(serde::de::Error::unknown_variant(
                other,
                &["AReq", "ARes", "CReq", "CRes", "Erro"],
            )),
        }
    }
}
