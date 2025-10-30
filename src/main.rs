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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("datatest/test.json")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let content: Table = serde_json::from_str(&contents)?;

    let map = get_random_key_value_pair(&content.content);
    println!("{:?}", map);


    //println!("Returned: {:?}", get_random_key_value_pair(&content.content));

    //println!("Obj: {:?}", content.content.get("characters").unwrap());

    Ok(())
}
