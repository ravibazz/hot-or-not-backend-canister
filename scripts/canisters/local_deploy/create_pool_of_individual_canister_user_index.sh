# !/bin/bash



# Specify the path to your Wasm.gz file
wasm="./target/wasm32-unknown-unknown/release/individual_user_template.wasm.gz"

char=$(hexdump -ve '1/1 "%.2x"' "$wasm")

# Escape special characters in the hexadecimal string
char_escaped=$(printf "%s" "$char" | sed 's/../\\&/g')

# Create a shell script with the escaped hexadecimal string
printf "(\"v1.0.0\", blob \"%s\")"  "$char_escaped" > argument
dfx canister call user_index create_pool_of_individual_user_available_canisters --argument-file argument
dfx canister deposit-cycles 20000000000000 user_index