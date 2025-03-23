use ic_cdk_macros::*;
use crate::models::{Listing, Quote, Platform};
use crate::storage::STORAGE;
use crate::auth::is_manager_or_admin;
use ic_cdk::println;

#[update]
pub fn upsert_listing(args: Listing) -> Result<(), String> {
    is_manager_or_admin()?;

    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let symbol = args.symbol.to_uppercase();
        println!("Attempting to upsert listing for symbol: {}", symbol);

        if let Some(existing_listing) = storage.listings.get_mut(&symbol) {
            println!("Updating existing listing for symbol: {}", symbol);
            existing_listing.name = args.name;
            existing_listing.slug = args.slug;
            existing_listing.cmc_rank = args.cmc_rank;
            existing_listing.num_market_pairs = args.num_market_pairs;
            existing_listing.circulating_supply = args.circulating_supply;
            existing_listing.total_supply = args.total_supply;
            existing_listing.max_supply = args.max_supply;
            existing_listing.infinite_supply = args.infinite_supply;
            existing_listing.date_added = args.date_added;
            existing_listing.tags = args.tags;
            existing_listing.platform = args.platform;
            existing_listing.self_reported_circulating_supply = args.self_reported_circulating_supply;
            existing_listing.self_reported_market_cap = args.self_reported_market_cap;

            existing_listing.quote.price = args.quote.price;
            existing_listing.quote.volume_24h = args.quote.volume_24h;
            existing_listing.quote.volume_change_24h = args.quote.volume_change_24h;
            existing_listing.quote.percent_change_1h = args.quote.percent_change_1h;
            existing_listing.quote.percent_change_24h = args.quote.percent_change_24h;
            existing_listing.quote.percent_change_7d = args.quote.percent_change_7d;
            existing_listing.quote.market_cap = args.quote.market_cap;
            existing_listing.quote.market_cap_dominance = args.quote.market_cap_dominance;
            existing_listing.quote.fully_diluted_market_cap = args.quote.fully_diluted_market_cap;
            existing_listing.quote.last_updated = args.quote.last_updated;
            existing_listing.last_updated = args.last_updated;
        } else {
            let listing = Listing {
                id: args.id,
                symbol: args.symbol.clone(),
                name: args.name,
                slug: args.slug,
                cmc_rank: args.cmc_rank,
                num_market_pairs: args.num_market_pairs,
                circulating_supply: args.circulating_supply,
                total_supply: args.total_supply,
                max_supply: args.max_supply,
                infinite_supply: args.infinite_supply,
                date_added: args.date_added,
                tags: args.tags,
                platform: args.platform.map(|data| Platform { id: data.id, name: data.name, symbol: data.symbol, slug: data.slug, token_address: data.token_address }),
                self_reported_circulating_supply: args.self_reported_circulating_supply,
                self_reported_market_cap: args.self_reported_market_cap,
                quote: Quote {
                    price: args.quote.price,
                    volume_24h: args.quote.volume_24h,
                    volume_change_24h: args.quote.volume_change_24h,
                    percent_change_1h: args.quote.percent_change_1h,
                    percent_change_24h: args.quote.percent_change_24h,
                    percent_change_7d: args.quote.percent_change_7d,
                    percent_change_30d: args.quote.percent_change_30d,
                    percent_change_60d: args.quote.percent_change_60d,
                    percent_change_90d: args.quote.percent_change_90d,
                    market_cap: args.quote.market_cap,
                    market_cap_dominance: args.quote.market_cap_dominance,
                    fully_diluted_market_cap: args.quote.fully_diluted_market_cap,
                    last_updated: args.quote.last_updated,
                },
                last_updated: args.last_updated,
            };
            println!("Inserting new listing for symbol: {}", symbol);
            storage.listings.insert(symbol, listing);
        }
        println!("Current listings count after upsert: {}", storage.listings.len());
        Ok(())
    })
}

