use std::{collections::HashMap, ffi::{c_char, c_int, CStr, c_ulong, c_ulonglong, c_longlong}};

use _tiktoken::CoreBPE;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

extern crate _tiktoken;
use lazy_static::lazy_static;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Spec {
    name: String,
    pat_str: String,
    mergeable_ranks: HashMap<String, usize>, // base64 encoded
    special_tokens: FxHashMap<String, usize> // NOT base64 encoded
}

const TIKTOKEN_JSON : &str = include_str!("data/dump_gpt2.json");

fn load_corebpe(spec_json: &str) -> CoreBPE {
    let spec: Spec = serde_json::from_str(spec_json).unwrap();

    // decode all keys in mergeable_ranks
    let decoded_mergeable_ranks = spec.mergeable_ranks.into_iter().map(|(key, value)| {
        let decoded_key = base64::decode(key).unwrap();
        (decoded_key, value)
    }).collect::<FxHashMap<Vec<u8>, usize>>();

    CoreBPE::new_native(decoded_mergeable_ranks, spec.special_tokens, &spec.pat_str).unwrap()
}

lazy_static! {
    static ref CORE_BPE_GPT2: CoreBPE = {
        load_corebpe(TIKTOKEN_JSON)
    };
}


#[no_mangle]
pub extern "C" fn encode_native(input: *const c_char, output: *mut *mut c_ulonglong) -> c_ulonglong  {
    let cstr = unsafe { CStr::from_ptr(input) };

    let (encoded, _) = CORE_BPE_GPT2._encode_native(cstr.to_str().unwrap(), &Default::default());
    let mut encoded_u64 = encoded.iter().map(|x| *x as u64).collect::<Vec<u64>>().into_boxed_slice();
    unsafe { *output = encoded_u64.as_mut_ptr(); }
    std::mem::forget(encoded_u64);
    encoded.len() as c_ulonglong
}