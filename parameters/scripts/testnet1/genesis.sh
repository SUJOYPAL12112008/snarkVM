# This script will run the transaction and block generation programs in the `examples` folder and move the resulting `.genesis` files
# to their respective folders under the `src` directory.
# If the transaction size or checksum has changed, you will need to manually update these in each corresponding struct.

# Generate transactions

# Inputs: network, recipient address, amount, genesis filepath, transaction filepath

RUST_BACKTRACE=1 cargo run --example genesis testnet1 aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah previous_hash.genesis block_header.genesis transaction_1.genesis || exit

mv previous_hash.genesis ../../src/testnet1/genesis || exit

mv transaction_1.genesis ../../src/testnet1/genesis || exit

mv block_header.genesis ../../src/testnet1/genesis || exit
