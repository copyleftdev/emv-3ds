//! # emv-3ds
//!
//! EMV 3-D Secure 2.x protocol — message types, state machine, and
//! transaction lifecycle for the 3DS Server role.
//!
//! Covers versions 2.1.0, 2.2.0, and 2.3.0 of the EMVCo specification.
//!
//! ## Roles
//! This crate models the **3DS Server** side: constructing AReqs, parsing
//! ARes/CRes responses, and driving the transaction state machine.
//! It does not implement the Directory Server or ACS roles.

pub mod error;
pub mod message;
pub mod transaction;
pub mod types;

pub use error::{Error, Result};
