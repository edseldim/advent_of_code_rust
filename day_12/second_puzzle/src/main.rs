use std::fs;

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
            // starting position
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
            let mut ok_combinations_cache: Vec<Vec<(usize, usize)>> = vec![];
            let mut ok_combinations = 0;
            loop{
                // starting pointer at the first windows
                let mut current_pointer_values = req_track_to_be_modified[0].clone();
                let mut start = current_pointer_values.1;
                let mut end = current_pointer_values.2;
                let mut available_tiles = current_pointer_values.3 + 1;
                for next_tile in 0..available_tiles{
                    let mut record_windows: Vec<String> = vec![];
                    let mut record_windows_metadata: Vec<(usize, usize)> = vec![];
                    // move first windows freely
                    let mut ok_windows = 0;
                    let sliding_window = record[start+next_tile..=end+next_tile].to_string();
                    record_windows.push(sliding_window.clone());
                    record_windows_metadata.push((start+next_tile, end+next_tile));
                    if sliding_window.chars().all(|x| x!='.'){
                        ok_windows +=1;
                    }
                    // forward checking
                    let mut is_not_valid = false;
                    for next_window in &req_track_to_be_modified[1..]{
                        let mut start = next_window.1;
                        let mut end = next_window.2;
                        let sliding_window = record[start..=end].to_string();
                        if sliding_window.contains(&"."){
                            is_not_valid = true;
                        }
                        record_windows.push(sliding_window.clone());
                        record_windows_metadata.push((start, end));
                        if sliding_window.chars().all(|x| x != '.'){
                            ok_windows +=1;
                        }
                    }

                    if is_not_valid{
                        break;
                    }

                    // println!("checking... {:?}", record_windows);
                    let mut spring_row = record.chars().collect::<Vec<char>>();
                    let mut contiguous_pos = 0;
                    for spring in 0..record.len(){
                        if record_windows_metadata.iter().any(|x| x.0 <= spring && spring <= x.1)
                            && spring_row[spring] != '.'{
                            spring_row[spring] = '$';
                        } else if record_windows_metadata.iter().any(|x| x.0 <= spring && spring <= x.1) {
                            spring_row[spring] = 's';
                            continue;
                        }else if spring_row[spring] == '?'{
                                spring_row[spring] = '.';
                            }
                        }
                    
                    println!("checking... {:?}", spring_row.iter().collect::<String>());

                    // if for some reason, a position is met again, then skip
                    if ok_combinations_cache.contains(&record_windows_metadata){
                        println!("skipped... {:?}", record_windows_metadata);
                        continue;
                    }

                    // check there's no # in between windows
                    // check windows sections are not contiguous
                    let mut is_ok_in_between = true;
                    let mut is_ok_in_sides = true;
                    for (i, window) in record_windows_metadata.iter().enumerate(){
                        if i > 0{
                            // println!("{}: current: {:?} prev: {:?}", i, window, record_windows_metadata[i-1]);
                            let prev_end = record_windows_metadata[i-1].1;
                            let current_start = window.0;
                            if prev_end + 1 == current_start{
                                is_ok_in_between = false;
                                continue;
                            }
                            let in_between_segm = record[prev_end+1..current_start].to_string();
                            // println!("in between: {:?}", in_between_segm);
                            if in_between_segm.contains("#"){
                                is_ok_in_between = false;
                            }
                        }

                        // checking the sides in case of a #
                        if i == 0 || i == record_windows_metadata.len()-1{
                            if i == 0 && window.0 > 0{
                                let left_side_char = record.chars().collect::<Vec<char>>()[window.0-1].to_string();
                                // println!("left side: {:?}", left_side_char);
                                if left_side_char.contains("#"){
                                    is_ok_in_sides = false;
                                }
                            }
                            if i == record_windows_metadata.len()-1 && window.1 < record.len()-1{
                                let right_side_char = record.chars().collect::<Vec<char>>()[window.1+1].to_string();
                                // println!("right side: {:?}", right_side_char);
                                if right_side_char.contains("#"){
                                    is_ok_in_sides = false;
                                }
                            }
                        }
                    }
                    
                    // all windows correctness checking
                    if ok_windows == req_track_to_be_modified.len() 
                        && is_ok_in_between
                        && is_ok_in_sides
                        && !spring_row.contains(&'#')
                        // && spring_row.iter().collect::<String>().split(".").filter(|x| x.to_string().len() > 0).collect::<Vec<_>>().len() == req_track_to_be_modified.len() 
                    {
                        
                        // println!("{:?}", record_windows_metadata);
                        println!("checking... {:?}", spring_row.iter().collect::<String>());
                        println!("match!");
                        ok_combinations += 1;
                        ok_combinations_cache.push(record_windows_metadata.clone());
                    }
                }

                // move next windows one by one
                let mut has_reached_max = false;
                if req_track_to_be_modified.len() == 2{
                    if req_track_to_be_modified[1].2 + 1 == record.len(){
                        has_reached_max = true;
                    }
                    // reset to starting positions except for the head (last pointer)
                    req_track_to_be_modified[1].1 += 1;
                    req_track_to_be_modified[1].2 += 1;
                    req_track[1].1 = req_track_to_be_modified[1].1;
                    req_track[1].2 = req_track_to_be_modified[1].2;
                    
                    for prev_pointer_shift in 0..1{
                        req_track_to_be_modified[prev_pointer_shift].1 = req_track[prev_pointer_shift].1;
                        req_track_to_be_modified[prev_pointer_shift].2 = req_track[prev_pointer_shift].2;
                        req_track_to_be_modified[prev_pointer_shift].3 = req_track[prev_pointer_shift+1].1 - req_track_to_be_modified[prev_pointer_shift].2 - 1;
                    }
                    // println!("new head shift: {:?}", req_track_to_be_modified);

                } else {
                    loop{
                        for next_pointer_shift in 1..=req_track_to_be_modified.len()-1{
                            // if second to first position end is not next to the next window's start
                            if req_track_to_be_modified[next_pointer_shift].2 + 1 < req_track_to_be_modified[next_pointer_shift+1].1{
                                req_track_to_be_modified[next_pointer_shift].1 += 1;
                                req_track_to_be_modified[next_pointer_shift].2 += 1;
                                for prev_pointer_shift in 0..next_pointer_shift{
                                    if prev_pointer_shift + 1 == next_pointer_shift{
                                        req_track_to_be_modified[prev_pointer_shift].1 = req_track[prev_pointer_shift].1;
                                        req_track_to_be_modified[prev_pointer_shift].2 = req_track[prev_pointer_shift].2;
                                        req_track_to_be_modified[prev_pointer_shift].3 = req_track_to_be_modified[prev_pointer_shift+1].1 - req_track_to_be_modified[prev_pointer_shift].2 - 1;
                                    } else {
                                        req_track_to_be_modified[prev_pointer_shift].1 = req_track[prev_pointer_shift].1;
                                        req_track_to_be_modified[prev_pointer_shift].2 = req_track[prev_pointer_shift].2;
                                        req_track_to_be_modified[prev_pointer_shift].3 = req_track[prev_pointer_shift+1].1 - req_track_to_be_modified[prev_pointer_shift].2 - 1;
                                    }
                                }
                                println!("new windows: {:?}", req_track_to_be_modified);
                                break;
                            }
                            // if second to last window has topped its available tiles, then move the last one
                            if next_pointer_shift + 1 == req_track_to_be_modified.len()-1{
                                if req_track_to_be_modified[next_pointer_shift+1].2 + 1 == record.len(){
                                    has_reached_max = true;
                                    break;
                                }
                                // reset to starting positions except for the head (last pointer)
                                req_track_to_be_modified[next_pointer_shift+1].1 += 1;
                                req_track_to_be_modified[next_pointer_shift+1].2 += 1;
                                req_track[next_pointer_shift+1].1 = req_track_to_be_modified[next_pointer_shift+1].1;
                                req_track[next_pointer_shift+1].2 = req_track_to_be_modified[next_pointer_shift+1].2;
                                
                                for prev_pointer_shift in 0..next_pointer_shift+1{
                                    req_track_to_be_modified[prev_pointer_shift].1 = req_track[prev_pointer_shift].1;
                                    req_track_to_be_modified[prev_pointer_shift].2 = req_track[prev_pointer_shift].2;
                                    req_track_to_be_modified[prev_pointer_shift].3 = req_track[prev_pointer_shift+1].1 - req_track_to_be_modified[prev_pointer_shift].2 - 1;
                                }
                                println!("new head shift: {:?}", req_track_to_be_modified);
                                break;
                            }       
                        }

                        let mut is_windows_dist_valid = true;
                        for spring in 0..req_track_to_be_modified.len(){
                            if req_track_to_be_modified.iter().any(|x| x.1 <= spring && spring <= x.2) 
                            && record.chars().collect::<Vec<char>>()[spring] == '.'{
                                is_windows_dist_valid = false;
                            }
                        }
                        if is_windows_dist_valid == false{
                            continue;
                        }
                        break;
                    }
                }
                // if all the record has been traversed, end
                if has_reached_max == true{
                    break;
                }
                
            }
            println!("{}",ok_combinations);
            total_ok_combinations += ok_combinations;
        }
    println!("The total amount of ok combinations is: {}", total_ok_combinations);
}
