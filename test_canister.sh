#!/bin/bash

# Set canister ID and network
# Default to local network, but can be overridden with environment variables
CANISTER_ID=${CANISTER_ID:-"$(dfx canister id ic-news-market)"}
NETWORK=${NETWORK:-"local"}

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Testing IC News Market Canister on $NETWORK Network${NC}"
echo -e "${YELLOW}Canister ID: ${CANISTER_ID}${NC}"
echo "----------------------------------------"

# Check if canister ID is valid
if [ -z "$CANISTER_ID" ] || [ "$CANISTER_ID" == "" ]; then
  echo -e "${RED}Error: Invalid canister ID. Make sure the canister is deployed.${NC}"
  echo "Run 'dfx deploy' to deploy the canister first."
  exit 1
fi

# Function to run a test and check the result
run_test() {
  local test_name=$1
  local command=$2
  
  echo -e "\n${YELLOW}Running test: ${test_name}${NC}"
  echo "Command: $command"
  
  # Run the command
  result=$(eval $command)
  
  # Check if the command was successful
  if [ $? -eq 0 ]; then
    echo -e "${GREEN}Test passed!${NC}"
    echo "Result: $result"
  else
    echo -e "${RED}Test failed!${NC}"
    echo "Error: $result"
  fi
}

# Test 1: Get listings with default parameters (should exclude stablecoins)
run_test "Get listings with default parameters" "dfx canister --network $NETWORK call $CANISTER_ID get_listings '(null, null, null)'"

# Test 2: Get listings with explicit exclude_stablecoins=true
run_test "Get listings with exclude_stablecoins=true" "dfx canister --network $NETWORK call $CANISTER_ID get_listings '(null, null, opt true)'"

# Test 3: Get listings with exclude_stablecoins=false (should include stablecoins)
run_test "Get listings with exclude_stablecoins=false" "dfx canister --network $NETWORK call $CANISTER_ID get_listings '(null, null, opt false)'"

# Test 4: Get listings with limit=5
run_test "Get listings with limit=5" "dfx canister --network $NETWORK call $CANISTER_ID get_listings '(opt 5, null, null)'"

# Test 5: Get listings with limit=5 and offset=0
run_test "Get listings with limit=5 and offset=0" "dfx canister --network $NETWORK call $CANISTER_ID get_listings '(opt 5, opt 0, null)'"

# Test 6: Get listings with limit=5 and offset=0 (exclude_stablecoins=false)
run_test "Get listings with limit=5 and offset=0 (exclude_stablecoins=false)" "dfx canister --network $NETWORK call $CANISTER_ID get_listings '(opt 5, opt 0, opt false)'"

# Test 7: Get listings with limit=5 and offset=0 (exclude_stablecoins=true)
run_test "Get listings with limit=5 and offset=0 (exclude_stablecoins=true)" "dfx canister --network $NETWORK call $CANISTER_ID get_listings '(opt 5, opt 0, opt true)'"

# Test 8: Get listings with offset=5
run_test "Get listings with offset=5" "dfx canister --network $NETWORK call $CANISTER_ID get_listings '(null, opt 5, null)'"

# Test 9: Get a specific listing (BTC)
# Add BTC listing first
echo -e "\n${YELLOW}Adding BTC listing for testing${NC}"
dfx canister --network $NETWORK call $CANISTER_ID upsert_listing '(record { id = 1 : nat32; symbol = "BTC"; name = "Bitcoin"; slug = "bitcoin"; cmc_rank = 1 : nat32; num_market_pairs = 500 : nat32; circulating_supply = 19000000.0 : float64; total_supply = 21000000.0 : float64; max_supply = 21000000.0 : float64; infinite_supply = false; date_added = "2013-04-28T00:00:00.000Z"; platform = null; quote = record { price = 50000.0 : float64; volume_24h = 30000000000.0 : float64; volume_change_24h = 2.5 : float64; percent_change_24h = 1.5 : float64; market_cap = 950000000000.0 : float64; market_cap_dominance = 45.0 : float64; fully_diluted_market_cap = 1050000000000.0 : float64; last_updated = "2025-04-08T00:00:00.000Z" }; last_updated = "2025-04-08T00:00:00.000Z" })'

run_test "Get specific listing (BTC)" "dfx canister --network $NETWORK call $CANISTER_ID get_listing '(\"BTC\")'"

