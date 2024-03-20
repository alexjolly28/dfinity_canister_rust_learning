#[allow(unused_imports)]
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    writer::Writer,
    BTreeMap, DefaultMemoryImpl, Memory as _, StableBTreeMap,
};
use serde::{Deserialize, Serialize};

type Memory = VirtualMemory<DefaultMemoryImpl>;

// The state of the canister.
#[derive(Serialize, Deserialize)]
struct State {
    // Data that lives on the heap.
    // This is an example for data that would need to be serialized/deserialized
    // on every upgrade for it to be persisted.
    data_on_the_heap: Vec<u8>,

    // An example `StableBTreeMap`. Data stored in `StableBTreeMap` doesn't need to
    // be serialized/deserialized in upgrades, so we tell serde to skip it.
    #[serde(skip, default = "init_stable_data")]
    stable_data: StableBTreeMap<u128, u128, Memory>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            data_on_the_heap: vec![],
            stable_data: init_stable_data(),
        }
    }
}

fn init_stable_data() -> StableBTreeMap<u128, u128, Memory> {
    StableBTreeMap::init(crate::memory::get_stable_btree_memory())
}

#[test]
fn test() {
    let mut state = State::default();
    state.data_on_the_heap = vec![1, 2, 3, 4, 5];
    let a = state.data_on_the_heap.clone();

    println!("{:?}", a);

    // pre upgrade

    let mut state_bytes: Vec<u8> = vec![];

    ciborium::ser::into_writer(&state, &mut state_bytes).expect("failed to encode state");

    let len = state_bytes.len() as u32;

    println!("{:?}", state_bytes);

    let mut memory = crate::memory::get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap();

    // post upgrade

    let memory = crate::memory::get_upgrades_memory();

    // Read the length of the state bytes.
    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);

    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    // println!("{:?}", state_len);

    // Read the bytes
    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    println!("{:?}", state_bytes);

    // Deserialize and set the state.
    let state: State = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");

    println!("{:?}", state.data_on_the_heap);

    // let mem_mgr = MemoryManager::init(DefaultMemoryImpl::default());
    // let mut map_1: BTreeMap<u64, u64, _> = BTreeMap::init(mem_mgr.get(MemoryId::new(0)));
    // let mut map_2: BTreeMap<u64, u64, _> = BTreeMap::init(mem_mgr.get(MemoryId::new(1)));

    // map_1.insert(1, 2);
    // map_2.insert(1, 3);
    // // assert_eq!(map_1.get(&1), Some(2)); // Succeeds, as expected.

    // println!("{:?}", map_1.get(&1));
}
