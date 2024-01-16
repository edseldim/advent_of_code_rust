use std::fs;

fn calculate_hash_formula(hash: &str) -> u32{
    let mut hash_value = 0;
    for hash_char in hash.chars(){
        hash_value += hash_char as u32;
        hash_value *= 17;
        hash_value = hash_value % 256;
    }
    hash_value
}

fn main() {
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }
    
    let mut hashes = file_lines[0].split(",")
                                    .collect::<Vec<&str>>();
    println!("{:?}",hashes);

    /*
    The current value starts at 0.
    The first character is H; its ASCII code is 72.
    The current value increases to 72.
    The current value is multiplied by 17 to become 1224.
    The current value becomes 200 (the remainder of 1224 divided by 256).
    The next character is A; its ASCII code is 65.
    The current value increases to 265.
    The current value is multiplied by 17 to become 4505.
    The current value becomes 153 (the remainder of 4505 divided by 256).
    The next character is S; its ASCII code is 83.
    The current value increases to 236.
    The current value is multiplied by 17 to become 4012.
    The current value becomes 172 (the remainder of 4012 divided by 256).
    The next character is H; its ASCII code is 72.
    The current value increases to 244.
    The current value is multiplied by 17 to become 4148.
    The current value becomes 52 (the remainder of 4148 divided by 256).
    */

    
    let mut boxes = std::iter::repeat::<Vec<(String,u32)>>(vec![]).take(256).collect::<Vec<_>>();
    for hash in hashes{
        if hash.to_string().chars().collect::<Vec<char>>().contains(&'-'){
            let hash_string = hash.to_string().clone();
            let box_lens = hash_string.split("-").nth(0).unwrap();
            let box_number = calculate_hash_formula(&box_lens);
            let pos_to_be_deleted = match boxes[box_number as usize].iter().enumerate().filter(|x| x.1.0 == box_lens).nth(0){
                                        Some(lens) => lens.0 as i32,
                                        None => -1
                                    };
            println!("removing from box #{} {:?} at position {}",box_number,hash,pos_to_be_deleted);
            if pos_to_be_deleted >= 0{
                boxes[box_number as usize].remove(pos_to_be_deleted as usize);
            }
        } else {
            let hash_string = hash.to_string().clone();
            let box_lens = hash_string.split("=").nth(0).unwrap();
            let focal_length = hash_string.split("=").nth(1).unwrap().parse().unwrap();
            let box_number = calculate_hash_formula(&box_lens);
            let pos_to_be_added_at = match boxes[box_number as usize].iter().enumerate().filter(|x| x.1.0 == box_lens).nth(0){
                Some(lens) => lens.0 as i32,
                None => -1
            };
            println!("adding to box #{} {:?} at position {}",box_number, hash, pos_to_be_added_at);
            // println!("current_box #{}: {:?}",box_number, boxes[box_number as usize]);
            if pos_to_be_added_at >= 0{
                boxes[box_number as usize][pos_to_be_added_at as usize] = (box_lens.to_string(), focal_length);
            } else {
                boxes[box_number as usize].push((box_lens.to_string(), focal_length));
            }
        }
    }

    let mut focusing_power = 0;
    for (box_number, lens_box) in boxes.iter().enumerate(){
        focusing_power += lens_box.iter().enumerate().map(|x| (box_number+1)*(x.0+1)*(x.1.1 as usize)).sum::<usize>();
    }

    println!("box summary: {:?}", boxes);
    println!("focusing power: {:?}", focusing_power);
}
