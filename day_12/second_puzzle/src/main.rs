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

                    

                    // format record with windows
                    let mut spring_row = record.chars().collect::<Vec<char>>();
                    let mut contiguous_pos = 0;
                    for spring in 0..record.len(){
                        if record_windows_metadata.iter().any(|x| x.0 <= spring && spring <= x.1)
                            && spring_row[spring] != '.'{
                            spring_row[spring] = '$';
                        } else if record_windows_metadata.iter().any(|x| x.0 <= spring && spring <= x.1) {
                            spring_row[spring] = 's';
                            // continue;
                        }else if spring_row[spring] == '?'{
                                spring_row[spring] = '.';
                            }
                        }
                    
                    

                    // if for some reason, a position is met again, then skip
                    if ok_combinations_cache.contains(&record_windows_metadata){
                        println!("skipped cache... {:?}", record_windows_metadata);
                        continue;
                    }

                    if is_not_valid{
                        println!("invalid... {:?}", spring_row.iter().collect::<String>());
                        break;
                    }

                    // println!("checking... {:?}", spring_row.iter().collect::<String>());

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
                        // println!("checking... {:?}", spring_row.iter().collect::<String>());
                        println!("match!");
                        ok_combinations += 1;
                        ok_combinations_cache.push(record_windows_metadata.clone());
                    }
                }

                // move next windows one by one
                let mut has_reached_max = false;
                let mut has_reached_max_not_head = false;
                loop{
                    for next_pointer_shift in 1..=req_track_to_be_modified.len()-1{ 
                            // if there are only contiguous windows, then the last one can be updated
                            if next_pointer_shift == req_track_to_be_modified.len()-1{
                                // println!("updating head {}",next_pointer_shift);
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
                                    let mut start = req_track_to_be_modified[last_window].1;
                                    let mut end = req_track_to_be_modified[last_window].2;
                                    if !record[start..=end].contains("."){
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
                                // println!("new head shift: {:?}", req_track_to_be_modified);
                                let mut spring_row = record.chars().collect::<Vec<char>>();
                                let mut contiguous_pos = 0;
                                for spring in 0..record.len(){
                                    if req_track_to_be_modified.iter().any(|x| x.1 <= spring && spring <= x.2)
                                        && spring_row[spring] != '.'{
                                        spring_row[spring] = '$';
                                    } else if req_track_to_be_modified.iter().any(|x| x.1 <= spring && spring <= x.2) {
                                        spring_row[spring] = 's';
                                    }else if spring_row[spring] == '?'{
                                            spring_row[spring] = '.';
                                        }
                                }
    
                                // println!("{:?}", spring_row.iter().collect::<String>());
                                break;
                            } 
                            // if current windows is not contiguous
                            if req_track_to_be_modified[next_pointer_shift].2 + 1 < req_track_to_be_modified[next_pointer_shift+1].1{
                                // println!("updating window {} {:?}",next_pointer_shift, req_track_to_be_modified);
                                // there's room for moving it
                                loop{
                                    req_track_to_be_modified[next_pointer_shift].1 += 1;
                                    req_track_to_be_modified[next_pointer_shift].2 += 1;
                                    let start = req_track_to_be_modified[next_pointer_shift].1;
                                    let end = req_track_to_be_modified[next_pointer_shift].2;

                                    let mut spring_row = record.chars().collect::<Vec<char>>();
                                    let mut contiguous_pos = 0;
                                    for spring in 0..record.len(){
                                        if req_track_to_be_modified.iter().any(|x| x.1 <= spring && spring <= x.2)
                                            && spring_row[spring] != '.'{
                                            spring_row[spring] = '$';
                                        } else if req_track_to_be_modified.iter().any(|x| x.1 <= spring && spring <= x.2) {
                                            spring_row[spring] = 's';
                                        }else if spring_row[spring] == '?'{
                                                spring_row[spring] = '.';
                                            }
                                    }
                                    
                                    if !spring_row.iter().collect::<String>()[start..=end].contains("s"){
                                        // println!("{}({},{},{:?}) has stopped at {:?}",next_pointer_shift, start, end, spring_row.iter().collect::<String>()[start..=end].to_string(), spring_row.iter().collect::<String>());
                                        break;
                                    }

                                    if end + 1 == req_track_to_be_modified[next_pointer_shift+1].1{
                                        // println!("{}({},{},{:?}) has ended at {:?}",next_pointer_shift, start, end, spring_row.iter().collect::<String>()[start..=end].to_string(), spring_row.iter().collect::<String>());
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
                                // println!("new windows: {:?}", req_track_to_be_modified);

                                if has_reached_max_not_head{
                                    has_reached_max_not_head = false;
                                    continue;
                                }
                                break;
                            } 
                        }
                        
                        

                        if has_reached_max || has_reached_max_not_head{
                            break;
                        }
                        // println!("{:?}", req_track_to_be_modified);
                        // skip dist check if there's an invalid window
                        let mut is_windows_dist_valid = true;
                        for (i, window) in req_track_to_be_modified.iter().enumerate(){
                            if i > 1{
                                // println!("{}: current: {:?} prev: {:?}", i, window, req_track_to_be_modified[i-1]);
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
                            
                            let mut spring_row = record.chars().collect::<Vec<char>>();
                            let mut contiguous_pos = 0;
                            for spring in 0..record.len(){
                                if req_track_to_be_modified.iter().any(|x| x.1 <= spring && spring <= x.2)
                                    && spring_row[spring] != '.'{
                                    spring_row[spring] = '$';
                                } else if req_track_to_be_modified.iter().any(|x| x.1 <= spring && spring <= x.2) {
                                    spring_row[spring] = 's';
                                }else if spring_row[spring] == '?'{
                                        spring_row[spring] = '.';
                                    }
                            }

                            // println!("skipped... {:?}", spring_row.iter().collect::<String>());
                            continue;
                        }
                        break;
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

/*
TODO:
1. Que las cabeceras no tengan ninguna s (sin puntos)
2. Que las cabeceras cuando lleguen al final, no hagan iteraciones innecesarias. Que si llegan al final, haga continue
y la otra se mueva hasta que encuentre una ventana sin s (sin puntos) */