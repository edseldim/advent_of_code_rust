use std::fs;

fn sort_vector(vector_to_be_sorted: &mut Vec<(u64,u64,u64)>){
    for i in 0..vector_to_be_sorted.len(){
        for j in 0..vector_to_be_sorted.len()-i-1{
            if vector_to_be_sorted[j] < vector_to_be_sorted[j+1]{
                let aux = vector_to_be_sorted[j+1];
                vector_to_be_sorted[j+1] = vector_to_be_sorted[j];
                vector_to_be_sorted[j] = aux;
            }
        }
    }
}

fn calculate_reflection_propagation(centres: &Vec<(u64,u64,u64)>, matches: &Vec<(u64,u64,u64)>) -> Vec<(u64,u64,u64)>{
    let mut centre_depths = vec![];
    for centre in centres{
        let mut ref_top = centre.1;
        let mut ref_bottom = centre.2;
        let mut centre_depth = 0;
        // centreness is measured by amounts of 'layers' around the centre
        while matches.iter().any(|x| x.1 == ref_top-1 && x.2 == ref_bottom+1){
            centre_depth += 1;
            ref_top = ref_top-1;
            ref_bottom = ref_bottom+1;
        }
        centre_depths.push((centre.1, centre.2, centre_depth)) // first match second match reflection propagation
    }
    centre_depths
}

fn main() {
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

    // horizontal match (row match)
    let mut horizontal_matches: Vec<(u64, u64, u64)> = vec![];
    for (pattern_num, pattern_vec) in reflection_patterns.iter().enumerate(){
        for (i, reference_pattern) in pattern_vec.iter().enumerate(){
            for (j, checked_pattern) in pattern_vec.iter().enumerate(){ 
                if reference_pattern == checked_pattern && i != j //&& i+1 == j
                {
                    if horizontal_matches.iter().any(|x| x.1 == (j+1) as u64 && x.2 == (i+1) as u64) == false{
                        horizontal_matches.push((pattern_num as u64, (i+1) as u64, (j+1) as u64));
                    } else {
                        continue;
                    }
                }
            }
        }
    }


    // vertical match (column match)
    let mut vertical_matches: Vec<(u64, u64, u64)> = vec![];
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
                    vertical_matches.push((pattern_num as u64, (current_col+1) as u64, (current_shift+1) as u64));
                }
                current_col += 1;
                if current_col == current_shift{
                    break;
                }
            }

            current_shift += 1;
        }
        
    }

    let mut column_sum = 0;
    let mut row_sum = 0;
    for pattern_num in 0..reflection_patterns.len(){
        println!("------------------STARTING ROW {}------------------", pattern_num);
        println!("pattern dim: ({},{})",reflection_patterns[pattern_num].len(), reflection_patterns[pattern_num][0].len());
        let mut row_matches = horizontal_matches.iter()
                                                .filter(|x| x.0 == pattern_num as u64)
                                                .map(|x| (x.0,x.1,x.2))
                                                .collect::<Vec<(u64,u64,u64)>>();

        let mut column_matches = vertical_matches.iter()
                                                .filter(|x| x.0 == pattern_num as u64)
                                                .map(|x| (x.0,x.1,x.2))
                                                .rev()
                                                .collect::<Vec<(u64,u64,u64)>>();
        sort_vector(&mut row_matches);
        sort_vector(&mut column_matches);

        println!("sorted row: {:?}", row_matches);
        println!("sorted column: {:?}", column_matches);

        let column_centres = column_matches.iter()
                                            .filter(|x| x.1 == x.2 - 1)
                                            .map(|x| (x.0,x.1,x.2))
                                            .collect::<Vec<(u64,u64,u64)>>();
        let row_centres = row_matches
                            .iter()
                            .filter(|x| x.1 == x.2 - 1)
                            .map(|x| (x.0,x.1,x.2))
                            .collect::<Vec<(u64,u64,u64)>>();

        let column_centres = calculate_reflection_propagation(&column_centres, &column_matches);
        let row_centres = calculate_reflection_propagation(&row_centres, &row_matches);
        println!("calculated reflection for column centres: {:?}", column_centres);
        println!("calculated reflection for row centres: {:?}", row_centres);

        /*
        ORDER:
            1. CHECK EDGE REFLECTION
            2. CHECK COLUMNS
            3. CHECK ROWS
            4. OTHERWISE, THERE'S NONE
         */

        let left_most_column = column_matches
                .iter()
                .any(|x| x.1 == 1 as u64 && x.2 == 2 as u64);

        let right_most_column = column_matches
                .iter()
                .any(|x| x.1 == (reflection_patterns[pattern_num][0].len()-1) as u64 && x.2 == (reflection_patterns[pattern_num][0].len()) as u64);

        let bottom_most_row = row_matches
                .iter()
                .any(|x| x.1 == (reflection_patterns[pattern_num].len()-1) as u64 && x.2 == (reflection_patterns[pattern_num].len()) as u64);

        let upper_most_row = row_matches
                .iter()
                .any(|x| x.1 == 1 as u64 && x.2 == 2 as u64);

        if right_most_column{
            println!("[WARNING] pattern {} has right most column", pattern_num);
            column_sum += (reflection_patterns[pattern_num][0].len()-1) as u64;
            println!("Added {}",(reflection_patterns[pattern_num][0].len()-1) as u64);
            continue
        }

        if left_most_column{
            println!("[WARNING] pattern {} has left most column", pattern_num);
            // println!("{:?}", reflection_patterns[pattern_num]);
            continue
        }

        if bottom_most_row{
            println!("[WARNING] pattern {} has bottom most row", pattern_num);
            row_sum += (reflection_patterns[pattern_num].len()-1) as u64;
            println!("Added {}",(reflection_patterns[pattern_num].len()-1) as u64 * 100);
            continue
        }

        if upper_most_row{
            println!("[WARNING] pattern {} has upper most row", pattern_num);
            // println!("{:?}", reflection_patterns[pattern_num]);
            continue
        }

        let column_centre = match column_centres
                            .iter()
                            .filter(
                                |x| x.2 != 0
                                    && ((x.1 + x.2 >= reflection_patterns[pattern_num][0].len() as u64)
                                        || ((x.0 - x.2 - 1) <= 0)))
                            .nth(0){
                                Some(centre) => centre.clone(),
                                None => (0,0,0)
                            };
        
        let row_centre = match row_centres
                                    .iter()
                                    .filter(
                                        |x| x.2 != 0
                                            && ((x.1 + x.2 >= reflection_patterns[pattern_num].len() as u64)
                                                || ((x.0 - x.2 - 1) <= 0)))
                                    .nth(0){
                                        Some(centre) => centre.clone(),
                                        None => (0,0,0)
                                    };

        if column_centre.2 != 0{
            column_sum += column_centre.0;
            println!("Added {}",column_centre.0);
            continue;
        }

        if row_centre.2 != 0{
            row_sum += row_centre.0;
            println!("Added {}",row_centre.0*100);
            continue;
        }

        println!("pattern {} has NONE", pattern_num);
        println!("{:?}", reflection_patterns[pattern_num]);

    }

    println!("total sum: {}",column_sum+row_sum*100)

}