use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref COMPUTED_HASH: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref LAST_HASH: Mutex<String> = Mutex::new("".to_string());
}

pub fn compute_hash(hash_seed: &str, filename: &str) -> String {
    let mut computed_hash = COMPUTED_HASH.lock().unwrap();
    let mut last_hash = LAST_HASH.lock().unwrap();

    if !computed_hash.contains_key(filename) {
        let hash = md5::compute(format!("{}{}{}", hash_seed, filename, last_hash));
        let hash_string = format!("{:x}", hash)[..8].to_string();
        computed_hash.insert(filename.to_string(), hash_string);
    }

    *last_hash = computed_hash.get(filename).unwrap().clone();


    computed_hash.get(filename).unwrap().to_string()
}
