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

fn check_edge(x: &(u64, u64,u64), 
            reflection_patterns:&Vec<Vec<String>>, 
            pattern_num: usize,
            reference_axis: i64) -> bool{

    let left_most_column = x.1 == 1 as u64 && x.2 == 2 as u64 && reference_axis == 1;

    let right_most_column = x.1 == (reflection_patterns[pattern_num][0].len()-1) as u64 && x.2 == (reflection_patterns[pattern_num][0].len()) as u64 && reference_axis == 1;

    let bottom_most_row = x.1 == (reflection_patterns[pattern_num].len()-1) as u64 && x.2 == (reflection_patterns[pattern_num].len()) as u64 && reference_axis == 0;

    let upper_most_row = x.1 == 1 as u64 && x.2 == 2 as u64 && reference_axis == 0;

    if right_most_column || left_most_column || bottom_most_row || upper_most_row{
        return true
    }
    return false
}

fn get_valid_centre(column_matches:&Vec<(u64,u64,u64)>, 
                    column_centres:&Vec<(u64,u64,u64)>,
                    row_matches:&Vec<(u64,u64,u64)>, 
                    row_centres:&Vec<(u64,u64,u64)>, 
                    reflection_patterns:&Vec<Vec<String>>, 
                    pattern_num: usize,
                    reference_axis: i16,
                    reference_center: &(u64,u64,u64,u64)) -> (u64,u64,u64,u64){
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

    if right_most_column && (reference_axis == 1 || reference_axis == -1){
        // println!("[WARNING] pattern {} has right most column", pattern_num);
        // column_sum += (reflection_patterns[pattern_num][0].len()-1) as u64;
        // println!("Added {}",(reflection_patterns[pattern_num][0].len()-1) as u64);
        return ((reflection_patterns[pattern_num][0].len()-1) as u64, reflection_patterns[pattern_num][0].len() as u64,0,1)
    }

    if left_most_column && (reference_axis == 1 || reference_axis == -1){
        // println!("[WARNING] pattern {} has left most column", pattern_num);
        // column_sum += 1;
        return (1,2,0,1)
    }

    if bottom_most_row && (reference_axis == 0 || reference_axis == -1){
        // println!("[WARNING] pattern {} has bottom most row", pattern_num);
        // row_sum += (reflection_patterns[pattern_num].len()-1) as u64;
        // println!("Added {}",(reflection_patterns[pattern_num].len()-1) as u64 * 100);
        return ((reflection_patterns[pattern_num].len()-1) as u64, reflection_patterns[pattern_num].len() as u64,0,0)
    }

    if upper_most_row && (reference_axis == 0 || reference_axis == -1){
        // println!("[WARNING] pattern {} has upper most row", pattern_num);
        // row_sum += 1;
        return (1,2,0,0)
    }

    let column_centre = match column_centres
                        .iter()
                        .filter(
                            |x| x.2 != 0
                                && ((x.1 + x.2 >= reflection_patterns[pattern_num][0].len() as u64)
                                    || ((x.0 - x.2 - 1) <= 0))
                                && (((reference_center.0,reference_center.1, reference_center.2) != (x.0,x.1,x.2) || (reference_axis as u64 != reference_center.3 && reference_axis != -1)) 
                                    && (reference_axis == 1 || reference_axis == -1)))
                        .nth(0){
                            Some(centre) => centre.clone(),
                            None => (0,0,0)
                        };
 
    let row_centre = match row_centres
                                .iter()
                                .filter(
                                    |x| x.2 != 0
                                        && ((x.1 + x.2 >= reflection_patterns[pattern_num].len() as u64)
                                            || ((x.0 - x.2 - 1) <= 0))
                                        && (((reference_center.0,reference_center.1, reference_center.2) != (x.0,x.1,x.2) || (reference_axis as u64 != reference_center.3 && reference_axis != -1)) 
                                        && (reference_axis == 0 || reference_axis == -1)))
                                .nth(0){
                                    Some(centre) => centre.clone(),
                                    None => (0,0,0)
                                };

    if column_centre.2 != 0 && (reference_axis == 1 || reference_axis == -1){
        // column_sum += column_centre.0;
        // println!("Added {}",column_centre.0);
        return (column_centre.0, column_centre.1, column_centre.2, 1);
    }

    if row_centre.2 != 0 && (reference_axis == 0 || reference_axis == -1){
        // row_sum += row_centre.0;
        // println!("Added {}",row_centre.0*100);
        return (row_centre.0, row_centre.1, row_centre.2, 0);
    }

    // println!("pattern {} has NONE", pattern_num);
    // println!("{:?}", reflection_patterns[pattern_num]);
    return (0,0,0,0)
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
    let mut horizontal_matches: Vec<(u64, u64, u64, u64)> = vec![];
    for (pattern_num, pattern_vec) in reflection_patterns.iter().enumerate(){
        for (i, reference_pattern) in pattern_vec.iter().enumerate(){
            for (j, checked_pattern) in pattern_vec.iter().enumerate(){ 
                if reference_pattern == checked_pattern && i != j //&& i+1 == j
                {
                    if horizontal_matches.iter().any(|x| x.1 == (j+1) as u64 && x.2 == (i+1) as u64) == false{
                        horizontal_matches.push((pattern_num as u64, (i+1) as u64, (j+1) as u64, reference_pattern.len() as u64));
                    } else {
                        continue;
                    }
                }

                let mut matched_positions = 0;
                for pattern_char in 0..reference_pattern.len(){
                    if reference_pattern.chars().collect::<Vec<char>>()[pattern_char] == checked_pattern.chars().collect::<Vec<char>>()[pattern_char]{
                        matched_positions += 1;
                    }
                }

                if matched_positions == reference_pattern.len()-1{
                    if horizontal_matches.iter().any(|x| x.1 == (j+1) as u64 && x.2 == (i+1) as u64) == false{
                        horizontal_matches.push((pattern_num as u64, (i+1) as u64, (j+1) as u64, (reference_pattern.len()-1) as u64));
                    }
                }
            }
        }
    }


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
                if correct_pos == pattern_vec.len() || correct_pos == pattern_vec.len()-1
                {
                    vertical_matches.push((pattern_num as u64, (current_col+1) as u64, (current_shift+1) as u64, correct_pos as u64));
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
                                                .filter(|x| x.0 == pattern_num as u64
                                                            && x.1 != x.2 - 1
                                                            && x.3 == reflection_patterns[pattern_num][0].len() as u64)
                                                .map(|x| (x.0,x.1,x.2))
                                                .collect::<Vec<(u64,u64,u64)>>();

        let mut column_matches = vertical_matches.iter()
                                                .filter(|x| x.0 == pattern_num as u64
                                                    && x.1 != x.2 - 1 
                                                    && x.3 == reflection_patterns[pattern_num].len() as u64)
                                                .map(|x| (x.0,x.1,x.2))
                                                .rev()
                                                .collect::<Vec<(u64,u64,u64)>>();
        let mut row_almost_matches = horizontal_matches.iter()
                                            .filter(|x| x.0 == pattern_num as u64
                                                        && x.3+1 == reflection_patterns[pattern_num][0].len() as u64)
                                            .map(|x| (x.0,x.1,x.2))
                                            .collect::<Vec<(u64,u64,u64)>>();

        let mut column_almost_matches = vertical_matches.iter()
                                            .filter(|x| x.0 == pattern_num as u64
                                                        && x.3+1 == reflection_patterns[pattern_num].len() as u64)
                                            .map(|x| (x.0,x.1,x.2))
                                            .rev()
                                            .collect::<Vec<(u64,u64,u64)>>();

        sort_vector(&mut row_matches);
        sort_vector(&mut column_matches);
        sort_vector(&mut row_almost_matches);
        sort_vector(&mut column_almost_matches);

        println!("sorted row: {:?}", row_matches);
        println!("sorted column: {:?}", column_matches);
        println!("sorted almost row matches: {:?}", row_almost_matches);
        println!("sorted almost column matches: {:?}", column_almost_matches);

        let mut row_centres_pre = horizontal_matches.iter()
                                                .filter(|x| x.0 == pattern_num as u64 && x.1 == x.2 - 1 && x.3 == reflection_patterns[pattern_num][0].len() as u64)
                                                .map(|x| (x.0,x.1,x.2))
                                                .collect::<Vec<(u64,u64,u64)>>();

        let mut column_centres_pre = vertical_matches.iter()
                                                .filter(|x| x.0 == pattern_num as u64 && x.1 == x.2 - 1 && x.3 == reflection_patterns[pattern_num].len() as u64)
                                                .map(|x| (x.0,x.1,x.2))
                                                .rev()
                                                .collect::<Vec<(u64,u64,u64)>>();

        println!("pre row centres: {:?}",row_centres_pre);
        println!("pre column centres: {:?}",column_centres_pre);

        let column_centres = calculate_reflection_propagation(&column_centres_pre, &column_matches);
        let row_centres = calculate_reflection_propagation(&row_centres_pre, &row_matches);
        println!("calculated reflection for column centres: {:?}", column_centres);
        println!("calculated reflection for row centres: {:?}", row_centres);
        let original_centre = get_valid_centre(&column_matches, 
                                                &column_centres,
                                                &row_matches, 
                                                &row_centres, 
                                                &reflection_patterns, 
                                                pattern_num,
                                                -1,
                                                &(0,0,0,0)); // wildcard
        
        println!("the part-1 centre is {:?}", original_centre);

        let mut new_centre = (0,0,0,0);
        for new_column_match in column_almost_matches{
            // println!("------Checking column {:?}------", new_column_match);
            if new_column_match.1 + 1 == new_column_match.2
                && !check_edge(&new_column_match, &reflection_patterns, pattern_num,1){
                let mut column_centre_mod = column_centres_pre.clone();
                column_centre_mod.push(new_column_match.clone());
                column_centre_mod = calculate_reflection_propagation(&column_centre_mod, &column_matches);
                // println!("calculated reflection for column centres: {:?}", column_centre_mod);
                new_centre = get_valid_centre(&column_matches, 
                                                &column_centre_mod,
                                                &row_matches, 
                                                &row_centres, 
                                                &reflection_patterns, 
                                                pattern_num,
                                                1,
                                                &original_centre)
            } else {
                let mut column_match_mod = column_matches.clone();
                column_match_mod.push(new_column_match.clone());
                // println!("new column matches: {:?}", column_match_mod);
                let column_centre_mod = calculate_reflection_propagation(&column_centres_pre, &column_match_mod);
                // println!("calculated reflection for column centres: {:?}", column_centre_mod);
                new_centre = get_valid_centre(&column_match_mod, 
                                            &column_centre_mod,
                                            &row_matches, 
                                            &row_centres, 
                                            &reflection_patterns, 
                                            pattern_num,
                                            1,
                                            &original_centre)
            }
            // println!("for column match {:?} the new centre is {:?}",new_column_match, new_centre);

            if new_centre != original_centre && new_centre != (0,0,0,0){
                break;
            }
        }

        if new_centre != original_centre && new_centre != (0,0,0,0){
            println!("found centre: {:?}", new_centre);
            if new_centre.3 == 1{
                column_sum += new_centre.0
            } else {
                row_sum += new_centre.0
            }
            continue
        }

        for new_row_match in row_almost_matches{
            
            // println!("------Checking row {:?}------", new_row_match);
            if new_row_match.1 + 1 == new_row_match.2
             && !check_edge(&new_row_match, &reflection_patterns, pattern_num,0){
                let mut row_centre_mod = row_centres_pre.clone();
                row_centre_mod.push(new_row_match.clone());
                // println!("new row centres: {:?}", row_centre_mod);
                row_centre_mod = calculate_reflection_propagation(&row_centre_mod, &row_matches);
                // println!("calculated reflection for row centres: {:?}", row_centre_mod);
                new_centre = get_valid_centre(&column_matches, 
                                                &column_centres,
                                                &row_matches, 
                                                &row_centre_mod,
                                                &reflection_patterns, 
                                                pattern_num,
                                                0,
                                                &original_centre)
            } else {
                let mut row_match_mod = row_matches.clone();
                row_match_mod.push(new_row_match.clone());
                // println!("new row matches: {:?}", row_match_mod);
                let row_centre_mod = calculate_reflection_propagation(&row_centres_pre, &row_match_mod);
                // println!("calculated reflection for row centres: {:?}", row_centre_mod);
                new_centre = get_valid_centre(&column_matches, 
                                            &column_centres,
                                            &row_match_mod, 
                                            &row_centre_mod, 
                                            &reflection_patterns, 
                                            pattern_num,
                                            0,
                                            &original_centre)
            }
            // println!("for row match {:?} the new centre is {:?}",new_row_match, new_centre);

            if new_centre != original_centre && new_centre != (0,0,0,0){
                break;
            }
        }

        if new_centre != original_centre && new_centre != (0,0,0,0){
            println!("found centre: {:?}", new_centre);
            if new_centre.3 == 1{
                column_sum += new_centre.0
            } else {
                row_sum += new_centre.0
            }
        } else  {
            println!("NOT FOUND!!");
            if original_centre.3 == 1{
                column_sum += original_centre.0
            } else {
                row_sum += original_centre.0
            }
        }
        continue
        

    }

    println!("total sum: {}",column_sum+row_sum*100)

}