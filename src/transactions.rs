use crate::ffi::{string_to_ptr, log_message};
use crate::utils::log;
use serde_json;

#[no_mangle]
pub extern "C" fn execute_credit_leg(amount_ptr: *const u8, amount_len: usize, account_ptr: *const u8, account_len: usize) -> *const u8 {
    let amount = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(amount_ptr, amount_len)) };
    let account = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(account_ptr, account_len)) };
    
    let query = crate::query::generate_balance_query(account);
    
    let query_ptr = crate::ffi::alloc(query.len());
    unsafe {
        std::ptr::copy_nonoverlapping(query.as_ptr(), query_ptr, query.len());
    }
    query_ptr as *const u8
}

#[no_mangle]
pub extern "C" fn process_credit_result(result_ptr: *const u8, result_len: usize, amount_ptr: *const u8, amount_len: usize) -> *const u8 {
    let result = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(result_ptr, result_len)) };
    let amount = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(amount_ptr, amount_len)) };
    
    log(&format!("Processing result: {}, amount: {}", result, amount));

    // ... (rest of the function remains the same)
}

#[no_mangle]
pub extern "C" fn execute_debit_leg(amount_ptr: *const u8, amount_len: usize, account_ptr: *const u8, account_len: usize) -> *const u8 {
    let amount = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(amount_ptr, amount_len)) };
    let account = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(account_ptr, account_len)) };
    
    log(&format!("Executing debit leg: amount = {}, account = {}", amount, account));

    let result = format!("Debiting {} from account {}", amount, account);
    log(&result);

    string_to_ptr(&result)
}