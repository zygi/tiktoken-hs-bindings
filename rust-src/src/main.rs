use std::collections::HashMap;

use _tiktoken::CoreBPE;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

extern crate _tiktoken;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Spec {
    name: String,
    pat_str: String,
    mergeable_ranks: HashMap<String, usize>, // base64 encoded
    special_tokens: FxHashMap<String, usize> // NOT base64 encoded
}

fn main() {
    // load contents of dump_gpt2.json
    let contents = std::fs::read_to_string("dump_gpt2.json")
        .expect("Something went wrong reading the file");

    let spec: Spec = serde_json::from_str(&contents).unwrap();

    // decode all keys in mergeable_ranks
    let mut decoded_mergeable_ranks = spec.mergeable_ranks.into_iter().map(|(key, value)| {
        let decoded_key = base64::decode(key).unwrap();
        (decoded_key, value)
    }).collect::<FxHashMap<Vec<u8>, usize>>();

    let cbpe = CoreBPE::new_native(decoded_mergeable_ranks, spec.special_tokens, &spec.pat_str).unwrap();

    dbg!(cbpe._encode_native("hello world", &Default::default()));

    println!("Hello, world!");
}