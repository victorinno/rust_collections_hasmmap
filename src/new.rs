use std::collections::HashMap;

pub fn starting_new_hash_map() {
    let mut new_hash_map:HashMap<String, u32> = HashMap::new();
    new_hash_map.insert(String::from("a"), 324);
    new_hash_map.insert(String::from("b"), 66);

    println!("{:?}", new_hash_map);
}