//! # `skuld`
//!
//! `skuld` is a pure-Rust asynchronous client library for the LDAP protocol. It is
//! still very early in development so expect changes often!
//!
//! ## Examples
//!
//! TODO: when code exists

#![warn(missing_docs)]
#![allow(dead_code, clippy::match_bool)]

pub mod ber;
mod protocol_types;
mod search_filter;
