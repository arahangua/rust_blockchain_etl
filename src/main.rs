use clap::{Parser, Subcommand}; // import clap for arg handling
use tokio; 
use dotenv::dotenv;
use std::env;
use web3::{self, types::{U64, Transaction}};
use csv::Writer;

#[derive(Parser)]
#[command(name="rust_etl")]
#[command(author="arahangua <arahangua@gmail.com>")]
#[command(version="0.1")]
#[command(about="practice rust project for etl of on-chain data")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    /// get transactions that belong to the input block number (saved as csv files inside ./outputs folder)
   EthByBnum { bl_num: Option<String>},
}


// The main function, where the program starts executing.

#[tokio::main]
async fn main() {
    dotenv().ok();
    let rpc_https_url = env::var("ETH_MAINNET_EXECUTION_RPC").expect("RPC url must be set");

    let cli = Cli::parse();

    match &cli.command{
        Commands::EthByBnum {bl_num}=>{
            match bl_num{
                // Some(num) => println!("at least it is working {:?}", num),
                Some(num_str) => get_tx_by_block(num_str, rpc_https_url).await,
                None => println!("missing a block number"),
            }
            
        }
    }
}




async fn get_tx_by_block(num_str:&String, rpc_https_url:String) {
    //set connection
    let http = web3::transports::Http::new(&rpc_https_url).unwrap();
    let conn = web3::Web3::new(http);

    //get the block
    let num:U64 = num_str.parse().expect("not a valid number");
    let block_number = web3::types::BlockNumber::Number(num);
    let block = conn.eth().block(web3::types::BlockId::from(block_number)).await.unwrap();
    
    match block {
        Some(block) => {
            //println!("Block details: {:?}", block);
            let transactions = block.transactions;
            for tx in transactions {
                //println!("Transaction details: {:?}", tx);
                let tx_result = conn.eth().transaction(web3::types::TransactionId::Hash(tx)).await.unwrap();
                match tx_result{
                    Some(content)=> save_to_csv(content),
                    None => println!("fetching tx receipt for tx hash {tx:?}"),
                }

                


            }
        },
        None => println!("No block found for {:?}", num_str),
    }
    
}



fn save_to_csv(content:Transaction) {
    let mut wtr = Writer::from_path(format!("./outputs/tx_{:?}.csv", content.hash)).unwrap();
    wtr.write_record(&["hash", "nonce", "blockHash", "blockNumber", "transactionIndex", "from", "to", "value", "gasPrice", "gas", "input"]).unwrap();
    wtr.write_record(&[
        format!("{:?}", content.hash),
        format!("{:?}", content.nonce),
        format!("{:?}", content.block_hash.unwrap()),
        format!("{:?}", content.block_number.unwrap()),
        format!("{:?}", content.transaction_index.unwrap()),
        format!("{:?}", content.from.unwrap()),
        format!("{:?}", content.to.unwrap()),
        format!("{:?}", content.value),
        format!("{:?}", content.gas_price.unwrap()),
        format!("{:?}", content.gas),
        format!("0x{}", hex::encode(content.input.0)),




    ]).unwrap();
    wtr.flush().unwrap();
    println!("transaction {:?} saved to csv", &content.hash);

}





    // // Prepare CSV writer
    // let mut wtr: Writer = Writer::from_path("block.csv").unwrap();

    // // Write block data to CSV
    // wtr.write_record(&["number", "hash", "parent_hash", "nonce", "sha3_uncles", "logs_bloom", "transactions_root", "state_root", "receipts_root", "miner", "difficulty", "total_difficulty", "extra_data", "size", "gas_limit", "gas_used", "timestamp", "uncles"]).unwrap();
    // wtr.write_record(&[
    //     format!("{:?}", block.number.unwrap()),
    //     format!("{:?}", block.hash.unwrap()),
    //     format!("{:?}", block.parent_hash),
    //     format!("{:?}", block.nonce.unwrap()),
    //     format!("{:?}", block.sha3_uncles),
    //     format!("{:?}", block.logs_bloom),
    //     format!("{:?}", block.transactions_root),
    //     format!("{:?}", block.state_root),
    //     format!("{:?}", block.receipts_root),
    //     format!("{:?}", block.miner),
    //     format!("{:?}", block.difficulty),
    //     format!("{:?}", block.total_difficulty),
    //     format!("{:?}", block.extra_data),
    //     format!("{:?}", block.size.unwrap()),
    //     format!("{:?}", block.gas_limit),
    //     format!("{:?}", block.gas_used),
    //     format!("{:?}", block.timestamp),
    //     format!("{:?}", block.uncles),
    // ]).unwrap();

    // wtr.flush().unwrap();
    // println!("Block data saved to block.csv");
// }
