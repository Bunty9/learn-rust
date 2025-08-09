//hashmaps in rust store a key-value pair
// similar to objects in js, dict in py, hashmap in java
// methods: insert, get, remove, clear

use std::collections::HashMap;

pub fn demonstrate_hashmaps() {
    let mut users = HashMap::new();

    users.insert(String::from("Alice"), 30);
    users.insert(String::from("Bob"), 32);

    let first_user_age = users.get("Bob"); //returs an Option enum for the get method
    //pattern match to get the value if it exists
    match first_user_age {
        Some(age) => println!("First user age: {}", age),
        None => println!("No user found in DB"),
    }

    // vector pairs
    let pairs = vec![
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
        ("apple".to_string(), 3),
        ("banana".to_string(), 4),
        ("apple".to_string(), 5),
    ];
    let grouped_pairs = group_by_key_pairs(pairs);
    println!("grouped pairs: {:?}", grouped_pairs);
}

// Write a function that takes a vector of tuples (each tuple containing a key and a
// value) and returns a Hashmap where the keys are the unique keys from the input tuples
// and the values are vectors of all corresponding values associated with each key.

fn group_by_key_pairs(pairs: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();

    for (key, value) in pairs {
        map.entry(key).or_insert(Vec::new()).push(value);
    }
    return map;
}
