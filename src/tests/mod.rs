use crate::models::{Listing, Quote, Platform};
use crate::services::get_listings;
use crate::storage::STORAGE;
use std::collections::HashMap;

// Helper function to create a test listing
fn create_test_listing(id: u32, symbol: &str, rank: u32) -> Listing {
    Listing {
        id,
        symbol: symbol.to_string(),
        name: format!("{} Coin", symbol),
        slug: symbol.to_lowercase(),
        cmc_rank: rank,
        num_market_pairs: 100,
        circulating_supply: 1000000.0,
        total_supply: 2000000.0,
        max_supply: 5000000.0,
        infinite_supply: false,
        date_added: "2023-01-01".to_string(),
        tags: vec!["crypto".to_string()],
        platform: None,
        self_reported_circulating_supply: 0.0,
        self_reported_market_cap: 0.0,
        quote: Quote {
            price: 100.0,
            volume_24h: 1000000.0,
            volume_change_24h: 5.0,
            percent_change_1h: 1.0,
            percent_change_24h: 2.0,
            percent_change_7d: 3.0,
            percent_change_30d: 4.0,
            percent_change_60d: 5.0,
            percent_change_90d: 6.0,
            market_cap: 100000000.0,
            market_cap_dominance: 0.5,
            fully_diluted_market_cap: 200000000.0,
            last_updated: "2023-01-01T00:00:00Z".to_string(),
        },
        last_updated: "2023-01-01T00:00:00Z".to_string(),
    }
}

// Setup test data with both stablecoins and non-stablecoins
fn setup_test_data() {
    STORAGE.with(|storage| {
        let mut s = storage.borrow_mut();
        s.listings = HashMap::new();
        
        // Add non-stablecoins
        s.listings.insert("BTC".to_string(), create_test_listing(1, "BTC", 1));
        s.listings.insert("ETH".to_string(), create_test_listing(2, "ETH", 2));
        s.listings.insert("SOL".to_string(), create_test_listing(3, "SOL", 3));
        s.listings.insert("DOT".to_string(), create_test_listing(4, "DOT", 4));
        s.listings.insert("AVAX".to_string(), create_test_listing(5, "AVAX", 5));
        
        // Add stablecoins
        s.listings.insert("USDT".to_string(), create_test_listing(6, "USDT", 6));
        s.listings.insert("USDC".to_string(), create_test_listing(7, "USDC", 7));
        s.listings.insert("DAI".to_string(), create_test_listing(8, "DAI", 8));
        s.listings.insert("BUSD".to_string(), create_test_listing(9, "BUSD", 9));
        s.listings.insert("USDP".to_string(), create_test_listing(10, "USDP", 10));
        s.listings.insert("XUSD".to_string(), create_test_listing(11, "XUSD", 11));
    });
}

#[test]
fn test_get_listings_with_stablecoin_filter() {
    // Setup test data
    setup_test_data();
    
    // Test with exclude_stablecoins = true (default)
    let listings_excluding_stablecoins = get_listings(None, None, None);
    
    // Should only return non-stablecoins (5 items)
    assert_eq!(listings_excluding_stablecoins.len(), 5);
    
    // Verify that no stablecoins are included
    for listing in &listings_excluding_stablecoins {
        assert!(!listing.symbol.to_lowercase().contains("usd"));
        assert_ne!(listing.symbol, "DAI");
    }
    
    // Test with exclude_stablecoins = false
    let all_listings = get_listings(None, None, Some(false));
    
    // Should return all coins (11 items)
    assert_eq!(all_listings.len(), 11);
    
    // Verify that stablecoins are included
    let stablecoin_count = all_listings
        .iter()
        .filter(|listing| {
            let symbol = listing.symbol.to_lowercase();
            symbol.contains("usd") || symbol == "dai"
        })
        .count();
    
    assert_eq!(stablecoin_count, 6); // 6 stablecoins in our test data
}

#[test]
fn test_get_listings_pagination() {
    // Setup test data
    setup_test_data();
    
    // Test with limit = 3, offset = 0, exclude_stablecoins = true
    let first_page = get_listings(Some(3), Some(0), Some(true));
    
    // Should return first 3 non-stablecoins
    assert_eq!(first_page.len(), 3);
    assert_eq!(first_page[0].symbol, "BTC");
    assert_eq!(first_page[1].symbol, "ETH");
    assert_eq!(first_page[2].symbol, "SOL");
    
    // Test with limit = 3, offset = 3, exclude_stablecoins = true
    let second_page = get_listings(Some(3), Some(3), Some(true));
    
    // Should return next 2 non-stablecoins (only 5 total non-stablecoins)
    assert_eq!(second_page.len(), 2);
    assert_eq!(second_page[0].symbol, "DOT");
    assert_eq!(second_page[1].symbol, "AVAX");
    
    // Test with limit = 3, offset = 0, exclude_stablecoins = false
    let first_page_all = get_listings(Some(3), Some(0), Some(false));
    
    // Should return first 3 coins (including stablecoins)
    assert_eq!(first_page_all.len(), 3);
}

#[test]
fn test_get_listings_empty_result() {
    // Setup test data
    setup_test_data();
    
    // Test with offset beyond available data
    let empty_result = get_listings(Some(10), Some(100), Some(true));
    
    // Should return empty vector
    assert_eq!(empty_result.len(), 0);
}
