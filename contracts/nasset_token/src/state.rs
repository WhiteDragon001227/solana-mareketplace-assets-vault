use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::Item;

static KEY_CONFIG_HOLDER_CONTRACT: Item<Addr> = Item::new("config_holder_contract");

pub fn load_config_holder_contract(storage: &dyn Storage) -> StdResult<Addr> {
    KEY_CONFIG_HOLDER_CONTRACT.load(storage)
}

pub fn save_config_holder_contract(storage: &mut dyn Storage, addr: &Addr) -> StdResult<()> {
    KEY_CONFIG_HOLDER_CONTRACT.save(storage, addr)
}
