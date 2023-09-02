use anyhow::{Ok, Result};
use bytes::Bytes;
use ethers_contract::BaseContract;
use ethers_core::abi::{parse_abi, AbiEncode};
use ethers_providers::{Http, Provider, Middleware};
use revm::{
    db::{CacheDB, EmptyDB, EthersDB},
    inspectors::{NoOpInspector},
    primitives::{ExecutionResult, Output, TransactTo, B160, U256 as rU256, B256, TxEnv},
    Database, EVM,
};
use std::{str::FromStr, sync::Arc};
use tokio;


#[tokio::main]
async fn main() -> Result<()> {
    // create ethers client and wrap it in Arc<M>
    let client = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    )?;
    
    let client = Arc::new(client);


    let mut number = 18044086;

    let block = client.get_block_with_txs(number).await?;
    // initialize new EthersDB
    let mut ethersdb = EthersDB::new(Arc::clone(&client), None).unwrap();
    // let mut cache_db = CacheDB::new(ethersdb);
    let mut cache_db = CacheDB::new(EmptyDB::default());

    let mut evm = EVM::new();
    evm.database(cache_db);
    if let Some(block) = block {
        let txns = block.transactions;
        txns.iter().for_each(|tx| {
            evm.env.tx.caller = tx.from.into();

            let txn = tx.hash;
            if let Some(to) = tx.to {
                evm.env.tx.transact_to = TransactTo::Call(to.into());
            }

            // println!("Tx: {:#?}", txn);
            let inspector = NoOpInspector();
            // let hexdata = 
            // evm.env.tx.data = tx.input;
            // evm.env.tx.value = rU256::from(tx.value);
            let yeild = evm.inspect(inspector).unwrap();
            println!("Yeild: {:#?}", yeild);
            println!("txnhash: {:#?}", txn);
        });
    }
    // let txns: Vec<TX> = block.transactions;
    // block.iter().for_each(|tx| {

    //     println!("Tx: {:#?}",);
    //     // let txn = tx.hash;

    //     // client.get_transaction(txn).await.unwrap();

    //     // // TODO(@ckartik): Turn into accessListInspector
        let inspector = NoOpInspector();
    //     // evm.env.tx.caller = B160::from_str("0x0000000000000000000000000000000000000000")?;
    //     // evm.env.tx.transact_to = TransactTo::Call(pool_address);
    //     // evm.env.tx.data = tx.input;
    //     // evm.env.tx.value = rU256::ZERO;
        
    //     // // execute transaction without writing to the DB
    //     // let ref_tx = evm.inspect(&mut inspector).unwrap();
    //     // let result = ref_tx.result;
    // });


            
    //     // unpack output call enum into raw bytes
    //     let value = match result {
    //         ExecutionResult::Success {
    //             output: Output::Call(value),
    //             ..
    //         } => value,
    //         result => panic!("Execution failed: {result:?}"),
    //     };

    //     // do something with the access list in the inspector
    // }


    Ok(())
}