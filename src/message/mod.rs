pub mod areq;
pub mod ares;
pub mod creq;
pub mod cres;
pub mod error_msg;
pub mod preq;
pub mod pres;
pub mod rreq;
pub mod rres;

pub use areq::AuthenticationRequest;
pub use ares::AuthenticationResponse;
pub use creq::ChallengeRequest;
pub use cres::ChallengeResponse;
pub use error_msg::ErrorMessage;
pub use preq::PreparationRequest;
pub use pres::{CardRangeData, PreparationResponse};
pub use rreq::{MessageExtension, ResultsRequest};
pub use rres::ResultsResponse;

use serde::{Deserialize, Serialize};

/// A type-erased EMV 3DS message, dispatched by the `messageType` field.
#[derive(Debug, Clone)]
pub enum Message {
    AReq(Box<AuthenticationRequest>),
    ARes(AuthenticationResponse),
    CReq(ChallengeRequest),
    CRes(ChallengeResponse),
    Erro(ErrorMessage),
    PReq(Box<PreparationRequest>),
    PRes(PreparationResponse),
    RReq(ResultsRequest),
    RRes(ResultsResponse),
}

impl Message {
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
            "PReq" => {
                let m: PreparationRequest = serde_json::from_value(peek)?;
                Ok(Self::PReq(Box::new(m)))
            }
            "PRes" => Ok(Self::PRes(serde_json::from_value(peek)?)),
            "RReq" => Ok(Self::RReq(serde_json::from_value(peek)?)),
            "RRes" => Ok(Self::RRes(serde_json::from_value(peek)?)),
            other => Err(crate::error::Error::Protocol {
                code: "405".to_owned(),
                description: format!("unknown messageType: {other}"),
            }),
        }
    }

    pub fn to_json(&self) -> crate::error::Result<String> {
        match self {
            Self::AReq(m) => serde_json::to_string(m.as_ref()),
            Self::ARes(m) => serde_json::to_string(m),
            Self::CReq(m) => serde_json::to_string(m),
            Self::CRes(m) => serde_json::to_string(m),
            Self::Erro(m) => serde_json::to_string(m),
            Self::PReq(m) => serde_json::to_string(m.as_ref()),
            Self::PRes(m) => serde_json::to_string(m),
            Self::RReq(m) => serde_json::to_string(m),
            Self::RRes(m) => serde_json::to_string(m),
        }
        .map_err(Into::into)
    }
}

impl Serialize for Message {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AReq(m) => m.serialize(s),
            Self::ARes(m) => m.serialize(s),
            Self::CReq(m) => m.serialize(s),
            Self::CRes(m) => m.serialize(s),
            Self::Erro(m) => m.serialize(s),
            Self::PReq(m) => m.serialize(s),
            Self::PRes(m) => m.serialize(s),
            Self::RReq(m) => m.serialize(s),
            Self::RRes(m) => m.serialize(s),
        }
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
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
            "PReq" => serde_json::from_value::<PreparationRequest>(v)
                .map(|m| Self::PReq(Box::new(m)))
                .map_err(serde::de::Error::custom),
            "PRes" => serde_json::from_value(v)
                .map(Self::PRes)
                .map_err(serde::de::Error::custom),
            "RReq" => serde_json::from_value(v)
                .map(Self::RReq)
                .map_err(serde::de::Error::custom),
            "RRes" => serde_json::from_value(v)
                .map(Self::RRes)
                .map_err(serde::de::Error::custom),
            other => Err(serde::de::Error::unknown_variant(
                other,
                &[
                    "AReq", "ARes", "CReq", "CRes", "Erro", "PReq", "PRes", "RReq", "RRes",
                ],
            )),
        }
    }
}
