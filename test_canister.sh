#!/bin/bash

# Set canister ID and network
CANISTER_ID="ddlxl-gyaaa-aaaag-at7gq-cai"
NETWORK="ic"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Testing IC News Market Canister on IC Network${NC}"
echo -e "${YELLOW}Canister ID: ${CANISTER_ID}${NC}"
echo "----------------------------------------"

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
run_test "Get specific listing (BTC)" "dfx canister --network $NETWORK call $CANISTER_ID get_listing '(\"BTC\")'"

# Test 10: Get a specific stablecoin listing (USDT)
run_test "Get specific stablecoin listing (USDT)" "dfx canister --network $NETWORK call $CANISTER_ID get_listing '(\"USDT\")'"

# Test 11: Count total listings vs non-stablecoin listings
echo -e "\n${YELLOW}Comparing total listings vs non-stablecoin listings${NC}"

all_listings=$(dfx canister --network $NETWORK call $CANISTER_ID get_listings '(null, null, opt false)')
non_stablecoin_listings=$(dfx canister --network $NETWORK call $CANISTER_ID get_listings '(null, null, opt true)')

# Extract the count of listings from the result (this is a simplistic approach)
all_count=$(echo "$all_listings" | grep -o "id = " | wc -l)
non_stablecoin_count=$(echo "$non_stablecoin_listings" | grep -o "id = " | wc -l)

echo "Total listings: $all_count"
echo "Non-stablecoin listings: $non_stablecoin_count"
echo "Stablecoin listings: $((all_count - non_stablecoin_count))"

if [ $all_count -gt $non_stablecoin_count ]; then
  echo -e "${GREEN}Stablecoin filtering is working correctly!${NC}"
else
  echo -e "${RED}Stablecoin filtering might not be working as expected.${NC}"
fi

echo -e "\n${YELLOW}All tests completed!${NC}"
