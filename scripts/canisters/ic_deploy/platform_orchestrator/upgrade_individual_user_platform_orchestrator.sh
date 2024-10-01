# !/bin/bash



# Specify the path to your Wasm.gz file
dfx build individual_user_template --network=ic

# Specify the path to your Wasm.gz file
wasm=".dfx/ic/canisters/individual_user_template/individual_user_template.wasm.gz"


# Display the hexdump or use the variable as needed
# echo "$(hexdump -ve '1/1 "%.2x"' "$wasm" | sed 's/../\\&/g')"

# Use xxd to convert the file content to a hexadecimal string
char=$(hexdump -ve '1/1 "%.2x"' "$wasm")

# Escape special characters in the hexadecimal string
char_escaped=$(printf "%s" "$char" | sed 's/../\\&/g')

# Create a shell script with the escaped hexadecimal string
printf "(record {canister = variant {IndividualUserWasm}; version = \"v2.2.0\"; wasm_blob = blob \"%s\"})"  "$char_escaped" > argument
dfx canister call platform_orchestrator upgrade_canister --argument-file argument --network=ic 
