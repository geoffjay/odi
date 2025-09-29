//! # ODI Binary Crate Library
//!
//! Shared utilities and types for the ODI command-line interface.

pub mod cli;
pub mod commands;
pub mod error;

pub use error::{OdiError, Result};