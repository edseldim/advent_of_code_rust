use std::fs;
use ndarray::Array;
use ndarray_stats::QuantileExt;

fn main() {
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }
    // println!("{:?}", file_lines);

    /* print unmodified galaxy */
    for row_number in 0..file_lines.len(){
        let mut row_to_print = vec![];
        for col_number in 0..file_lines[0].len(){
            row_to_print.push(file_lines[row_number].chars().collect::<Vec<char>>()[col_number]);
        }
        println!("{:?}", row_to_print);
    } 

    /* take into account space expansion */
    let mut rows_to_duplicate = vec![];
    let mut columns_to_duplicate = vec![];
    for (i, row) in file_lines.iter().enumerate(){
        if file_lines[i].chars().all(|x| x == '.'){
            rows_to_duplicate.push(i);
        }
    }

    for col_number in 0..file_lines[0].len(){
        let mut dots_count = 0;
        for row_number in 0..file_lines.len(){
            // println!("chk {}{} {}",row_number, col_number, file_lines[row_number].chars().collect::<Vec<char>>()[col_number]);
            if file_lines[row_number].chars().collect::<Vec<char>>()[col_number] == '.'{
                dots_count += 1;
            }
        }
        // println!("total {}", dots_count);
        if dots_count == file_lines.len(){
            columns_to_duplicate.push(col_number);
        }
    }

    // for row_number in rows_to_duplicate.iter().rev().collect::<Vec<&usize>>(){
    //     for repetition in 0..0{
    //         file_lines.insert(*row_number+1,file_lines[*row_number].clone());
    //     }
    // }

    // for (row_number, row) in file_lines.iter_mut().enumerate(){
    //     for col_number in columns_to_duplicate.iter().rev().collect::<Vec<&usize>>(){
    //         for repetition in 0..0{
    //             row.insert(col_number+1,'.');
    //         }
    //     }
    // }

    println!("rows to duplicate: {:?}", rows_to_duplicate);
    println!("columns to duplicate: {:?}", columns_to_duplicate);

    // /* print modified galaxy */
    // for row_number in 0..file_lines.len(){
    //     let mut row_to_print = vec![];
    //     for col_number in 0..file_lines[0].len(){
    //         row_to_print.push(file_lines[row_number].chars().collect::<Vec<char>>()[col_number]);
    //     }
    //     println!("{:?}", row_to_print);
    // } 


    /* find galaxies */
    let mut galaxies: Vec<(usize, usize,String)> = vec![];
    for (i, row) in file_lines.iter().enumerate(){
        for (j, col) in row.chars().enumerate(){
            if col != '.'{
                let columns_bef = columns_to_duplicate.iter().filter(|x| **x < j).collect::<Vec<&usize>>().len();
                let rows_bef = rows_to_duplicate.iter().filter(|x| **x < i).collect::<Vec<&usize>>().len();
                let corrected_row = ((i as i32 - (rows_bef as i32)) + (rows_bef as i32) * 1000000).abs();
                let corrected_col = ((j as i32 - (columns_bef as i32)) + (columns_bef as i32) * 1000000).abs();
                galaxies.push((corrected_row as usize,corrected_col as usize,col.to_string()));
            }
        }
    }

    /* get shortest distance */
    println!("The amount of galaxies is: {}", galaxies.len());
    println!("{:?}",galaxies);
    let mut optimal_path_steps: Vec<u64> = vec![];
    let mut processed_distances: Vec<(usize,usize,usize,usize)> = vec![];
    for galaxy in &galaxies{
        // println!("analizing galaxy {:?}", galaxy);
        let target_galaxies = galaxies.iter().filter(|galaxy_ref| galaxy_ref.0 != galaxy.0 || galaxy_ref.1 != galaxy.1).collect::<Vec<&(usize,usize,String)>>();
        // println!("calculating following distances: {:?}", target_galaxies);
        let valid_steps: Vec<(i32, i32)> = vec![(0,-1),(-1,0),(0,1),(1,0)]; /* left, up, right, bottom */
        for target_galaxy in &target_galaxies{
            println!("-------starting at {:?} (target: {:?})-------",galaxy, target_galaxy);
            if !processed_distances.iter().any(|processed_dist| processed_dist.0 == target_galaxy.0 &&
                                                                processed_dist.1 == target_galaxy.1 &&
                                                                processed_dist.2 == galaxy.0 &&
                                                                processed_dist.3 == galaxy.1){ /* checking whether another galaxy has calculated the dist to the current one */
                
                let optimal_steps = ((galaxy.0 as i32-target_galaxy.0 as i32).abs()+(galaxy.1 as i32 -target_galaxy.1 as i32).abs()) as u64;
                println!("-------it takes {} steps to get to target-------",optimal_steps);
                optimal_path_steps.push(optimal_steps);
                processed_distances.push((galaxy.0,galaxy.1,target_galaxy.0,target_galaxy.1));
            }
        //         let mut base = (galaxy.0, galaxy.1); /* starting point */
        //         processed_distances.push((base.0,base.1,target_galaxy.0,target_galaxy.1));
        //         let mut path_to_objective: Vec<(usize, usize)> = vec![];
        //         let mut duplicate_steps: u32 = 0;
        //         loop{ /* iterate until target is reached */
        //             let mut euclid_distances_in_iter: Vec<f32> = vec![];
        //             let mut valid_steps_from_base_in_iter: Vec<(i32, i32)> = vec![];
        //             /* iterate across valid steps from base to find the most optimal */
        //             for step in &valid_steps{
        //                 let mut base_ref = (step.0+(base.0 as i32), step.1+(base.1 as i32));
        //                 let euclid_distance = f32::sqrt(((base_ref.0-(target_galaxy.0 as i32)).pow(2) as f32)+ ((base_ref.1-(target_galaxy.1 as i32)).pow(2) as f32));
        //                 euclid_distances_in_iter.push(euclid_distance);
        //                 valid_steps_from_base_in_iter.push(base_ref.clone());
        //             }
        //             // println!("valid steps: {:?}", valid_steps_from_base_in_iter);
        //             // println!("valid steps distances: {:?}",euclid_distances_in_iter);
        //             let optimal_step = Array::from(euclid_distances_in_iter.clone()).argmin().unwrap();
        //             base = (valid_steps_from_base_in_iter[optimal_step].0 as usize, valid_steps_from_base_in_iter[optimal_step].1 as usize);
        //             // if rows_to_duplicate.contains(&base.0){
        //             //     duplicate_steps += 1;
        //             // } 
        //             // if columns_to_duplicate.contains(&base.1){
        //             //     duplicate_steps += 1;
        //             // }
        //             // println!("optimal step: {:?}",base);
        //             path_to_objective.push(base.clone());
        //             if base.0 == target_galaxy.0 && base.1 == target_galaxy.1{
        //                 println!("-------it takes {} steps to get to target-------",path_to_objective.len());
        //                 // duplicate_steps += 1;
        //                 // optimal_path_steps.push((((path_to_objective.len() as i32-duplicate_steps as i32) as u32)+(duplicate_steps*10)) as u64);
        //                 optimal_path_steps.push(path_to_objective.len() as u64);
        //                 break;
        //             }
        //         }
        //     }
        // }
        // println!("-----------------------------");
        // println!("Processed distances so far: {:?}", processed_distances);
        // println!("-----------------------------");
        }
    }
    println!("the sum of optimal steps is: {}", optimal_path_steps.iter().sum::<u64>());
}
