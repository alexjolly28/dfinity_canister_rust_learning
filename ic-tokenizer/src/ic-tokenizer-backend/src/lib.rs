mod storage;

use bytes::Bytes;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    DefaultMemoryImpl,
};
use rust_tokenizers::tokenizer::{BertTokenizer, Tokenizer, TruncationStrategy};
use rust_tokenizers::vocab::BertVocab;
use std::cell::RefCell;

const WASI_MEMORY_ID: MemoryId = MemoryId::new(0);
const VOCAB_PATH: &str = "bert-base-uncased-vocab.txt";
const TARGET_LEN: usize = 256;

thread_local! {

    static VOCAB: RefCell<Option<BertVocab>>=RefCell::new(None);

    // The memory manager is used for simulating multiple memories.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

}

#[ic_cdk::init]
fn init() {
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(WASI_MEMORY_ID));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(WASI_MEMORY_ID));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);
}

#[ic_cdk::update]
fn append_bytes(bytes: Vec<u8>) {
    storage::append_bytes(VOCAB_PATH, bytes);
}

pub fn setup_vocab(vocab_bytes: Bytes) -> Result<(), ()> {
    // let v = vocab_bytes.to_vec();
    let vocab = BertVocab::from_bytes(&vocab_bytes.to_vec()).unwrap();
    VOCAB.with_borrow_mut(|m| {
        *m = Some(vocab);
    });
    Ok(())
}

#[ic_cdk::update]
fn setup_models() -> Result<(), String> {
    setup_vocab(storage::bytes(VOCAB_PATH))
        .map_err(|err| format!("Failed to setup model: {:?}", err))
}

fn pad_vector<T: Default + Clone>(input_ids: Vec<T>, target_len: usize) -> Vec<T> {
    let mut padded = input_ids.clone();
    padded.resize(target_len, T::default());
    padded
}

fn fmt(n: u64) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join("_")
}

#[ic_cdk::query]
fn tokenize(sentence: String) -> Result<(Vec<i64>, Vec<i32>, Vec<i8>), ()> {
    let instructions_before = ic_cdk::api::instruction_counter();

    let lowercase: bool = true;
    let strip_accents: bool = true;
    VOCAB.with_borrow(|vocab| {
        let vocab = vocab.clone().unwrap();

        let bert_tokenizer = BertTokenizer::from_existing_vocab(vocab, lowercase, strip_accents);

        let tokens = bert_tokenizer.encode(
            &sentence,
            None,
            TARGET_LEN,
            &TruncationStrategy::DoNotTruncate,
            0,
        );

        let input_ids = tokens.token_ids;
        let input_ids: Vec<i64> = pad_vector(input_ids, TARGET_LEN);

        // Generate and print the attention mask
        let attention_mask = input_ids
            .iter()
            .map(|&id| if id == 0 { 0 } else { 1 })
            .collect();

        let attention_mask = pad_vector(attention_mask, TARGET_LEN);

        let segment_ids = tokens.segment_ids;
        let token_type_ids = pad_vector(segment_ids, TARGET_LEN);

        let instructions = ic_cdk::api::instruction_counter() - instructions_before;

        ic_cdk::println!(
            "Tokenization:     {:>12} Wasm instructions",
            fmt(instructions)
        );

        Ok((input_ids, attention_mask, token_type_ids))
    })
}

ic_cdk::export_candid!();
