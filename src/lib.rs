mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const ANGPAO_TRACKED_CONTRACT: [u8; 20] = hex!("25a25506B36626d328B1ebE0D16aEF2d3713CE91");

fn map_angpao_events(blk: &eth::Block, events: &mut contract::Events) {
    events.angpao_angpow_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ANGPAO_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::angpao_contract::events::AngpowCreated::match_and_decode(log) {
                        return Some(contract::AngpaoAngpowCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            donator: event.donator,
                            id: event.id.to_string(),
                            quantity: event.quantity.to_string(),
                            token: event.token,
                            token_amount: event.token_amount.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.angpao_angpow_receiveds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ANGPAO_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::angpao_contract::events::AngpowReceived::match_and_decode(log) {
                        return Some(contract::AngpaoAngpowReceived {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            id: event.id.to_string(),
                            index: event.index.to_string(),
                            recipient: event.recipient,
                            token: event.token,
                            token_amount: event.token_amount.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.angpao_initializeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ANGPAO_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::angpao_contract::events::Initialized::match_and_decode(log) {
                        return Some(contract::AngpaoInitialized {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.angpao_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ANGPAO_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::angpao_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::AngpaoOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.angpao_role_admin_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ANGPAO_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::angpao_contract::events::RoleAdminChanged::match_and_decode(log) {
                        return Some(contract::AngpaoRoleAdminChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin_role: Vec::from(event.new_admin_role),
                            previous_admin_role: Vec::from(event.previous_admin_role),
                            role: Vec::from(event.role),
                        });
                    }

                    None
                })
        })
        .collect());
    events.angpao_role_granteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ANGPAO_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::angpao_contract::events::RoleGranted::match_and_decode(log) {
                        return Some(contract::AngpaoRoleGranted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                            role: Vec::from(event.role),
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.angpao_role_revokeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ANGPAO_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::angpao_contract::events::RoleRevoked::match_and_decode(log) {
                        return Some(contract::AngpaoRoleRevoked {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                            role: Vec::from(event.role),
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_angpao_events(&blk, &mut events);
    substreams::skip_empty_output();
    Ok(events)
}

