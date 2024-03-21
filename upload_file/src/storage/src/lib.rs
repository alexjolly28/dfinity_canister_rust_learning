use candid::Principal;
use ic_cdk_macros::export_candid;
use std::collections::HashMap;

pub mod asset_handler;
pub mod canister_method;
pub mod chunk_handler;
pub mod http_handler;
pub mod init;
pub mod memory;
pub mod state;
mod test;
pub mod utils;
// pub mod candid_file_generator;

pub use asset_handler::*;
pub use chunk_handler::*;
pub use http_handler::*;

export_candid!();
