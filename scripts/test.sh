#!/bin/bash

NETWORK="local"
CANISTER_ID=""

while [[ "$#" -gt 0 ]]; do
    case $1 in
        --canister) CANISTER_ID="$2"; shift ;;
        --network) NETWORK="$2"; shift ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
    shift
done

if [ -z "$CANISTER_ID" ]; then
    echo "Error: Please provide a Canister ID using --canister <CANISTER_ID>"
    exit 1
fi

if [ "$NETWORK" != "local" ] && [ "$NETWORK" != "ic" ]; then
    echo "Error: Invalid network. Use 'local' or 'ic'."
    exit 1
fi

dfx identity use ic-news
IC_NEWS_PRINCIPAL="sw6pw-cfq23-ao3y7-pyhy7-b32ft-6kdzh-enguv-zgnct-cpujp-4xu4z-6ae"

if [ "$NETWORK" == "local" ]; then
    if ! pgrep -f "dfx start" > /dev/null; then
        echo "DFX is not running. Starting DFX in the background..."
        dfx start --background
        sleep 5  # 等待 DFX 启动
    fi
fi

DFX_CALL="dfx canister call $CANISTER_ID"
if [ "$NETWORK" == "ic" ]; then
    DFX_CALL="$DFX_CALL --network ic"
fi

echo "Adding ic-news as manager..."
$DFX_CALL add_manager "(principal \"$IC_NEWS_PRINCIPAL\")"

# echo "Initializing Listings..."

# # 测试用例 1: BTC - 完整数据
# echo "Inserting BTC..."
# $DFX_CALL upsert_listing '(record { 
#     id = 1; 
#     symbol = "BTC"; 
#     name = "Bitcoin"; 
#     slug = "bitcoin"; 
#     cmc_rank = 1; 
#     num_market_pairs = 500; 
#     circulating_supply = 19000000.0; 
#     total_supply = 21000000.0; 
#     max_supply = 21000000.0; 
#     infinite_supply = false; 
#     date_added = "2013-04-28T00:00:00.000Z"; 
#     tags = vec { "cryptocurrency"; "proof-of-work" }; 
#     platform = null; 
#     self_reported_circulating_supply = 0.0; 
#     self_reported_market_cap = 0.0; 
#     quote = record { 
#         price = 60000.0; 
#         volume_24h = 1000000000.0; 
#         volume_change_24h = 5.0; 
#         percent_change_1h = 1.0; 
#         percent_change_24h = 2.0; 
#         percent_change_7d = 10.0; 
#         percent_change_30d = 0.0; 
#         percent_change_60d = 0.0; 
#         percent_change_90d = 0.0; 
#         market_cap = 1140000000000.0; 
#         market_cap_dominance = 40.0; 
#         fully_diluted_market_cap = 1260000000000.0; 
#         last_updated = "2025-03-22T13:12:53.000Z" 
#     }; 
#     last_updated = "2025-03-22T13:12:53.000Z" 
# })'
# echo "Querying BTC..."
# $DFX_CALL get_listing '("BTC")'

# # 测试用例 2: ETH - 完整数据，带平台信息
# echo "Inserting ETH..."
# $DFX_CALL upsert_listing '(record { 
#     id = 2; 
#     symbol = "ETH"; 
#     name = "Ethereum"; 
#     slug = "ethereum"; 
#     cmc_rank = 2; 
#     num_market_pairs = 300; 
#     circulating_supply = 120000000.0; 
#     total_supply = 12000000.0; 
#     max_supply = 0.0; 
#     infinite_supply = true; 
#     date_added = "2015-08-07T00:00:00.000Z"; 
#     tags = vec { "smart-contracts"; "proof-of-stake" }; 
#     platform = opt record { id = 1; name = "Ethereum"; symbol = "ETH"; slug = "ethereum"; token_address = "0x0000000000000000000000000000000000000000" }; 
#     self_reported_circulating_supply = 115000000.0; 
#     self_reported_market_cap = 230000000000.0; 
#     quote = record { 
#         price = 2000.0; 
#         volume_24h = 500000000.0; 
#         volume_change_24h = 3.0; 
#         percent_change_1h = 0.5; 
#         percent_change_24h = 1.5; 
#         percent_change_7d = 8.0; 
#         percent_change_30d = 0.0; 
#         percent_change_60d = 0.0; 
#         percent_change_90d = 0.0; 
#         market_cap = 240000000000.0; 
#         market_cap_dominance = 10.0; 
#         fully_diluted_market_cap = 240000000000.0; 
#         last_updated = "2025-03-22T13:12:53.000Z" 
#     }; 
#     last_updated = "2025-03-22T13:12:53.000Z" 
# })'
# echo "Querying ETH..."
# $DFX_CALL get_listing '("ETH")'

