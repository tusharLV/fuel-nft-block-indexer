extern crate alloc;

use fuel_indexer_macros::indexer;
use fuel_indexer_plugin::types::*;
use fuels_core::*;

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

    pub fn handle_mint_event(event: MintEvent) {
        let MintEvent { owner, token_id } = event;

        let zero_account = crate::load_or_get_account(crate::get_zero_address());
        let owner_address = crate::get_address_from_identity(owner);
        let owner_account = crate::load_or_get_account(owner_address);

        let collection = match Collection::load(1) {
            Some(mut c) => {
                c.total_supply += 1;
                c.total_minted += 1;
                c
            },
            None => {
                Collection {
                    id: 1,
                    contract: crate::get_zero_contract(),
                    admin: zero_account.id,
                    total_supply: 1,
                    total_minted: 1,
                }
            }
        };

        let nft_token = match NFTToken::load(token_id) {
            Some(token) => token,
            None => {
                NFTToken {
                    id: token_id,
                    token_id: crate::get_i64_from_u64(token_id),
                    previous_owner: crate::get_zero_address(),
                    current_owner: owner_address,
                }
            }
        };

        let mint_event = match Mint::load(token_id) {
            Some(mint) => mint,
            None => {
                Mint {
                    id: token_id,
                    owner: owner_account.id,
                    token: nft_token.id,
                }
            }
        };

        collection.save();
        nft_token.save();
        mint_event.save();
    }

    pub fn handle_transfer_event(event: TransferEvent) {
        let TransferEvent { from, to, token_id } = event;

        let from_address = crate::get_address_from_identity(from);
        let to_address = crate::get_address_from_identity(to);
        crate::load_or_get_account(from_address);
        crate::load_or_get_account(to_address);

        let nft_token = match NFTToken::load(token_id) {
            Some(mut token) => {
                token.previous_owner = from_address;
                token.current_owner = to_address;
                token
            },
            None => {
                NFTToken {
                    id: token_id,
                    token_id: crate::get_i64_from_u64(token_id),
                    previous_owner: from_address,
                    current_owner: to_address,
                }
            }
        };

        let transfer_id = crate::get_u64_from_string_vec(
            vec![from_address.to_string(), to_address.to_string(), token_id.to_string()]
        );
        let transfer = match Transfer::load(transfer_id) {
            Some(t) => t,
            None => {
                Transfer {
                    id: transfer_id,
                    token: nft_token.id,
                    from_user: from_address,
                    to_user: to_address,
                }
            }
        };

        nft_token.save();
        transfer.save();
    }

    pub fn handle_block_event(block_data: BlockData){
        let block = Block {
            id: block_data.id,
            height: block_data.height,
            timestamp: block_data.time,
        };

        block.save();
    }
}
