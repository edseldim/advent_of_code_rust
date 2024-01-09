use std::fs;

fn sort_vector(vector_to_be_sorted: &mut Vec<(u64,u64,u64,u64)>){
    for i in 0..vector_to_be_sorted.len(){
        for j in 0..vector_to_be_sorted.len()-i-1{
            if vector_to_be_sorted[j] < vector_to_be_sorted[j+1]{
                let mut aux = vector_to_be_sorted[j+1];
                vector_to_be_sorted[j+1] = vector_to_be_sorted[j];
                vector_to_be_sorted[j] = aux;
            }
        }
    }
}

fn find_centre(matches_vector: &Vec<(u64,u64,u64,u64)>) -> Vec<(u64,u64,u64)>{
    let mut centres: Vec<(u64,u64,u64)> = vec![];
    let mut left_bef = 0;
    let mut right_bef = 0;
    let mut current_centre_depth = 0;
    for (i, matches) in matches_vector.iter().enumerate(){

        if i == 0{
            left_bef = matches.1;
            right_bef = matches.2;
            current_centre_depth += 1;
            continue;
        } else if left_bef == (matches.1 - 1) && right_bef == (matches.2 + 1){
            current_centre_depth += 1;
            if matches.1 == matches.2 - 1{
                centres.push((matches.1,matches.2,current_centre_depth));
            }
        } else {
            current_centre_depth = 0;
        }

        left_bef = matches.1;
        right_bef = matches.2;
    }

    centres
}

