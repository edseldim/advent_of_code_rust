use std::fs;
use std::collections::{HashMap, HashSet};
use std::cmp;

fn main(){
    let file_path = "src/puzzle_data.txt";
    let mut wires: HashMap<String, HashSet<String>> = HashMap::new();
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let left = line.split(":").nth(0).unwrap().trim().to_string();
        let right = line.split(":").nth(1).unwrap()
                                    .split_whitespace()
                                    .map(|node| node.to_string())
                                    .collect::<Vec<String>>();

        for node in &right{
            wires.entry(left.clone()).or_insert(HashSet::new());
            wires.get_mut(&left).unwrap().insert(node.clone());
            wires.entry(node.to_string()).or_insert(HashSet::new());
            wires.get_mut(node).unwrap().insert(left.clone());
        }
        
    }

    let mut wire_group = wires.keys().map(|key| key.to_string()).collect::<HashSet<String>>();
    while wire_group.iter()
                .map(|key| wires.get(&key.to_string()).unwrap().difference(&wire_group).collect::<HashSet<&String>>().len())
                .sum::<usize>() != 3{
        let mut keys_condition = wire_group.iter()
                                .map(|key| (key.to_string(), wires.get(&key.to_string()).unwrap().difference(&wire_group).collect::<HashSet<&String>>().len()))
                                .collect::<Vec<(String, usize)>>();
        keys_condition.sort_by(|a,b| b.1.cmp(&a.1));
        wire_group.remove(&keys_condition[0].0);
    }

    println!("{}", wire_group.len() * wires.keys().map(|key| key.to_string()).collect::<HashSet<String>>().difference(&wire_group).collect::<HashSet<&String>>().len())
}