use std::fs;

fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };

    /* parse race information */
    let race_max_time = file_lines[0]
                    .split(":")
                    .collect::<Vec<&str>>()[1]
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>(); 
    let race_records = file_lines[1]
                    .split(":")
                    .collect::<Vec<&str>>()[1]
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>(); 
    
    println!("max times: {:?}", race_max_time);
    println!("records: {:?}", race_records);
    let mut race_record_breaking_options = vec![];
    for (i, time) in race_max_time.iter().enumerate(){
        race_record_breaking_options.push(0);
        for hold_time in 0..=*time{
            let distance_traveled = hold_time * (time-hold_time);
            if distance_traveled > race_records[i]{
                race_record_breaking_options[i]+=1;
            }
        }           
    }
    println!("record breaking options for each race: {:?}", race_record_breaking_options);
    println!("total record breaking options are: {}",{let mut total = 1; for record_opt in race_record_breaking_options {total*=record_opt} total})
}