fn get_max_centre(centres: &Vec<(u64,u64,u64)>) -> (u64,u64,u64){
    let mut max_centre = 0;
    let mut max_centre_idx = 0;
    for (i, centre) in centres.iter().enumerate(){
        let current_centre = centre.2;
        if i == 0{
            max_centre = centre.2;
            max_centre_idx = 0;
        }

        if current_centre > max_centre{
            max_centre_idx = i;
        }
    }

    centres[max_centre_idx]
}

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

    // println!("{:?}", reflection_patterns);

    // let mut pattern_metadata_list: Vec<PatternMetadata> = vec![];

    // horizontal match (row match)
    let mut horizontal_matches: Vec<(u64, u64, u64, u64)> = vec![];
    for (pattern_num, pattern_vec) in reflection_patterns.iter().enumerate(){
        for (i, reference_pattern) in pattern_vec.iter().enumerate(){
            for (j, checked_pattern) in pattern_vec.iter().enumerate(){ 
                if reference_pattern == checked_pattern && i != j //&& i+1 == j
                {
                    // horizontal_matches.push(vec![]);
                    // let last_len = horizontal_matches.len() - 1;
                    let entry_to_be_pushed = (pattern_num as u64, (j+1) as u64, (i+1) as u64, (pattern_vec.len() as f64 / 2.0).ceil() as u64);
                    // println!("{} {}",(i+1) as u64, (j+1) as u64);
                    if horizontal_matches.iter().any(|x| x.1 == (j+1) as u64 && x.2 == (i+1) as u64) == false{
                        horizontal_matches.push((pattern_num as u64, (i+1) as u64, (j+1) as u64, (pattern_vec.len() as f64 / 2.0).ceil() as u64));
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    // println!("row_matches: {:?}", horizontal_matches);


    // vertical match (column match)
    let mut vertical_matches: Vec<(u64, u64, u64, u64)> = vec![];
    for (pattern_num, pattern_vec) in reflection_patterns.iter().enumerate(){
        // start comparison shift (second column to be compared)
        let mut current_shift = 1;
        while current_shift <= (pattern_vec[0].len() - 1){
            // start current column (first column to compare the others to)
            let mut current_col = 0;
            while current_col < (pattern_vec[0].len() - 1){ // as long as the first column has another to compare to
                let mut correct_pos = 0;
                // check every row
                for (i, reference_pattern) in pattern_vec.iter().enumerate(){
                    // check every column
                    let row_chars = reference_pattern.chars().collect::<Vec<char>>();
                    if row_chars[current_col] == row_chars[current_shift]{
                        correct_pos += 1;
                    }
                }
                if correct_pos == pattern_vec.len(){
                    vertical_matches.push((pattern_num as u64, (current_col+1) as u64, (current_shift+1) as u64, (pattern_vec[0].len() as f64 / 2.0).ceil() as u64));
                }
                current_col += 1;
                if current_col == current_shift{
                    break;
                }
            }

            current_shift += 1;
        }
        
    }

    // println!("column_matches: {:?}", vertical_matches);

    let mut column_sum = 0;
    let mut row_sum = 0;
    for pattern_num in 0..reflection_patterns.len(){
        println!("------------------STARTING ROW {}------------------", pattern_num);
        let mut row_matches = horizontal_matches.iter()
                                                .filter(|x| x.0 == pattern_num as u64)
                                                .map(|x| (x.0,x.1,x.2,x.3))
                                                .collect::<Vec<(u64,u64,u64,u64)>>();

        let mut column_matches = vertical_matches.iter()
                                                .filter(|x| x.0 == pattern_num as u64)
                                                .map(|x| (x.0,x.1,x.2,x.3))
                                                .rev()
                                                .collect::<Vec<(u64,u64,u64,u64)>>();
        // sort_vector(&mut row_matches);
        // sort_vector(&mut column_matches);

        println!("sorted row: {:?}", row_matches);
        println!("sorted column: {:?}", column_matches);

        let column_centres = find_centre(&column_matches);
        let row_centres = find_centre(&row_matches);
        println!("detected column centres are: {:?}", column_centres);
        println!("detected row centres are: {:?}", row_centres);
        
        let mut max_column_centre: (u64,u64,u64) = (0,0,0);
        let mut max_row_centre: (u64,u64,u64) = (0,0,0);

        if column_centres.len() != 0{
            max_column_centre = get_max_centre(&column_centres);
        }

        if row_centres.len() != 0{
            max_row_centre = get_max_centre(&row_centres);
        } 

        println!("detected max column centre are: {:?}", max_column_centre);
        println!("detected max row centre are: {:?}", max_row_centre);

        if max_column_centre.2 > max_row_centre.2{
            column_sum += max_column_centre.0;
        } else {
            row_sum += max_row_centre.0;
        }

        if max_column_centre == (0,0,0) && max_row_centre == (0,0,0){
            let right_most_column = column_matches
                                    .iter()
                                    .any(|x| x.1 == (reflection_patterns[pattern_num][0].len()-1) as u64 && x.2 == (reflection_patterns[pattern_num][0].len()) as u64);
            let bottom_most_row = row_matches
                                    .iter()
                                    .any(|x| x.1 == (reflection_patterns[pattern_num].len()-1) as u64 && x.2 == (reflection_patterns[pattern_num].len()) as u64);
            let left_most_row = column_matches
                                .iter()
                                .any(|x| x.1 == 1 as u64 && x.2 == 2 as u64);
            let upper_most_row = row_matches
                            .iter()
                            .any(|x| x.1 == 1 as u64 && x.2 == 2 as u64);
            
            if right_most_column{
                // println!("pattern {} has right most column", pattern_num);
                // println!("{:?}", reflection_patterns[pattern_num]);
                column_sum += (reflection_patterns[pattern_num][0].len()-1) as u64;
            }

            if bottom_most_row{
                // println!("pattern {} has bottom most column", pattern_num);
                // println!("{:?}", reflection_patterns[pattern_num]);
                row_sum += (reflection_patterns[pattern_num].len()-1) as u64;
            }

            if left_most_row{
                // println!("pattern {} has left most column", pattern_num);
            }

            if upper_most_row{
                // println!("pattern {} has upper most column", pattern_num);
            }

            if right_most_column == false
            && bottom_most_row == false
            && left_most_row == false
            && upper_most_row == false{
                println!("pattern {} has NONE", pattern_num);
                println!("{:?}", reflection_patterns[pattern_num]);
            }
        }

    }

    println!("total sum: {}",column_sum+row_sum*100)

}