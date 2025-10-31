use std::hash::Hash;
use std::{collections::HashMap, fs::File};
use std::io::Read;
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Deserialize, Serialize)]
struct Table {
    name: String,
    content: HashMap<String, Vec<String>>,
}

fn get_random_key_value_pair<K, >(map: &HashMap<K, Vec<String>>) -> Option<(&K, &String)>
where
    K: Clone, K: Eq, K: Hash // K needs to be cloneable to be stored in the Vec
{
    if map.is_empty() {
        return None;
    }

    let keys: Vec<&K> = map.keys().collect();
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..keys.len());

    let random_key = keys[random_index];
    let vec = map.get(random_key).unwrap();

    let random_v_index = rng.random_range(0..vec.len());

    Some((random_key, &vec[random_v_index]))
}

// change to something like, "collect question", since this func just reads the file and grabs a
// random phrase and word
fn load_game_data() -> Vec<String> {
    let mut file = File::open("datatest/test.json").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to open file");

    let content: Table = serde_json::from_str(&contents).unwrap();

    let map = get_random_key_value_pair(&content.content);

    let mut arr = Vec::new();

    arr.push(map.unwrap().0.to_owned());
    arr.push(map.unwrap().1.to_owned());

    return arr;
}

fn main() {
    let data = load_game_data();
    println!("{:?}", data);
}
