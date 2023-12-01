use std::fs;

fn retrieve_calibration_values_from_vec(vec_ref: Vec<String>) -> Vec<Vec<(i32, String)>> {
    let mut numbers_in_file = Vec::new();
    for (i, string) in vec_ref.iter().enumerate(){
        let mut numbers_in_string = Vec::new();
        /* Extracting words as digits */
        for number_str in vec!["one","two","three","four","five","six","seven","eight","nine"]{
            let found_number_first: (i32,String) = match string.find(number_str) {
                                    Some(start_num) => (start_num as i32, number_str.to_string()),
                                    None => (-1,String::from("None"))
                                };

            let found_number_last: (i32,String) = match string.rfind(number_str) {
                Some(start_num) => (start_num as i32, number_str.to_string()),
                None => (-1,String::from("None"))
            };

            if found_number_first.0 != -1 {
            numbers_in_string.push(found_number_first)
            }

            if found_number_last.0 != -1 {
                numbers_in_string.push(found_number_last)
            }
        }
        /* Extracting Digits */
        for (i, letter) in string.chars().enumerate(){
            const RADIX: u32 = 10;
            if letter.is_digit(RADIX) {
                numbers_in_string.push((i as i32,letter.to_string()))
            };
        }
        numbers_in_file.push(numbers_in_string);
    }
    numbers_in_file

}

fn get_max_value_from_vec(vec: Vec<(i32,String)>) -> (i32, String) {
    let mut max_pos = -1;
    let mut max_value = String::from("None");
    for (i, (pos, number)) in vec.iter().enumerate(){
        if i == 0{
            max_pos = *pos;
            max_value = number.clone();
        }
        
        if *pos > max_pos {
            max_pos = *pos;
            max_value = number.clone();
        }
    }

    (max_pos,max_value)

}

fn get_min_value_from_vec(vec: Vec<(i32,String)>) -> (i32, String) {
    let mut min_pos = -1;
    let mut min_value = String::from("None");
    for (i, (pos, number)) in vec.iter().enumerate(){
        if i == 0{
            min_pos = *pos;
            min_value = number.clone();
        }
        
        if *pos < min_pos {
            min_pos = *pos;
            min_value = number.clone();
        }
    }

    (min_pos,min_value)

}

fn retrieve_value_from_str(number_str: String) -> i32{
    if number_str.len() == 1 {
        const RADIX: u32 = 10;
        number_str.chars().nth(0).unwrap().to_digit(RADIX).unwrap() as i32
    } else {
        match &number_str as &str {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0
        }
    }
}

fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };

    let mut processed_lines = retrieve_calibration_values_from_vec(file_lines.clone());

    let mut total_sum = 0;
    for (i, numbers_in_vector) in processed_lines.iter().enumerate(){
        println!("iteration {i}");
        if numbers_in_vector.len() == 1{
            let unique_value = retrieve_value_from_str(numbers_in_vector[0].1.clone());
            println!("{} detected!", unique_value);
            let processed_number = dbg!(unique_value*10+unique_value);
            println!("{}",processed_number);
            total_sum += processed_number;
        }else{
            let first_value = get_min_value_from_vec(numbers_in_vector.clone()).1;
            let last_value = get_max_value_from_vec(numbers_in_vector.clone()).1;
            println!("{} and {} detected!", first_value, last_value);
            let processed_number = dbg!(retrieve_value_from_str(first_value) * 10 + retrieve_value_from_str(last_value));
            println!("{}",processed_number);
            total_sum += processed_number;
        }
    }
    println!("The total sum of the digits in the file is {}", total_sum);
}