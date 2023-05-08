# get the contract address from explorer tx of instantiating
CONTRACT="inj1vjvhnl4szj8twhaqc5ke6x20kzrecs6semyjnr"
readonly INJ_ADDRESS="inj1lsuerzge89tyd4p2pj8wrj903v5ja5emmugntd"

GET_SUPPORTED_TOKENS_QUERY='{"get_supported_tokens": {}}'
injectived query wasm contract-state smart $CONTRACT "$GET_SUPPORTED_TOKENS_QUERY" --node=https://k8s.testnet.tm.injective.network:443 --output json

sleep 2

# USDC price 1
SET_PRICE_USDC='{"set_price":{"denom": "peggy0x87aB3B4C8661e07D6372361211B96ed4Dc36B1B5", "price": "1"}}'
yes 12345678 | injectived tx wasm execute $CONTRACT "$SET_PRICE_USDC" --from=$(echo $INJ_ADDRESS) --amount="0peggy0x87aB3B4C8661e07D6372361211B96ed4Dc36B1B5" --chain-id="injective-888" --yes --gas-prices=500000000inj --gas=20000000 --node=https://k8s.testnet.tm.injective.network:443

sleep 2

# INJ price 10
SET_PRICE_INJ='{"set_price":{"denom": "inj", "price": "10"}}'
yes 12345678 | injectived tx wasm execute $CONTRACT "$SET_PRICE_INJ" --from=$(echo $INJ_ADDRESS) --amount="0peggy0x87aB3B4C8661e07D6372361211B96ed4Dc36B1B5" --chain-id="injective-888" --yes --gas-prices=500000000inj --gas=20000000 --node=https://k8s.testnet.tm.injective.network:443

sleep 2
# APE price 4
SET_PRICE_APE='{"set_price":{"denom": "peggy0x44C21afAaF20c270EBbF5914Cfc3b5022173FEB7", "price": "4"}}'
yes 12345678 | injectived tx wasm execute $CONTRACT "$SET_PRICE_APE" --from=$(echo $INJ_ADDRESS) --amount="0peggy0x87aB3B4C8661e07D6372361211B96ed4Dc36B1B5" --chain-id="injective-888" --yes --gas-prices=500000000inj --gas=20000000 --node=https://k8s.testnet.tm.injective.network:443

sleep 2

GET_PRICE='{"get_price": {"denom": "peggy0x87aB3B4C8661e07D6372361211B96ed4Dc36B1B5"}}'
injectived query wasm contract-state smart $CONTRACT "$GET_PRICE" --node=https://k8s.testnet.tm.injective.network:443 --output json

sleep 2

GET_PRICE='{"get_price": {"denom": "inj"}}'
injectived query wasm contract-state smart $CONTRACT "$GET_PRICE" --node=https://k8s.testnet.tm.injective.network:443 --output json

sleep 2

GET_PRICE='{"get_price": {"denom": "peggy0x44C21afAaF20c270EBbF5914Cfc3b5022173FEB7"}}'
injectived query wasm contract-state smart $CONTRACT "$GET_PRICE" --node=https://k8s.testnet.tm.injective.network:443 --output json

sleep 2