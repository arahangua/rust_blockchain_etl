use clap::{Parser, Subcommand}; // import clap for arg handling
use tokio; 
use dotenv::dotenv;
use std::env;
use web3::{self, types::{U64, Transaction}};
use csv::Writer;
use std::path::Path;
use std::fs;

#[derive(Parser)]
#[command(name="rust_etl")]
#[command(author="arahangua")]
#[command(version="0.1")]
#[command(about="practice rust project for etl of on-chain data")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    /// get transactions that belong to the input block number (saved as csv files inside ./outputs folder)
   EthByBnum {bl_num: Option<String>},
}

const OUTPUT_FOLDER:&str = "./outputs/";

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
    let path = Path::new(OUTPUT_FOLDER);
    // make dirs if not existing
    if !path.exists(){
        match fs::create_dir_all(path) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
        }
    }
    let output_fol = format!("{}tx_{:?}.csv", OUTPUT_FOLDER, content.hash);
    let mut wtr = Writer::from_path(output_fol).unwrap();
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





