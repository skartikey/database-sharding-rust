use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::hash::DefaultHasher;
use std::num::Wrapping;

const NUMBER_OF_SHARDS: usize = 4;

#[derive(Clone)]
struct Shard {
    data: HashMap<String, String>, // Simulating a simple key-value store
}

impl Shard {
    fn new() -> Self {
        Shard {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn update(&mut self, key: String, value: String) -> Option<String> {
        self.data.insert(key, value)
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}

struct ShardedDatabase {
    shards: Vec<Shard>,
}

impl ShardedDatabase {
    fn new() -> Self {
        ShardedDatabase {
            shards: vec![Shard::new(); NUMBER_OF_SHARDS],
        }
    }

    fn determine_shard(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (Wrapping(hasher.finish() as usize) % Wrapping(NUMBER_OF_SHARDS)).0 as usize
    }

    fn shard_mut(&mut self, key: &str) -> &mut Shard {
        let shard_id = self.determine_shard(key);
        &mut self.shards[shard_id]
    }

    fn insert(&mut self, key: String, value: String) {
        self.shard_mut(&key).insert(key, value);
    }

    fn get(&mut self, key: &str) -> Option<&String> {
        self.shard_mut(key).get(key)
    }

    fn update(&mut self, key: String, value: String) -> Option<String> {
        self.shard_mut(&key).update(key, value)
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        self.shard_mut(key).delete(key)
    }
}

fn main() {
    let mut db = ShardedDatabase::new();

    // Insert some data
    println!("Insert some data");
    db.insert("user1".to_string(), "Alice".to_string());
    db.insert("user2".to_string(), "Bob".to_string());

    // Retrieve and print a value
    println!("Print all");
    if let Some(name) = db.get("user1") {
        println!("Found: {}", name);
    }
    if let Some(name) = db.get("user2") {
        println!("Found: {}", name);
    }

    // Update a value
    println!("Update user1");
    db.update("user1".to_string(), "Alicia".to_string());

    // Retrieve and print a value
    println!("Print all");
    if let Some(name) = db.get("user1") {
        println!("Found: {}", name);
    }
    if let Some(name) = db.get("user2") {
        println!("Found: {}", name);
    }

    // Delete a value
    println!("Update user2");
    db.delete("user2");

    // Try to retrieve a deleted value
    if db.get("user2").is_none() {
        println!("User2 deleted successfully");
    }

    // Retrieve and print a value
    println!("Print all");
    if let Some(name) = db.get("user1") {
        println!("Found: {}", name);
    }
    if let Some(name) = db.get("user2") {
        println!("Found: {}", name);
    }
}
