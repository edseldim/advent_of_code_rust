use std::fs;

fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };

    /* parse race information */
    let race_max_time:i64 = file_lines[0]
                    .split(":")
                    .collect::<Vec<&str>>()[1]
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("")
                    .parse().unwrap()
                    ; 
    let race_records:i64 = file_lines[1]
                    .split(":")
                    .collect::<Vec<&str>>()[1]
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("")
                    .parse().unwrap()
                    ; 
    
    println!("max times: {:?}", race_max_time);
    println!("records: {:?}", race_records);
    let mut race_record_breaking_options = vec![];
    race_record_breaking_options.push(0);
    for hold_time in 0..=race_max_time{
        let distance_traveled = hold_time * (race_max_time-hold_time);
        if distance_traveled > race_records{
            race_record_breaking_options[0]+=1;
        }
    } 
    println!("record breaking options for each race: {:?}", race_record_breaking_options);
    println!("total record breaking options are: {}",{let mut total = 1; for record_opt in race_record_breaking_options {total*=record_opt} total})
}
