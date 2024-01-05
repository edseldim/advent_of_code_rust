use std::fs;
use std::collections::HashMap;

/*
So far:

???.### 1,1,3 yields 1 (correct!)
.??..??...?##. 1,1,3 yields 16384 (correct!)
?#?#?#?#?#?#?#? 1,3,1,6 yields 1 (correct!)
????.#...#... 4,1,1 yields 16 (correct!)
????.######..#####. 1,6,5 yields 2500 (correct!)
?###???????? 3,2,1 yields 506250 (correct!)

YESSSSSSSSSSSSSSSSSSSSS
I FINALLY DID IT!!!!!!!!
YAAAAAAAAAAAAAAAAAAAAAAAAAAAAY
*/

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

fn memoised_traverse(springs_input: &String, constraints: &Vec<(usize, usize, usize, usize)>, cache: &mut HashMap<String, u64>) -> u64{


    if springs_input.len() == 0{
        if constraints.len() > 0{
            // println!("no string with constraints thus 0");
            return 0
        } else {
            // println!("no string no constraint thus 1");
            return 1
        }
    }

    // check remaining string
    if constraints.len() == 0{
        if !springs_input.contains("#"){
            // println!("no constraint no remaining blocks thus 1");
            return 1
        }
        // println!("no constraint remaining blocks thus 1");
        return 0
    }

    let key = format!("{:?} {:?}", &springs_input.to_string(), &constraints.to_vec());
    let cached_answer = match cache.get(&key) {
                    Some(combinations) => *combinations as i64,
                    None => -1
                    };

    if cached_answer != -1{
        return cached_answer as u64;
    }

    let mut total_ok_combinations = 0;

    // block data
    let start_char = springs_input.chars().nth(0).unwrap();
    let length = constraints[0].0;
    let start = constraints[0].1;
    let end = constraints[0].2;
    // println!("{} - {} (length: {}, remaining constraints: {:?})",start, end, length, constraints);
    // println!("Remaining springs: {}", springs_input);

    // block conditions
    let does_block_fit = length < springs_input.len();
    let mut is_end_correct = false;
    let mut is_block_correct = false;

    if does_block_fit{ // normal block check
        is_end_correct = &springs_input[length..=length] != "#";
        is_block_correct = !springs_input[..length].contains(".");
    } else if length == springs_input.len() { // last block
        is_end_correct = true;
        is_block_correct = !springs_input[..length].contains(".");
    } else {
        is_end_correct = false;
    }

    if is_block_correct
        && is_end_correct
        && length == springs_input.len()
        && constraints.len() == 1{
            return 1
    }

    // shift block 1 pos
    if ".?".contains(start_char){
        total_ok_combinations = memoised_traverse(&springs_input[1..].to_string(), constraints, cache);
    }

    // block start
    if "#?".contains(start_char)
        && does_block_fit
        && is_block_correct
        && is_end_correct{

                total_ok_combinations += memoised_traverse(&springs_input[length+1..].to_string(), &constraints[1..].to_vec(), cache);
                cache.insert(key, total_ok_combinations);
    }

    return total_ok_combinations
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
            // println!("starting positions for {} (length: {}) with requirements {:?}: {:?}",record, record.len(), req_copy, req_track);
            let mut req_track_to_be_modified = req_track.clone();
            // println!("REQUIREMENTS: {:?}", req_track_to_be_modified);
            // println!("DIST {:?}", get_formatted_record(record, &req_track_to_be_modified));
            total_ok_combinations += memoised_traverse(record, &req_track_to_be_modified, &mut cache);
        }
            
    println!("The total amount of ok combinations is: {}", total_ok_combinations);
    // I DID IT!!!!!
}