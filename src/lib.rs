// lib.rs
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod ffi;
pub mod transactions;
pub mod query;
mod utils;
mod http;

// Re-export main functions for easier use
pub use transactions::{execute_credit_leg, process_credit_result, execute_debit_leg};
pub use query::generate_balance_query;
pub use http::handle_http_request;