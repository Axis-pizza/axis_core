#![no_std]

pub mod error;

#[cfg(feature = "bpf-entrypoint")]
pub mod entrypoint;

pub mod processor;

pub use error::AxisCoreError;
