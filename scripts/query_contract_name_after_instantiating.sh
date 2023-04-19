# get the contract address from the response
#CODE_ID="713"
#injectived query wasm list-contract-by-code $CODE_ID --node=https://k8s.testnet.tm.injective.network:443 --output json

CONTRACT="inj1u6jq52m9vfkedzka40fvnchddmcsfvu655lhp2"
readonly INJ_ADDRESS="inj1lsuerzge89tyd4p2pj8wrj903v5ja5emmugntd"

## query arbitrary balance
BALANCE_QUERY='{"get_deposit": {"address": "'$(echo $INJ_ADDRESS)'", "denom": "inj1jyldpwc5ycuj5nn7tg7wejfs62pvxsy0l9n9xs"}}'
injectived query wasm contract-state smart $CONTRACT "$BALANCE_QUERY" --node=https://k8s.testnet.tm.injective.network:443 --output json



