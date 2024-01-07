use std::fs;

fn main() {
    // let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    let mut reflection_patterns: Vec<Vec<String>> = vec![];
    reflection_patterns.push(vec![]);
    for line in fs::read_to_string(file_path).unwrap().lines(){
        if line.len() > 1{
            let len = reflection_patterns.len()-1;
            reflection_patterns[len].push(line.to_string());
        } else{
            reflection_patterns.push(vec![]);
        }
    }

    println!("{:?}", reflection_patterns);

    // horizontal match (row match)
    let mut horizontal_matches: Vec<(u64, u64, u64, u64)> = vec![];
    for (pattern_num, pattern_vec) in reflection_patterns.iter().enumerate(){
        for (i, reference_pattern) in pattern_vec.iter().enumerate(){
            for (j, checked_pattern) in pattern_vec.iter().enumerate(){ 
                if reference_pattern == checked_pattern && i != j && i+1 == j{
                    // horizontal_matches.push(vec![]);
                    // let last_len = horizontal_matches.len() - 1;
                    horizontal_matches.push((pattern_num as u64, (i+1) as u64, (j+1) as u64, (pattern_vec.len() as f64 / 2.0).ceil() as u64));
                }
            }
        }
    }

    println!("horizontal_matches: {:?}", horizontal_matches);


    // vertical match (column match)
    let mut vertical_matches: Vec<(u64, u64, u64, u64)> = vec![];
    for (pattern_num, pattern_vec) in reflection_patterns.iter().enumerate(){
        let mut current_col = 0;
        while current_col < (pattern_vec[0].len() - 1){
            let mut correct_pos = 0;
            // check every row
            for (i, reference_pattern) in pattern_vec.iter().enumerate(){
                // check every column
                let row_chars = reference_pattern.chars().collect::<Vec<char>>();
                if row_chars[current_col] == row_chars[current_col+1]{
                    correct_pos += 1;
                }
            }
            if correct_pos == pattern_vec.len(){
                // vertical_matches.push(vec![]);
                // let last_len = vertical_matches.len() - 1;
                vertical_matches.push((pattern_num as u64, (current_col+1) as u64, (current_col+2) as u64, (pattern_vec[0].len() as f64 / 2.0).ceil() as u64));
            }
            current_col += 1;
        }
    }

    println!("vertical_matches: {:?}", vertical_matches);

    let mut horizontal_matches_filtered = horizontal_matches.iter().filter(|x| (x.1 as u64) == x.3).collect::<Vec<_>>();
    let mut vertical_matches_filtered = vertical_matches.iter().filter(|x| (x.1 as u64) == x.3).collect::<Vec<_>>();

    println!("horizontal_matches: {:?}", horizontal_matches_filtered);
    println!("vertical_matches: {:?}", vertical_matches_filtered);

    let horizontal_sum: u64 = horizontal_matches_filtered.iter().map(|x| x.1*100).collect::<Vec<u64>>().iter().sum();
    let vertical_sum: u64 = vertical_matches_filtered.iter().map(|x| x.1).collect::<Vec<u64>>().iter().sum();

    println!("the answer is: {}", horizontal_sum+vertical_sum)
}
 // BRB