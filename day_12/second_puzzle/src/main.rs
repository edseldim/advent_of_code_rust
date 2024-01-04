use std::fs;
use std::collections::HashMap;

fn memoised_traverse(springs_input: &String, constraints: Vec<(usize, usize, usize, usize)>, cache: &mut HashMap<String, u64>) -> u64{
    let mut ok_combinations = 0;
    println!("constraint at same level: {:?}", constraints);
    // for (i, current_constraint) in constraints.iter().enumerate(){
    let current_constraint = constraints[0];
    println!("current constraint: {:?}", current_constraint);
    let start = current_constraint.1;
    let end = current_constraint.2;
    let available_tiles = current_constraint.3;
    for i in 0..=available_tiles{
        let new_start = start+i;
        let new_end = end+i;
        let current_window = &springs_input[new_start..=new_end];
        
        let mut next_tile_symbol = "";
        let mut has_reached_end = false;
        if new_end+1 == springs_input.len(){
            has_reached_end = true;
        } else {
            next_tile_symbol = &springs_input[new_end+1..=new_end+1];
        }

        let mut prev_tile_symbol = "";
        let mut is_at_beginning = false;
        if new_start == 0{
            is_at_beginning = true;
        } else {
            prev_tile_symbol = &springs_input[new_start-1..=new_start-1];
        }

        println!("current substr: {:?}{:?}{:?}", prev_tile_symbol,current_window,next_tile_symbol);
        if !current_window.contains(".")
        && !next_tile_symbol.contains("#")
        && !prev_tile_symbol.contains("#")
        {
            if constraints.len() == 1{
                println!("score!");
                ok_combinations += 1;
            } else {
                let mut next_iter_shift = 0;
                if new_end < springs_input.len()-1{
                    next_iter_shift = 2;
                }

                let combination_key = format!("{:?} {:?}",&springs_input[new_end+next_iter_shift..].to_string(), constraints[0]);
                let retrieved_combination = match cache.get(&combination_key)
                {
                    Some(a) => *a as i16,
                    None => -1
                };
                if retrieved_combination != -1{
                    // ok_combinations = retrieved_combination as u64;
                    ok_combinations = 0;
                } else {
                    println!("Cache at {:?}: {:?}", current_constraint, cache);
                    ok_combinations += memoised_traverse(&springs_input.to_string(), constraints[1..].to_vec(), cache);
                    cache.insert(combination_key, ok_combinations);
                }
                
            }
        }
    }
    println!("Ended traverse ({:?})... Ok combinations so far: {}", current_constraint, ok_combinations);
    // }
    return ok_combinations;
}

fn main() {
    
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }

    /* requirements and record parse */
    let mut spring_records: Vec<String> = vec![]; 
    let mut requirements: Vec<Vec<usize>> = vec![];
    for line in &file_lines{
        let mut spring_record = line
                        .split_whitespace()
                        .collect::<Vec<&str>>()[0]
                        .to_string();
        spring_record.push_str("?");
        spring_record = spring_record.repeat(5)
                        .chars()
                        .rev()
                        .collect::<Vec<char>>()[1..]
                        .iter()
                        .rev()
                        .collect::<String>();
        
        let mut requirement = line
                    .split_whitespace()
                    .collect::<Vec<&str>>()[1]
                    .split(",")
                    .map(|x| x.to_string())
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<usize>>();
        requirement = requirement.iter().cycle().take(requirement.len() * 5).map(|x| *x).collect();
        spring_records.push(spring_record);
        requirements.push(requirement);
    }
    let mut total_ok_combinations = 0;
    for (record_num, record) in spring_records.iter().enumerate(){
            let mut current_idx = 0;
            let mut req_copy = requirements[record_num].clone();
            let mut req_track: Vec<(usize,usize,usize, usize)> = vec![];
            // new_starting position
            for (i, req) in req_copy.iter().enumerate(){
                let mut start = current_idx;
                let mut end = *req+current_idx-1;
                if i == req_copy.len() - 1{
                    current_idx = end + 1;
                } else {
                    current_idx = end+2;
                }
                
                req_track.push((*req, start, end, current_idx - end - 1)); // (requirement, start, end, available tiles)
            }
            println!("starting positions for {} (length: {}) with requirements {:?}: {:?}",record, record.len(), req_copy, req_track);
            /*
            GOALS:
            1. Make algo to update positions
             */
            let mut cache: HashMap<String, u64> = HashMap::new();
            total_ok_combinations += memoised_traverse(record, req_track, &mut cache);

        }

            
    println!("The total amount of ok combinations is: {}", total_ok_combinations);
}