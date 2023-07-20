# Rust-blockchain-ETL
practice project for Blockchain ETL using Rust.


# Changelog

- 18.07.23
Function implemented: export Ethereum (mainnet) transactions using a block number (using input from the user) and writes them under 'outputs' folder.

# Installation 

1. Clone the repo
```
git clone https://github.com/arahangua/rust_blockchain_etl.git
```


2. Create .env file in the root directory and set "ETH_MAINNET_EXECUTION_RPC" path. (e.g. use Alchemy, Infura, etc.)
```
// inside your .env file
ETH_MAINNET_EXECUTION_RPC = "Your address of RPC provider"
```

3. Build the project
```
cargo build --release
```

4. Run ETL job
```
./target/release/rust_blockchain_etl --help
``` 
The above command will print the list of implemented commands. 

e.g. export transactions from the specified Ethereum mainnet block number to csv files.
```
./target/release/rust_blockchain_etl eth-by-bnum <block number>
```