#[update]
pub fn upsert_listings(args: Vec<Listing>) -> Result<(), String> {
    is_manager_or_admin()?;

    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();

        for arg in args {
            let symbol = arg.symbol.to_uppercase();

            if let Some(existing_listing) = storage.listings.get_mut(&symbol) {
                println!("Updating listing for symbol: {}", symbol.clone());
                existing_listing.name = arg.name;
                existing_listing.slug = arg.slug;
                existing_listing.cmc_rank = arg.cmc_rank;
                existing_listing.num_market_pairs = arg.num_market_pairs;
                existing_listing.circulating_supply = arg.circulating_supply;
                existing_listing.total_supply = arg.total_supply;
                existing_listing.max_supply = arg.max_supply;
                existing_listing.infinite_supply = arg.infinite_supply;
                existing_listing.date_added = arg.date_added;
                existing_listing.tags = arg.tags;
                existing_listing.platform = arg.platform;
                existing_listing.self_reported_circulating_supply = arg.self_reported_circulating_supply;
                existing_listing.self_reported_market_cap = arg.self_reported_market_cap;

                existing_listing.quote.price = arg.quote.price;
                existing_listing.quote.volume_24h = arg.quote.volume_24h;
                existing_listing.quote.volume_change_24h = arg.quote.volume_change_24h;
                existing_listing.quote.percent_change_1h = arg.quote.percent_change_1h;
                existing_listing.quote.percent_change_24h = arg.quote.percent_change_24h;
                existing_listing.quote.percent_change_7d = arg.quote.percent_change_7d;
                existing_listing.quote.market_cap = arg.quote.market_cap;
                existing_listing.quote.market_cap_dominance = arg.quote.market_cap_dominance;
                existing_listing.quote.fully_diluted_market_cap = arg.quote.fully_diluted_market_cap;
                existing_listing.quote.last_updated = arg.quote.last_updated;

                existing_listing.last_updated = arg.last_updated;
            } else {
                let listing = Listing {
                    id: arg.id,
                    symbol: arg.symbol.clone(),
                    name: arg.name,
                    slug: arg.slug,
                    cmc_rank: arg.cmc_rank,
                    num_market_pairs: arg.num_market_pairs,
                    circulating_supply: arg.circulating_supply,
                    total_supply: arg.total_supply,
                    max_supply: arg.max_supply,
                    infinite_supply: arg.infinite_supply,
                    date_added: arg.date_added,
                    tags: arg.tags,
                    platform: arg.platform.map(|data| Platform { id: data.id, name: data.name, symbol: data.symbol, slug: data.slug, token_address: data.token_address }),
                    self_reported_circulating_supply: arg.self_reported_circulating_supply,
                    self_reported_market_cap: arg.self_reported_market_cap,
                    quote: Quote {
                        price: arg.quote.price,
                        volume_24h: arg.quote.volume_24h,
                        volume_change_24h: arg.quote.volume_change_24h,
                        percent_change_1h: arg.quote.percent_change_1h,
                        percent_change_24h: arg.quote.percent_change_24h,
                        percent_change_7d: arg.quote.percent_change_7d,
                        percent_change_30d: arg.quote.percent_change_30d,
                        percent_change_60d: arg.quote.percent_change_60d,
                        percent_change_90d: arg.quote.percent_change_90d,
                        market_cap: arg.quote.market_cap,
                        market_cap_dominance: arg.quote.market_cap_dominance,
                        fully_diluted_market_cap: arg.quote.fully_diluted_market_cap,
                        last_updated: arg.quote.last_updated,
                    },
                    last_updated: arg.last_updated,
                };
                storage.listings.insert(symbol.clone(), listing);
                println!("Inserted new listing for symbol: {}", symbol);
            }
        }
        println!("Current listings count: {}", storage.listings.len());
        Ok(())
    })
}

#[query]
pub fn get_listing(symbol: String) -> Option<Listing> {
    let symbol = symbol.to_uppercase();
    STORAGE.with(|storage| {
        let s = storage.borrow();
        let keys = s.listings.keys().collect::<Vec<_>>();
        println!("Querying symbol: {}, available keys: {:?}", symbol, keys);
        let result = s.listings.get(&symbol);
        println!("Raw get result for {}: {:?}", symbol, result);
        let cloned_result = result.cloned();
        println!("Cloned result for {}: {:?}", symbol, cloned_result);
        cloned_result
    })
}

#[query]
pub fn get_listings(limit: Option<u64>, offset: Option<u64>) -> Vec<Listing> {
    // Process parameters
    let real_limit = match limit {
        Some(l) => {
            // Ensure limit parameter is valid and not zero
            if l == 0 {
                10 // Default return 10 items
            } else if l > 100 {
                100 // Maximum return 100 items
            } else {
                l as usize
            }
        },
        None => 20, // If not specified, default return 20 items
    };
    let real_offset = offset.map(|o| o as usize).unwrap_or(0);
    
    ic_cdk::println!("get_listings with effective limit: {}, offset: {}", real_limit, real_offset);
    
    STORAGE.with(|storage| {
        let mut listings: Vec<Listing> = storage.borrow().listings.values().cloned().collect();
        
        // Sort by cmc_rank (ascending order, lower rank is better)
        listings.sort_by(|a, b| a.cmc_rank.cmp(&b.cmc_rank));
        
        // Apply pagination
        if real_offset >= listings.len() {
            return vec![];
        }
        
        // Calculate end index
        let end_index = std::cmp::min(real_offset + real_limit, listings.len());
        
        // Extract listings within specified range
        let result = listings[real_offset..end_index].to_vec();
        ic_cdk::println!("Returning {} listings", result.len());
        
        result
    })
}

#[update]
pub fn delete_listing(symbol: String) -> Result<(), String> {
    is_manager_or_admin()?;
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.listings.remove(&symbol).is_none() {
            return Err("Listing not found".to_string());
        }
        Ok(())
    })
}
