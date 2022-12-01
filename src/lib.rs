extern crate alloc;

use fuel_indexer_macros::indexer;
use fuel_indexer_plugin::types::*;
use fuels_core::*;
use fuel_indexer_plugin::{types::tx::*, types::Bytes32, utils::sha256_digest};
use std::str;

pub fn derive_id(id: [u8; 32], data: Vec<u8>) -> Bytes32 {
    let mut buff: [u8; 32] = [0u8; 32];
    let result = [id.to_vec(), data].concat();
    buff.copy_from_slice(&sha256_digest(&result).as_bytes()[..32]);
    Bytes32::from(buff)
}

pub fn get_zero_address() -> Address {
    Address::new([0u8; 32])
}

pub fn get_zero_contract() -> ContractId {
    ContractId::new([0u8; 32])
}

pub fn get_u64_from_address(address: Address) -> u64 {
    let mut buff = [0u8; 8];
    buff.copy_from_slice(&address.to_string().as_bytes()[..8]);
    u64::from_le_bytes(buff)
}

pub fn get_address_from_identity(identity: Identity) -> Address {
    match identity {
        Identity::Address(address) => address,
        Identity::ContractId(_) => get_zero_address(),
    }
}

pub fn get_i64_from_u64(val: u64) -> i64 {
    val.try_into().unwrap()
}

pub fn get_u64_from_string_vec(values: Vec<String>) -> u64 {
    let result = values.join("_");
    let mut buff = [0u8; 8];
    buff.copy_from_slice(&result.as_bytes()[..8]);
    u64::from_le_bytes(buff)
}

pub fn load_or_get_account(account: Address) -> Account {
    match Account::load(get_u64_from_address(account)) {
        Some(acc) => acc,
        None => {
            let acc = Account {
                id: get_u64_from_address(account),
                address: account,
            };
            acc.save();
            acc
        }
    }
}

#[indexer(manifest = "/Users/tusharverma/Code/fuel-indexers/fuel-nft-block-indexer/manifest.yaml")]
pub mod nft_indexer_module {

    pub fn handle_block_event(block_data: BlockData){
        for tx in block_data.transactions.iter() {
            match &tx.transaction {
                #[allow(unused)]
                Transaction::Script(t) => {
                    Logger::info("Inside a script transaction. (>^‿^)>");

                    let gas_limit = t.gas_limit();
                    let gas_price = t.gas_price();
                    let maturity = t.maturity();
                    let script = t.script();
                    let script_data = t.script_data();
                    let receipts_root = t.receipts_root();
                    let inputs = t.inputs();
                    let outputs = t.outputs();
                    let witnesses = t.witnesses();
                    Logger::info("Script Input");
                    Logger::info(&format!("{:?}",script_data).to_string());
                }
                #[allow(unused)]
                Transaction::Create(t) => {
                    Logger::info("Inside a create transaction. <(^.^)>");

                    let gas_limit = t.gas_limit();
                    let gas_price = t.gas_price();
                    let maturity = t.maturity();
                    let bytecode_length = t.bytecode_length();
                    let bytecode_witness_index = t.bytecode_witness_index();
                    let inputs = t.inputs();
                    let outputs = t.outputs();
                    let witnesses = t.witnesses();
                    let storage_slots = t.storage_slots();
                    Logger::info("Transaction Input");
                    Logger::info(&format!("{:?}",inputs[0]).to_string());
                }
                #[allow(unused)]
                Transaction::Mint(t) => {
                    Logger::info("Inside a mint transaction. <(^‿^<)");

                    let tx_pointer = t.tx_pointer();
                    let outputs = t.outputs();
                    Logger::info("Mint Output");
                    Logger::info(&format!("{:?}",outputs[0]).to_string());
                }
            }
            for receipt in &tx.receipts {

                match receipt {
                    #[allow(unused)]
                    Receipt::Call { id,to, param1,param2,pc,is,.. } => {
                        Logger::info("RECEIPT CALL PARAM1");
                        Logger::info(&format!("{:?}",id).to_string());
                        Logger::info(&format!("{:?}",to).to_string());
                        Logger::info(&format!("{:?}",param1).to_string());
                        Logger::info(&format!("{:?}",param2).to_string());
                        Logger::info(&format!("{:?}",pc).to_string());
                        Logger::info(&format!("{:?}",is).to_string());
                    }
                    #[allow(unused)]
                    Receipt::ReturnData { id,data, .. } => {
                        Logger::info("RECEIPT RETURN DATA");
                        Logger::info(&format!("{:?}",data).to_string());
                    }
                    #[allow(unused)]
                    Receipt::Transfer {
                        id,
                        to,
                        asset_id,
                        amount,
                        ..
                    } => {
                        Logger::info("RECEIPT TRANSFER AMOUNT");
                    }
                    #[allow(unused)]
                    Receipt::LogData {
                        id,data,ra,rb,pc,is,..
                    } => {
                        Logger::info("Inside Log Data");
                        Logger::info(&ra.to_string());
                        Logger::info(&rb.to_string());
                        Logger::info(&pc.to_string());
                        Logger::info(&is.to_string());
                        Logger::info(&format!("{:?}",data).to_string());
                    }
                    _ => {
                        Logger::info("This type is not handled yet.");
                    }
                }
            }
        }
    }
}
