use ic_cdk_macros::*;
use crate::models::listing::Listing;
use crate::auth::init_admin;
use candid::Principal;
use crate::storage::STORAGE;

mod models;
mod storage;
mod auth;
mod services;

#[init]
fn init() {
    ic_cdk::setup();
    // Initialize the creator as the admin
    init_admin();
    
    // Additional initialization if needed
    STORAGE.with(|s| {
        let mut storage = s.borrow_mut();
        // Storage initialization can be done here
    });
}

ic_cdk::export_candid!();