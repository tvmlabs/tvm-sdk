//#![no_std]
#[allow(warnings)]
mod bindings;

// The comments that follow the `use` declaration below
// correlate the rust module path segments with their
// `world.wit` counterparts:
//            <- items bundled with `export` keyword
//                     <- package namespace
//                           <- package
//                                  <- interface name
use std::collections::HashMap;

use bindings::exports::docs::adder::add_interface::Guest;
struct Component;

impl Guest for Component {
    fn add(kwargs: Vec<u8>) -> Vec<u8> {
        let mut fake_stdout: u64;
        let mut seen_exts: HashMap<String, bool> = HashMap::new();
        let z = 3;
        seen_exts.insert(z.to_string(), false);
        seen_exts.insert(String::from("basic_constraints_is_valid"), true);
        let key = (kwargs[0] + kwargs[1]).to_string();

        //---
        // Basic initialization and insertion
        let mut basic_map = HashMap::new();
        basic_map.insert("a", 1);
        basic_map.insert("b", 2);

        // Using entry API
        let mut entry_map = HashMap::new();
        entry_map.entry("x").or_insert(42);

        // Iterating over entries
        for (key, value) in entry_map.iter() {
            fake_stdout = *value;
        }

        // Reserving space
        let mut capacity_map: HashMap<u64, u64> = HashMap::with_capacity(10);

        // Checking for presence
        if basic_map.contains_key("a") {
            fake_stdout = 0;
        }

        // Accessing values (Option handling)
        match basic_map.get("b") {
            Some(value) => fake_stdout = *value,
            None => fake_stdout = 0,
        }

        // Updating values
        if let Some(val) = basic_map.get_mut("a") {
            *val += 10;
        }

        // Using entry API for more complex updates
        basic_map.entry("c").and_modify(|v| *v *= 2).or_insert(3);

        // Merging maps
        let mut map1 = HashMap::new();
        map1.insert("x", 1);
        let mut map2 = HashMap::new();
        map2.insert("y", 2);

        for (k, v) in map2 {
            map1.entry(k).or_insert(v);
        }

        // Splitting a map into two parts
        let split_map: HashMap<_, _> =
            basic_map.into_iter().filter(|(k, _)| k.chars().next() == Some('a')).collect();

        // Using custom types as keys and values
        #[derive(Hash, Eq, PartialEq)]
        struct KeyType(i32);

        let mut custom_map = HashMap::new();
        custom_map.insert(KeyType(5), "value");

        // Clearing a map
        capacity_map.clear();

        // Dropping a map (explicit memory management)
        drop(entry_map);
        //---

        if *seen_exts.get(&key).unwrap() { [10u8].to_vec() } else { [11u8].to_vec() }
    }

    fn substract(kwargs: Vec<u8>) -> Vec<u8> {
        [kwargs[0] - kwargs[1]].to_vec()
    }
}

bindings::export!(Component with_types_in bindings);
