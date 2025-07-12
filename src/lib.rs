//! EOS Delegation Protocol
//! 
//! This crate implements an efficient outsourcing scheme for SNARKs (EOS)
//! that allows delegation of computational tasks while preserving privacy
//! and ensuring verifiability.

pub mod circuit;
pub mod mpc;
pub mod piop;
pub mod protocol;
pub mod evaluation;
pub mod custom_circuits;
pub mod comprehensive_tests;

pub use circuit::*;
pub use mpc::*;
pub use piop::*;
pub use protocol::*;
pub use evaluation::*;
pub use comprehensive_tests::*;
