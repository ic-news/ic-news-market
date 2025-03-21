use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use ic_cdk::storage;
use ic_cdk_macros::*;
use crate::models::{language::Language, channel::Channel};

#[derive(CandidType, Serialize, Deserialize, Default)]
pub struct Storage {
    pub languages: HashMap<String, Language>,
    pub channels: HashMap<String, Channel>,
    pub managers: HashSet<Principal>,
    pub admin: Option<Principal>,
}

thread_local! {
    pub static STORAGE: std::cell::RefCell<Storage> = std::cell::RefCell::new(Storage::default());
}

#[pre_upgrade]
pub fn pre_upgrade() {
    STORAGE.with(|storage| {
        storage::stable_save((storage.borrow().clone(),)).unwrap();
    });
}

#[post_upgrade]
pub fn post_upgrade() {
    let (old_storage,): (Storage,) = storage::stable_restore().unwrap();
    STORAGE.with(|storage| {
        *storage.borrow_mut() = old_storage;
    });
}

impl Clone for Storage {
    fn clone(&self) -> Self {
        Storage {
            languages: self.languages.clone(),
            channels: self.channels.clone(),
            managers: self.managers.clone(),
            admin: self.admin.clone(),
        }
    }
}