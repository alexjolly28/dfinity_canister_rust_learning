use std::time::Duration;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::management_canister::provisional::CanisterIdRecord;
use ic_stable_structures::StableBTreeMap;
// use ic_stable_structures::{memory_manager::MemoryManager, DefaultMemoryImpl};
// use serde::Serialize;
// use utils::update_storage;

use candid::Nat;

use crate::{
    memory::{get_asset_stable_memory, get_chunk_stable_memory, Memory},
    Asset, Chunk,
};
use crate::{utils, ChunkArgs};

const EXPIRY_LIMIT: u64 = 10 * 60 * 60 * 1000_000;

#[derive(serde::Serialize, Deserialize)]
pub struct State {
    pub chunk_count: u128,
    pub asset_count: u128,
    pub used_storage: Nat,
    #[serde(skip, default = "get_chunk_stable_memory")]
    pub chunk_list: StableBTreeMap<u128, Chunk, Memory>,
    #[serde(skip, default = "get_asset_stable_memory")]
    pub asset_list: StableBTreeMap<u128, Asset, Memory>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            chunk_count: 0,
            asset_count: 0,
            used_storage: Nat::from(0),
            chunk_list: get_chunk_stable_memory(),
            asset_list: get_asset_stable_memory(),
        }
    }
}

impl State {
    pub fn get_chunk_id(&mut self) -> u128 {
        let id = self.chunk_count;
        self.chunk_count += 1;
        id
    }

    pub fn get_asset_id(&mut self) -> u128 {
        let id = self.asset_count;
        self.asset_count += 1;
        id
    }
}

pub async fn update_storage() {
    let id_record = CanisterIdRecord {
        canister_id: ic_cdk::id(),
    };
    // let status = ic_cdk::api::management_canister::main::canister_status(id_record)
    //     .await
    //     .unwrap()
    //     .0;

    // println!("{:?}", status.memory_size);
    // STATE.with(|s| {
    //     let mut s = s.borrow_mut();
    //     s.used_storage = status.memory_size;
    // })
}

#[test]
fn main() {
    // let mem_mgr: MemoryManager<DefaultMemoryImpl> =
    //     MemoryManager::init(DefaultMemoryImpl::default());

    // let mut state = State::default();
    // let chunk_id = state.get_chunk_id();
    // println!("{}", chunk_id);
    // let args = ChunkArgs {
    //     content: b"hi".to_vec(),
    //     order: 1,
    // };

    // let chunk = Chunk::from((chunk_id.clone(), args, Principal::anonymous()));
    // state.chunk_list.insert(chunk_id.clone(), chunk);
    // ic_cdk_timers::set_timer(Duration::from_secs(0), || ic_cdk::spawn(update_storage()));

    println!("{:?}", EXPIRY_LIMIT);
}
