use std::env;
use std::fs;

fn retrieve_calibration_values_from_vec(vec_ref: Vec<String>) -> Vec<Vec<u32>> {
    let mut numbers_in_file = Vec::new();
    for (i, string) in vec_ref.iter().enumerate(){
        let mut numbers_in_string = Vec::new();
        for letter in string.chars(){
            const RADIX: u32 = 10;
            if letter.is_digit(RADIX) {
                numbers_in_string.push(letter.to_digit(RADIX).unwrap())
            };
        }
        numbers_in_file.push(numbers_in_string);
    }
    numbers_in_file

}

fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };
    println!("{:?}",file_lines);
    let mut processed_lines = retrieve_calibration_values_from_vec(file_lines.clone());
    println!("{:?}",processed_lines);

    let mut total_sum = 0;
    for numbers_in_vector in &processed_lines{
        if numbers_in_vector.len() == 1{
            total_sum += numbers_in_vector[0]*10+numbers_in_vector[0];
        }else{
            let last_pos = numbers_in_vector.len() - 1;
            let first_pos = 0;
            let processed_number = numbers_in_vector[first_pos]*10 + numbers_in_vector[last_pos];
            total_sum += processed_number;
        }
    }
    println!("The total sum of the digits in the file is {}", total_sum);
}