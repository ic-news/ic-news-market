use candid::{CandidType, Principal};
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use ic_cdk::storage;
use ic_cdk_macros::*;
use crate::models::listing::Listing;

#[derive(Default, CandidType, Deserialize)]
pub struct Storage {
    pub listings: HashMap<String, Listing>,
    pub admin: Option<Principal>,
    pub managers: HashSet<Principal>,
}

thread_local! {
    pub static STORAGE: RefCell<Storage> = RefCell::new(Storage::default());
}

#[pre_upgrade]
pub fn pre_upgrade() {
    STORAGE.with(|storage| {
        if let Err(e) = storage::stable_save((storage.borrow().clone(),)) {
            ic_cdk::trap(&format!("Failed to save state: {:?}", e));
        }
    });
}

#[post_upgrade]
pub fn post_upgrade() {
    match storage::stable_restore::<(Storage,)>() {
        Ok((old_storage,)) => {
            STORAGE.with(|storage| {
                *storage.borrow_mut() = old_storage;
            });
        }
        Err(e) => {
            ic_cdk::println!("Failed to restore state: {:?}. Initializing with default.", e);
            STORAGE.with(|storage| {
                *storage.borrow_mut() = Storage::default();
            });
        }
    }
}

impl Clone for Storage {
    fn clone(&self) -> Self {
        Storage {
            listings: self.listings.clone(),
            admin: self.admin.clone(),
            managers: self.managers.clone(),
        }
    }
}