# # 测试用例 3: XRP - 部分字段缺失
# echo "Inserting XRP..."
# $DFX_CALL upsert_listing '(record { 
#     id = 3; 
#     symbol = "XRP"; 
#     name = "Ripple"; 
#     slug = "ripple"; 
#     cmc_rank = 6; 
#     num_market_pairs = 200; 
#     circulating_supply = 50000000000.0; 
#     total_supply = 100000000000.0; 
#     max_supply = 100000000000.0; 
#     infinite_supply = false; 
#     date_added = "2013-08-04T00:00:00.000Z"; 
#     tags = vec { "payment" }; 
#     platform = null; 
#     self_reported_circulating_supply = 1.0; 
#     self_reported_market_cap = 1.0; 
#     quote = record { 
#         price = 0.8; 
#         volume_24h = 300000000.0; 
#         volume_change_24h = 1.0; 
#         percent_change_1h = 1.0; 
#         percent_change_24h = 1.0; 
#         percent_change_7d = 1.0; 
#         percent_change_30d = 1.0; 
#         percent_change_60d = 1.0; 
#         percent_change_90d = 1.0; 
#         market_cap = 40000000000.0; 
#         market_cap_dominance = 2.0; 
#         fully_diluted_market_cap = 80000000000.0; 
#         last_updated = "2025-03-22T13:12:53.000Z" 
#     }; 
#     last_updated = "2025-03-22T13:12:53.000Z" 
# })'
# echo "Querying XRP..."
# $DFX_CALL get_listing '("XRP")'

# # 测试用例 4: ADA - 更新已有记录，仅更新 quote 数据
# echo "Inserting ADA..."
# $DFX_CALL upsert_listing '(record { 
#     id = 4; 
#     symbol = "ADA"; 
#     name = "Cardano"; 
#     slug = "cardano"; 
#     cmc_rank = 7; 
#     num_market_pairs = 150; 
#     circulating_supply = 35000000000.0; 
#     total_supply = 45000000000.0; 
#     max_supply = 45000000000.0; 
#     infinite_supply = false; 
#     date_added = "2017-10-01T00:00:00.000Z"; 
#     tags = vec { "proof-of-stake" }; 
#     platform = null; 
#     self_reported_circulating_supply = 0.0; 
#     self_reported_market_cap = 0.0; 
#     quote = record { 
#         price = 0.5; 
#         volume_24h = 200000000.0; 
#         volume_change_24h = 2.0; 
#         percent_change_1h = 0.3; 
#         percent_change_24h = 0.8; 
#         percent_change_7d = 5.0; 
#         percent_change_30d = 1.0; 
#         percent_change_60d = 1.0; 
#         percent_change_90d = 1.0; 
#         market_cap = 17500000000.0; 
#         market_cap_dominance = 1.0; 
#         fully_diluted_market_cap = 22500000000.0; 
#         last_updated = "2025-03-22T13:12:53.000Z" 
#     }; 
#     last_updated = "2025-03-22T13:12:53.000Z" 
# })'
# echo "Querying ADA..."
# $DFX_CALL get_listing '("ADA")'

# echo "Querying all listings..."
# $DFX_CALL get_listings '(null,null)'

# echo "=== Initialization Completed ==="