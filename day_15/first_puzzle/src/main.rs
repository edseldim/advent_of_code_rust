use std::fs;

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

    
    let mut hash_values: Vec<u32> = vec![];
    for hash in hashes{
        let mut hash_value = 0;
        for hash_char in hash.chars(){
            hash_value += hash_char as u32;
            hash_value *= 17;
            hash_value = hash_value % 256;
        }
        hash_values.push(hash_value);
    }
    println!("{:?}",hash_values);
    println!("sum of hashes is {}",hash_values.iter().sum::<u32>());
}
