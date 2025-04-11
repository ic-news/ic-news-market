use ic_cdk_macros::*;
use crate::models::{Listing, Quote, Platform};
use crate::storage::STORAGE;
use crate::auth::is_manager_or_admin;
use regex::Regex;

#[update]
pub fn upsert_listing(args: Listing) -> Result<(), String> {
    is_manager_or_admin()?;

    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let symbol = args.symbol.to_uppercase();

        if let Some(existing_listing) = storage.listings.get_mut(&symbol) {
            existing_listing.name = args.name;
            existing_listing.slug = args.slug;
            existing_listing.cmc_rank = args.cmc_rank;
            existing_listing.num_market_pairs = args.num_market_pairs;
            existing_listing.circulating_supply = args.circulating_supply;
            existing_listing.total_supply = args.total_supply;
            existing_listing.max_supply = args.max_supply;
            existing_listing.infinite_supply = args.infinite_supply;
            existing_listing.date_added = args.date_added;
            existing_listing.platform = args.platform;

            existing_listing.quote.price = args.quote.price;
            existing_listing.quote.volume_24h = args.quote.volume_24h;
            existing_listing.quote.volume_change_24h = args.quote.volume_change_24h;
            existing_listing.quote.percent_change_24h = args.quote.percent_change_24h;
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
                platform: args.platform.map(|data| Platform { id: data.id, name: data.name, symbol: data.symbol, slug: data.slug, token_address: data.token_address }),
                quote: Quote {
                    price: args.quote.price,
                    volume_24h: args.quote.volume_24h,
                    volume_change_24h: args.quote.volume_change_24h,
                    percent_change_24h: args.quote.percent_change_24h,
                    market_cap: args.quote.market_cap,
                    market_cap_dominance: args.quote.market_cap_dominance,
                    fully_diluted_market_cap: args.quote.fully_diluted_market_cap,
                    last_updated: args.quote.last_updated,
                },
                last_updated: args.last_updated,
            };
            storage.listings.insert(symbol, listing);
        }
        Ok(())
    })
}

#[update]
pub fn upsert_listings(args: Vec<Listing>) -> Result<(), String> {
    is_manager_or_admin()?;
    
    // Add log for debugging
    ic_cdk::println!("upsert_listings called with {} listings", args.len());

    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();

        for (index, arg) in args.into_iter().enumerate() {
            let symbol = arg.symbol.to_uppercase();
            
            // Log each listing being processed
            ic_cdk::println!("Processing listing {}: symbol={}", index + 1, symbol);

            if let Some(existing_listing) = storage.listings.get_mut(&symbol) {
                ic_cdk::println!("Updating existing listing: {}", symbol);
                existing_listing.name = arg.name;
                existing_listing.slug = arg.slug;
                existing_listing.cmc_rank = arg.cmc_rank;
                existing_listing.num_market_pairs = arg.num_market_pairs;
                existing_listing.circulating_supply = arg.circulating_supply;
                existing_listing.total_supply = arg.total_supply;
                existing_listing.max_supply = arg.max_supply;
                existing_listing.infinite_supply = arg.infinite_supply;
                existing_listing.date_added = arg.date_added;
                existing_listing.platform = arg.platform;

                existing_listing.quote.price = arg.quote.price;
                existing_listing.quote.volume_24h = arg.quote.volume_24h;
                existing_listing.quote.volume_change_24h = arg.quote.volume_change_24h;
                existing_listing.quote.percent_change_24h = arg.quote.percent_change_24h;
                existing_listing.quote.market_cap = arg.quote.market_cap;
                existing_listing.quote.market_cap_dominance = arg.quote.market_cap_dominance;
                existing_listing.quote.fully_diluted_market_cap = arg.quote.fully_diluted_market_cap;
                existing_listing.quote.last_updated = arg.quote.last_updated;

                existing_listing.last_updated = arg.last_updated;
            } else {
                ic_cdk::println!("Creating new listing: {}", symbol);
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
                    platform: arg.platform.map(|data| Platform { id: data.id, name: data.name, symbol: data.symbol, slug: data.slug, token_address: data.token_address }),
                    quote: Quote {
                        price: arg.quote.price,
                        volume_24h: arg.quote.volume_24h,
                        volume_change_24h: arg.quote.volume_change_24h,
                        percent_change_24h: arg.quote.percent_change_24h,
                        market_cap: arg.quote.market_cap,
                        market_cap_dominance: arg.quote.market_cap_dominance,
                        fully_diluted_market_cap: arg.quote.fully_diluted_market_cap,
                        last_updated: arg.quote.last_updated.clone(),
                    },
                    last_updated: arg.last_updated.clone(),
                };
                storage.listings.insert(symbol.clone(), listing);
                ic_cdk::println!("New listing inserted: {}", symbol);
            }
        }
        
        // Log the total count after operation
        let count = storage.listings.len();
        ic_cdk::println!("Operation completed. Total listings count: {}", count);
        
        Ok(())
    })
}

#[query]
pub fn get_listing(symbol: String) -> Option<Listing> {
    let symbol = symbol.to_uppercase();
    STORAGE.with(|storage| {
        let s = storage.borrow();
        let keys = s.listings.keys().collect::<Vec<_>>();
        let result = s.listings.get(&symbol);
        let cloned_result = result.cloned();
        cloned_result
    })
}

#[query]
pub fn get_listings(limit: Option<u64>, offset: Option<u64>, exclude_stablecoins: Option<bool>) -> Vec<Listing> {
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
    let should_exclude_stablecoins = exclude_stablecoins.unwrap_or(true);
    
    STORAGE.with(|storage| {
        // Get all listings
        let mut listings: Vec<Listing> = storage.borrow().listings.values().cloned().collect();
        
        // Filter out stablecoins if needed
        if should_exclude_stablecoins {
            // Create regex pattern for stablecoins
            // This pattern matches symbols that contain 'usd' in any position or case,
            // as well as other common stablecoin identifiers
            let stablecoin_regex = Regex::new(r"(?i)(.*usd.*|dai|usdt|usdc)").unwrap();
            
            listings = listings.into_iter()
                .filter(|listing| {
                    // Check if the symbol matches the stablecoin pattern
                    !stablecoin_regex.is_match(&listing.symbol)
                })
                .collect();
        }
        
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
        
        result
    })
}

#[query]
pub fn get_listings_count(exclude_stablecoins: Option<bool>) -> u64 {
    // Process parameters
    let should_exclude_stablecoins = exclude_stablecoins.unwrap_or(true);
    
    STORAGE.with(|storage| {
        // Get all listings
        let listings = storage.borrow().listings.values().cloned().collect::<Vec<Listing>>();
        
        // Filter out stablecoins if needed
        if should_exclude_stablecoins {
            // Create regex pattern for stablecoins
            let stablecoin_regex = Regex::new(r"(?i)(.*usd.*|dai|usdt|usdc)").unwrap();
            
            let filtered_listings = listings.into_iter()
                .filter(|listing| {
                    // Check if the symbol matches the stablecoin pattern
                    !stablecoin_regex.is_match(&listing.symbol)
                })
                .collect::<Vec<Listing>>();
            
            return filtered_listings.len() as u64;
        }
        
        // Return total count if not filtering
        listings.len() as u64
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
