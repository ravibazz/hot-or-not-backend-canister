# !/bin/bash


cargo test --package user_index
dfx build user_index --network=ic

# Specify the path to your Wasm.gz file
wasm=".dfx/ic/canisters/user_index/user_index.wasm.gz"



# Display the hexdump or use the variable as needed
# echo "$(hexdump -ve '1/1 "%.2x"' "$wasm" | sed 's/../\\&/g')"


# dfx canister install platform_orchestrator --mode=reinstall --argument "(record {
#   user_index_wasm = null;
#   subnet_orchestrator_wasm = null;
#   version= \"v1.0.0\"
# })"

# Use xxd to convert the file content to a hexadecimal string
char=$(hexdump -ve '1/1 "%.2x"' "$wasm")

# Escape special characters in the hexadecimal string
char_escaped=$(printf "%s" "$char" | sed 's/../\\&/g')

# Create a shell script with the escaped hexadecimal string
printf "(variant {SubnetOrchestratorWasm}, blob \"%s\")"  "$char_escaped" > argument
dfx canister call platform_orchestrator upload_wasms --argument-file argument --network=ic 