# Test 10: Get a specific stablecoin listing (USDT)
# Add USDT listing first
echo -e "\n${YELLOW}Adding USDT listing for testing${NC}"
dfx canister --network $NETWORK call $CANISTER_ID upsert_listing '(record { id = 2 : nat32; symbol = "USDT"; name = "Tether"; slug = "tether"; cmc_rank = 3 : nat32; num_market_pairs = 400 : nat32; circulating_supply = 80000000000.0 : float64; total_supply = 80000000000.0 : float64; max_supply = 80000000000.0 : float64; infinite_supply = false; date_added = "2015-02-25T00:00:00.000Z"; platform = null; quote = record { price = 1.0 : float64; volume_24h = 50000000000.0 : float64; volume_change_24h = 1.0 : float64; percent_change_24h = 0.01 : float64; market_cap = 80000000000.0 : float64; market_cap_dominance = 5.0 : float64; fully_diluted_market_cap = 80000000000.0 : float64; last_updated = "2025-04-08T00:00:00.000Z" }; last_updated = "2025-04-08T00:00:00.000Z" })'

run_test "Get specific stablecoin listing (USDT)" "dfx canister --network $NETWORK call $CANISTER_ID get_listing '(\"USDT\")'"

# Test 11: Get listings count (all listings including stablecoins)
run_test "Get listings count (all listings)" "dfx canister --network $NETWORK call $CANISTER_ID get_listings_count '(opt false)'"

# Test 12: Get listings count (excluding stablecoins)
run_test "Get listings count (excluding stablecoins)" "dfx canister --network $NETWORK call $CANISTER_ID get_listings_count '(null)'"

# Test 13: Compare get_listings_count with actual listings count
echo -e "\n${YELLOW}Comparing get_listings_count with actual listings count${NC}"

all_listings=$(dfx canister --network $NETWORK call $CANISTER_ID get_listings '(null, null, opt false)')
non_stablecoin_listings=$(dfx canister --network $NETWORK call $CANISTER_ID get_listings '(null, null, opt true)')
all_count_api=$(dfx canister --network $NETWORK call $CANISTER_ID get_listings_count '(opt false)')
non_stablecoin_count_api=$(dfx canister --network $NETWORK call $CANISTER_ID get_listings_count '(null)')

# Extract the count of listings from the result (this is a simplistic approach)
all_count=$(echo "$all_listings" | grep -o "id = " | wc -l)
non_stablecoin_count=$(echo "$non_stablecoin_listings" | grep -o "id = " | wc -l)

# Clean up the API results (remove parentheses, whitespace, and type annotations)
all_count_api=$(echo "$all_count_api" | sed 's/([[:space:]]*\([0-9]*\)[[:space:]]*:[[:space:]]*nat64[[:space:]]*)/\1/')
non_stablecoin_count_api=$(echo "$non_stablecoin_count_api" | sed 's/([[:space:]]*\([0-9]*\)[[:space:]]*:[[:space:]]*nat64[[:space:]]*)/\1/')

echo "Total listings (from get_listings): $all_count"
echo "Total listings (from get_listings_count): $all_count_api"
echo "Non-stablecoin listings (from get_listings): $non_stablecoin_count"
echo "Non-stablecoin listings (from get_listings_count): $non_stablecoin_count_api"
echo "Stablecoin listings: $((all_count - non_stablecoin_count))"

# Check if counts match
if [ "$all_count" -eq "$all_count_api" ] && [ "$non_stablecoin_count" -eq "$non_stablecoin_count_api" ]; then
  echo -e "${GREEN}Listing count APIs are working correctly!${NC}"
else
  echo -e "${RED}Listing count APIs might not be working as expected.${NC}"
fi

# Test 14: Test manager functions (only if we're in local environment)
if [ "$NETWORK" == "local" ]; then
  echo -e "\n${YELLOW}Testing manager functions${NC}"
  
  # Get identity principal
  PRINCIPAL=$(dfx identity get-principal)
  echo "Current principal: $PRINCIPAL"
  
  # Test list_managers
  run_test "List managers" "dfx canister --network $NETWORK call $CANISTER_ID list_managers '()'"
  
  # Test add_manager
  run_test "Add manager" "dfx canister --network $NETWORK call $CANISTER_ID add_manager '(principal \"$PRINCIPAL\")'"
  
  # Test list_managers again to verify addition
  run_test "List managers after addition" "dfx canister --network $NETWORK call $CANISTER_ID list_managers '()'"
  
  # Test remove_manager
  run_test "Remove manager" "dfx canister --network $NETWORK call $CANISTER_ID remove_manager '(principal \"$PRINCIPAL\")'"
  
  # Test list_managers again to verify removal
  run_test "List managers after removal" "dfx canister --network $NETWORK call $CANISTER_ID list_managers '()'"
fi

