//! defines the compiler for Erg (ergc).
extern crate erg_common;
pub extern crate erg_parser;

mod compile;
pub use compile::*;
mod codegen;
pub mod effectcheck;
pub mod error;
pub mod eval;
pub mod hir;
pub mod initialize;
pub mod lower;
pub use lower::ASTLowerer;
pub mod context;
pub mod optimize;
pub mod ownercheck;
pub mod varinfo;
