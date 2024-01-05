use std::fs;
use std::collections::HashMap;

fn get_formatted_record(record: &String, current_requirements: &Vec<(usize, usize, usize, usize)>) -> String{
    let mut spring_row = record.chars().collect::<Vec<char>>();
    for spring in 0..record.len(){
        if current_requirements.iter().any(|x| x.1 <= spring && spring <= x.2)
            && spring_row[spring] != '.'{
            spring_row[spring] = '$';
        } else if current_requirements.iter().any(|x| x.1 <= spring && spring <= x.2) {
            spring_row[spring] = 's';
        }else if spring_row[spring] == '?'{
                spring_row[spring] = '.';
            }
    }
    return spring_row.iter().collect::<String>()
}

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
                    ok_combinations += 0;
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
            let mut cache: HashMap<String, u64> = HashMap::new();
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
            let mut req_track_to_be_modified = req_track.clone();
            /*
            GOALS:
            1. Make algo to update positions
             */
            loop {
                println!("REQUIREMENTS: {:?}", req_track_to_be_modified);
                // println!("DIST {:?}", get_formatted_record(record, &req_track_to_be_modified));
                total_ok_combinations += memoised_traverse(record, req_track_to_be_modified.clone(), &mut cache);

                // move next windows one by one
                let mut has_reached_max = false;
                let mut has_reached_max_not_head = false;
                loop{
                    for next_pointer_shift in 1..=req_track_to_be_modified.len()-1{

                            // if there are only contiguous windows, then the last one can be updated
                            if next_pointer_shift == req_track_to_be_modified.len()-1{
                                let mut last_window = req_track_to_be_modified.len()-1;
                                // has last window reached end?
                                if req_track_to_be_modified[last_window].2 == record.len() - 1{
                                    has_reached_max = true;
                                    break;
                                }
                                // reset to starting positions except for the head (last pointer)
                                loop{
                                    req_track_to_be_modified[last_window].1 += 1;
                                    req_track_to_be_modified[last_window].2 += 1;
                                    req_track[last_window].1 = req_track_to_be_modified[last_window].1;
                                    req_track[last_window].2 = req_track_to_be_modified[last_window].2;

                                    if req_track_to_be_modified[last_window].2 == record.len() - 1{
                                        has_reached_max = true;
                                        break;
                                    }

                                    let mut start = req_track_to_be_modified[last_window].1;
                                    let mut end = req_track_to_be_modified[last_window].2;
                                    let left_side = record[start-1..=start-1].to_string();
                                    let right_side = record[end+1..=end+1].to_string();

                                    if !record[start..=end].contains(".")
                                        && !left_side.contains("#") 
                                        && !right_side.contains("#") 
                                    {
                                        break;
                                    }

                                    if req_track_to_be_modified[last_window].2 == record.len() - 1{
                                        has_reached_max = true;
                                        break;
                                    }
                                }
    
                                if has_reached_max{
                                    break;
                                }
                                
                                
                                // reset all the previous positions
                                for prev_pointer_shift in 0..last_window{
                                    req_track_to_be_modified[prev_pointer_shift].1 = req_track[prev_pointer_shift].1;
                                    req_track_to_be_modified[prev_pointer_shift].2 = req_track[prev_pointer_shift].2;
                                    req_track_to_be_modified[prev_pointer_shift].3 = req_track[prev_pointer_shift+1].1 - req_track[prev_pointer_shift].2 - 1;
                                }
                                // println!("new head... {:?}", get_formatted_record(record, &req_track_to_be_modified));
                                break;
                            }

                            // if current windows is not contiguous
                            if req_track_to_be_modified[next_pointer_shift].2 + 1 < req_track_to_be_modified[next_pointer_shift+1].1{
                                // there's room for moving it
                                loop{
                                    req_track_to_be_modified[next_pointer_shift].1 += 1;
                                    req_track_to_be_modified[next_pointer_shift].2 += 1;
                                    let start = req_track_to_be_modified[next_pointer_shift].1;
                                    let end = req_track_to_be_modified[next_pointer_shift].2;
    
                                    let next_start = req_track_to_be_modified[next_pointer_shift+1].1;
                                    let in_between_segm = record[end+1..next_start].to_string();
                                    let left_side = record[start-1..=start-1].to_string();
                                    
                                    if !get_formatted_record(record, &req_track_to_be_modified)[start..=end].contains("s")
                                        && !left_side.contains("#")
                                        && !in_between_segm.contains("#")
                                    {
                                        break;
                                    }
    
                                    if end + 1 == req_track_to_be_modified[next_pointer_shift+1].1{
                                        has_reached_max_not_head = true;
                                        break;
                                    }
                                }
    
                                // reset all previous windows to original
                                for prev_pointer_shift in 0..next_pointer_shift{
                                    req_track_to_be_modified[prev_pointer_shift].1 = req_track[prev_pointer_shift].1;
                                    req_track_to_be_modified[prev_pointer_shift].2 = req_track[prev_pointer_shift].2;
                                    // if it's the last one (before the one we are analyzing)
                                    if prev_pointer_shift + 1 == next_pointer_shift{
                                        req_track_to_be_modified[prev_pointer_shift].3 = req_track_to_be_modified[next_pointer_shift].1 - req_track[prev_pointer_shift].2 - 1;
                                    } else {
                                        // otherwise, reset the distances just like the beginning
                                        req_track_to_be_modified[prev_pointer_shift].3 = req_track[prev_pointer_shift+1].1 - req_track_to_be_modified[prev_pointer_shift].2 - 1;
                                    }
                                }
    
                                if has_reached_max_not_head{
                                    has_reached_max_not_head = false;
                                    continue;
                                }

                                // println!("new pointer switch... {:?}", get_formatted_record(record, &req_track_to_be_modified));
                                break;
                            } 
                        }
                        
                        for prev_pointer_shift in 0..req_track_to_be_modified.len()-1{
                            req_track_to_be_modified[prev_pointer_shift].3 = req_track_to_be_modified[prev_pointer_shift+1].1 - req_track_to_be_modified[prev_pointer_shift].2 - 1;
                        }
                        
    
                        if has_reached_max || has_reached_max_not_head{
                            break;
                        }
                        
                        // skip dist check if there's an invalid window
                        let mut is_windows_dist_valid = true;
                        for (i, window) in req_track_to_be_modified.iter().enumerate(){
                            if i > 1{

                                let prev_end = req_track_to_be_modified[i-1].2;
                                let current_start = window.1;
                                // check if contiguous
                                if prev_end + 1 == current_start{
                                    is_windows_dist_valid = false;
                                    break;
                                }
                                let in_between_segm = record[prev_end+1..current_start].to_string();
                                // check if invalid
                                if in_between_segm.contains("#"){
                                    is_windows_dist_valid = false;
                                    break;
                                }
                            }
                        }

                        // if dist is skipeable, then skip it
                        if is_windows_dist_valid == false{
                            // println!("skipped... {:?}", get_formatted_record(record, &req_track_to_be_modified));
                            continue;
                        }

                        break;
                    }

                // if all the record has been traversed, end
                if has_reached_max == true{
                    break;
                }
            }
        }
            
    println!("The total amount of ok combinations is: {}", total_ok_combinations);
}