# Test 15: Test upsert_listing and delete_listing (only in local environment)
if [ "$NETWORK" == "local" ]; then
  echo -e "\n${YELLOW}Testing upsert_listing and delete_listing${NC}"
  
  # First add current identity as manager to have permission
  PRINCIPAL=$(dfx identity get-principal)
  dfx canister --network $NETWORK call $CANISTER_ID add_manager "(principal \"$PRINCIPAL\")"
  
  # Create a test listing using proper Candid format
  TEST_LISTING='record { id = 999 : nat32; symbol = "TEST"; name = "Test Coin"; slug = "test-coin"; cmc_rank = 999 : nat32; num_market_pairs = 10 : nat32; circulating_supply = 1000000.0 : float64; total_supply = 2000000.0 : float64; max_supply = 2000000.0 : float64; infinite_supply = false; date_added = "2025-04-08T00:00:00.000Z"; platform = null; quote = record { price = 1.0 : float64; volume_24h = 1000000.0 : float64; volume_change_24h = 5.0 : float64; percent_change_24h = 2.5 : float64; market_cap = 1000000.0 : float64; market_cap_dominance = 0.01 : float64; fully_diluted_market_cap = 2000000.0 : float64; last_updated = "2025-04-08T00:00:00.000Z" }; last_updated = "2025-04-08T00:00:00.000Z" }'
  
  # Test upsert_listing
  run_test "Upsert listing" "dfx canister --network $NETWORK call $CANISTER_ID upsert_listing '($TEST_LISTING)'"
  
  # Test get_listing to verify
  run_test "Get listing after upsert" "dfx canister --network $NETWORK call $CANISTER_ID get_listing '(\"TEST\")'"
  
  # Test delete_listing
  run_test "Delete listing" "dfx canister --network $NETWORK call $CANISTER_ID delete_listing '(\"TEST\")'"
  
  # Test get_listing to verify deletion
  run_test "Get listing after deletion" "dfx canister --network $NETWORK call $CANISTER_ID get_listing '(\"TEST\")'"
  
  # Test upsert_listings (batch)
  echo -e "\n${YELLOW}Testing upsert_listings (batch)${NC}"
  
  # Create a simplified test for batch upload with proper escaping
  BATCH_CMD="dfx canister --network $NETWORK call $CANISTER_ID upsert_listings '(vec { record { id = 999 : nat32; symbol = \"TEST1\"; name = \"Test Coin 1\"; slug = \"test-coin-1\"; cmc_rank = 999 : nat32; num_market_pairs = 10 : nat32; circulating_supply = 1000000.0 : float64; total_supply = 2000000.0 : float64; max_supply = 2000000.0 : float64; infinite_supply = false; date_added = \"2025-04-08T00:00:00.000Z\"; platform = null; quote = record { price = 1.0 : float64; volume_24h = 1000000.0 : float64; volume_change_24h = 5.0 : float64; percent_change_24h = 2.5 : float64; market_cap = 1000000.0 : float64; market_cap_dominance = 0.01 : float64; fully_diluted_market_cap = 2000000.0 : float64; last_updated = \"2025-04-08T00:00:00.000Z\" }; last_updated = \"2025-04-08T00:00:00.000Z\" }; record { id = 998 : nat32; symbol = \"TEST2\"; name = \"Test Coin 2\"; slug = \"test-coin-2\"; cmc_rank = 998 : nat32; num_market_pairs = 10 : nat32; circulating_supply = 1000000.0 : float64; total_supply = 2000000.0 : float64; max_supply = 2000000.0 : float64; infinite_supply = false; date_added = \"2025-04-08T00:00:00.000Z\"; platform = null; quote = record { price = 1.0 : float64; volume_24h = 1000000.0 : float64; volume_change_24h = 5.0 : float64; percent_change_24h = 2.5 : float64; market_cap = 1000000.0 : float64; market_cap_dominance = 0.01 : float64; fully_diluted_market_cap = 2000000.0 : float64; last_updated = \"2025-04-08T00:00:00.000Z\" }; last_updated = \"2025-04-08T00:00:00.000Z\" } })'"
  run_test "Upsert multiple listings" "$BATCH_CMD"
  
  # Clean up by removing the test listings
  dfx canister --network $NETWORK call $CANISTER_ID delete_listing '("TEST")'
  dfx canister --network $NETWORK call $CANISTER_ID delete_listing '("TEST1")'
  dfx canister --network $NETWORK call $CANISTER_ID delete_listing '("TEST2")'
  dfx canister --network $NETWORK call $CANISTER_ID delete_listing '("BTC")'
  dfx canister --network $NETWORK call $CANISTER_ID delete_listing '("USDT")'
  
  # Remove manager status
  dfx canister --network $NETWORK call $CANISTER_ID remove_manager "(principal \"$PRINCIPAL\")"
fi

echo -e "\n${YELLOW}All tests completed!${NC}"
