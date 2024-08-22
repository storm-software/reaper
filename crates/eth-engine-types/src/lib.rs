//! # Reaper Ethereum Engine Types
//! This crate contains the types used by the Reaper Ethereum Engine.
//!
//! ## Features
//! - `serde`: Enables serialization and deserialization of types using Serde.
//! - `tracing`: Enables logging using the `tracing` crate.

#![feature(trait_alias)]
#![feature(trivial_bounds)]
#![feature(const_type_id)]
#![feature(core_intrinsics)]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![allow(internal_features)]
#![allow(clippy::type_complexity)]

pub mod block_metadata;
pub mod constants;
pub mod db;
pub mod executor;
pub mod serde_utils;
pub mod structured_trace;
