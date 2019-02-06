#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol/1.1.0/")]

//! # AMQP manipulation library
//!
//! amq-protocol is a library aiming at providing tools to help
//! implementing software using AMQP

/// Reexport of amq_protocol_codegen
pub mod codegen;
/// AMQP Frame handling utils
pub mod frame;
/// The AMQ Protocol implementation (Generated)
pub mod protocol;
/// Reexport of amq_protocol_types
pub mod types;
/// AMQP Uri utils
pub mod uri;
