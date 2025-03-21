use ic_cdk_macros::*;
use ic_cdk::api::time;
use crate::models::{language::Language, channel::Channel};
use crate::storage::STORAGE;
use crate::auth::is_manager_or_admin;

#[update]
pub fn create_language(
    language: String,
    language_code: String,
    country_code: String,
    enabled: bool,
) -> Result<String, String> {
    is_manager_or_admin()?;
    let timestamp = time();
    let new_language = Language {
        language: language.clone(),
        language_code,
        country_code,
        enabled,
        updated_at: timestamp,
    };

    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.languages.contains_key(&language) {
            return Err("Language already exists".to_string());
        }
        storage.languages.insert(language.clone(), new_language);
        Ok(language)
    })
}

#[update]
pub fn create_languages(languages: Vec<(String, String, String, bool)>) -> Result<Vec<String>, String> {
    is_manager_or_admin()?;
    let timestamp = time();
    let mut results = Vec::new();

    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        for (language, language_code, country_code, enabled) in languages {
            if storage.languages.contains_key(&language) {
                return Err(format!("Language '{}' already exists", language));
            }
            let new_language = Language {
                language: language.clone(),
                language_code,
                country_code,
                enabled,
                updated_at: timestamp,
            };
            storage.languages.insert(language.clone(), new_language);
            results.push(language);
        }
        Ok(results)
    })
}

#[update]
pub fn delete_language(language: String) -> Result<(), String> {
    is_manager_or_admin()?;
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.languages.remove(&language).is_some() {
            Ok(())
        } else {
            Err("Language not found".to_string())
        }
    })
}

#[query]
pub fn get_languages(enabled_only: Option<bool>) -> Vec<Language> {
    STORAGE.with(|storage| {
        let storage_ref = storage.borrow();
        if enabled_only.unwrap_or(false) {
            storage_ref.languages.values().filter(|lang| lang.enabled).cloned().collect()
        } else {
            storage_ref.languages.values().cloned().collect()
        }
    })
}

#[update]
pub fn update_language(
    language: String,
    new_language: Option<String>,
    language_code: Option<String>,
    country_code: Option<String>,
    enabled: Option<bool>
) -> Result<(), String> {
    is_manager_or_admin()?;
    let timestamp = time();

    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut lang = match storage.languages.remove(&language) {
            Some(l) => l,
            None => return Err("Language not found".to_string()),
        };

        if let Some(new_lang) = new_language { lang.language = new_lang; }
        if let Some(lc) = language_code { lang.language_code = lc; }
        if let Some(cc) = country_code { lang.country_code = cc; }
        if let Some(e) = enabled { lang.enabled = e; }
        lang.updated_at = timestamp;

        storage.languages.insert(lang.language.clone(), lang);
        Ok(())
    })
}

#[update]
pub fn create_channel(name: String, platform: String, enabled: bool) -> Result<String, String> {
    is_manager_or_admin()?;
    let valid_platforms = vec!["telegram", "x"];
    if !valid_platforms.contains(&platform.as_str()) {
        return Err("Invalid platform. Must be 'telegram' or 'x'".to_string());
    }
    let timestamp = time();
    let key = format!("{}_{}", name, platform);
    let new_channel = Channel {
        name: name.clone(),
        platform: platform.clone(),
        enabled,
        updated_at: timestamp,
    };

    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.channels.contains_key(&key) {
            return Err("Channel already exists".to_string());
        }
        storage.channels.insert(key.clone(), new_channel);
        Ok(key)
    })
}

#[update]
pub fn create_channels(channels: Vec<(String, String, bool)>) -> Result<Vec<String>, String> {
    is_manager_or_admin()?;
    let valid_platforms = vec!["telegram", "x"];
    let timestamp = time();
    let mut results = Vec::new();

    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        for (name, platform, enabled) in channels {
            if !valid_platforms.contains(&platform.as_str()) {
                return Err("Invalid platform. Must be 'telegram' or 'x'".to_string());
            }
            let key = format!("{}_{}", name, platform);
            if storage.channels.contains_key(&key) {
                return Err(format!("Channel '{}' already exists", key));
            }
            let new_channel = Channel {
                name: name.clone(),
                platform: platform.clone(),
                enabled,
                updated_at: timestamp,
            };
            storage.channels.insert(key.clone(), new_channel);
            results.push(key);
        }
        Ok(results)
    })
}

#[update]
pub fn delete_channel(name: String, platform: String) -> Result<(), String> {
    is_manager_or_admin()?;
    let key = format!("{}_{}", name, platform);
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.channels.remove(&key).is_some() {
            Ok(())
        } else {
            Err("Channel not found".to_string())
        }
    })
}

#[query]
pub fn get_channels(platform: Option<String>) -> Vec<Channel> {
    STORAGE.with(|storage| {
        let storage_ref = storage.borrow();
        match platform {
            Some(p) => storage_ref.channels.values().filter(|ch| ch.platform == p).cloned().collect(),
            None => storage_ref.channels.values().cloned().collect(),
        }
    })
}