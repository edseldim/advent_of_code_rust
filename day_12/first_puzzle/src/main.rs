use combinations::Combinations;
use std::collections::HashMap;
use std::fs;

fn get_total_combinatorics(req: Vec<usize>, sample: i32, unknown_part: Vec<usize>) -> usize {
    let total_count = 0;
    let mut number_req_passed = 0;
    if sample as usize == unknown_part.len(){
        return 1
    }else{
        let computed: Vec<_> = Combinations::new(unknown_part.clone(), sample as usize).collect();
        for combination in computed{
            // println!("Analyzing {:?}", combination);
            let mut contiguous_pos = 0;
            let mut last_number = 0;
            let mut req_copy = req.clone();
            for digit in &combination{
                if contiguous_pos == 0{
                    contiguous_pos += 1;
                    last_number = *digit;
                } else {
                    if *digit == (last_number+1){
                        contiguous_pos += 1;
                        last_number = *digit;
                    }
                }
                // println!("{} {}",last_number, contiguous_pos);
                if req_copy.contains(&contiguous_pos){
                    // println!("passed!");
                    contiguous_pos = 0;
                    let req_completed_idx = req_copy.iter().enumerate().filter(|x| x.0 == contiguous_pos).collect::<Vec<(usize, &usize)>>();
                    if req_completed_idx.len() == 0{
                        break;
                    }
                    req_copy.remove(req_completed_idx[0].0);

                    if req_copy.len() == 0{
                        number_req_passed += 1;
                        break;
                    }
                }   
            }
        }
    }
    // if we are sampling just one number, no need for sequential cases correction
    if sample == 1{
        return number_req_passed;
    }
    // remove sequential cases
    let mut last_result = number_req_passed as i32-(unknown_part.len() as i32 - sample + 1);
    // if last_result == 0{
    //     return 1;
    // }
    return last_result as usize
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
        let spring_record = line
                        .split_whitespace()
                        .collect::<Vec<&str>>()[0].to_string();
        let requirement = line
                    .split_whitespace()
                    .collect::<Vec<&str>>()[1]
                    .split(",")
                    .map(|x| x.to_string())
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<usize>>();
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
            // let mut current_pointer = 0;
            // let mut record_windows: Vec<String> = vec![];
            // let mut record_windows_metadata: Vec<(usize, usize)> = vec![];
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
                    if sliding_window.chars().any(|x| x=='.') == false{
                        ok_windows +=1;
                    }
                    // forward checking
                    for next_window in &req_track_to_be_modified[1..]{
                        let mut start = next_window.1;
                        let mut end = next_window.2;
                        let sliding_window = record[start..=end].to_string();
                        record_windows.push(sliding_window.clone());
                        record_windows_metadata.push((start, end));
                        if sliding_window.chars().any(|x| x == '.') == false{
                            ok_windows +=1;
                        }
                    }
                    println!("checking... {:?}", record_windows);
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
                            }
                            let in_between_segm = record[prev_end+1..current_start].to_string();
                            if in_between_segm.contains("#"){
                                is_ok_in_between = false;
                            }
                        }

                        // checking the sides in case of a #
                        if i == 0 || i == record_windows_metadata.len()-1{
                            if i == 0 && window.0 > 0{
                                let left_side_char = record.chars().collect::<Vec<char>>()[window.0-1].to_string();
                                if left_side_char.contains("#"){
                                    is_ok_in_sides = false;
                                }
                            }
                            if i == 0 && window.1 < record.len()-1{
                                let right_side_char = record.chars().collect::<Vec<char>>()[window.1+1].to_string();
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
                    {
                    
                        println!("match!");
                        ok_combinations += 1;
                        ok_combinations_cache.push(record_windows_metadata.clone());
                    }
                }

                // move next windows one by one
                let mut has_reached_max = false;
                for next_pointer_shift in 1..req_track_to_be_modified.len()-1{
                    // if second to first position end is not next to the next window's start
                    if req_track_to_be_modified[next_pointer_shift].2 + 1 < req_track_to_be_modified[next_pointer_shift+1].1{
                        req_track_to_be_modified[next_pointer_shift].1 += 1;
                        req_track_to_be_modified[next_pointer_shift].2 += 1;
                        for prev_pointer_shift in 0..next_pointer_shift{
                            req_track_to_be_modified[prev_pointer_shift].3 = req_track_to_be_modified[prev_pointer_shift+1].1 - req_track_to_be_modified[prev_pointer_shift].2 - 1;
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
