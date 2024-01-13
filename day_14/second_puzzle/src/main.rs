use std::fs;

fn print_dist(rolling_rocks: Vec<(usize, usize)>, 
            stopping_rocks: Vec<(usize, usize)>,
            raw_lines: Vec<String>) -> String{

    let mut processed_dist = String::from("");   
    for (i, line) in raw_lines.iter().enumerate(){
        let mut line_to_print: Vec<char> = vec![];
        for j in 0..line.len(){
            let next_char_in_line = match rolling_rocks.iter().filter(|x| x.0 == i && x.1 == j).nth(0){
                Some(x) => '0',
                None => match stopping_rocks.iter().filter(|x| x.0 == i && x.1 == j).nth(0) {
                    Some(y) => '#',
                    None => '.'
                }
            };
            line_to_print.push(next_char_in_line.clone());
        }
        line_to_print.push('\n');
        // println!("{:?}", line_to_print.iter().collect::<String>());
        processed_dist.push_str(&line_to_print.iter().collect::<String>().clone())
    }
    processed_dist
}

fn tilt_vertically(rolling_rocks: Vec<(usize, usize)>, 
                stopping_rocks: Vec<(usize, usize)>,
                upwards: bool,
                max_height: usize) -> Vec<(usize, usize)> {
    
    let mut tilt_direction = 1; // upwards
    let mut updated_rolling_rocks = rolling_rocks.clone();
    let mut reference_rolling_rocks = rolling_rocks.clone();
    if !upwards{
        tilt_direction = -1;
        updated_rolling_rocks = rolling_rocks.iter().rev().map(|x| (x.0,x.1)).collect::<Vec<(usize,usize)>>();
        reference_rolling_rocks = rolling_rocks.iter().rev().map(|x| (x.0,x.1)).collect::<Vec<(usize,usize)>>();
    }

    // println!("rocks {:?}", updated_rolling_rocks);
    // println!("reference rocks {:?}", reference_rolling_rocks);
    let mut prev_dist = updated_rolling_rocks.clone();
    loop{
        for (i, rock) in reference_rolling_rocks.iter().enumerate(){
            // println!("start #{}: {:?}",i,updated_rolling_rocks[i]);
            let mut new_column = rock.0;
            let mut pos_range = 1..rock.0+1;
            if !upwards{
                pos_range = 1..(max_height-new_column);
            }
            for pos in pos_range{
                if !stopping_rocks
                        .iter()
                        .any(|stopping_rock| stopping_rock.0 == (rock.0 as isize - (tilt_direction*pos as isize)) as usize && stopping_rock.1 == rock.1)
                    && !updated_rolling_rocks
                        .iter()
                        .any(|rolling_rock| rolling_rock.0 == (rock.0 as isize - (tilt_direction*pos as isize)) as usize && rolling_rock.1 == rock.1){
                    new_column = (rock.0 as isize - (tilt_direction*pos as isize)) as usize;
                    continue;
                }
                    break
            }
            updated_rolling_rocks[i] = (new_column, rock.1);
            // println!("end: {:?}",updated_rolling_rocks[i]);
        }

        if updated_rolling_rocks == prev_dist{
            break;
        }
        prev_dist = updated_rolling_rocks.clone();
        reference_rolling_rocks = updated_rolling_rocks.clone();
    }
    

    if !upwards{
        return updated_rolling_rocks.iter().rev().map(|x| (x.0,x.1)).collect::<Vec<(usize,usize)>>()
    }
    updated_rolling_rocks
}

fn tilt_horizontally(rolling_rocks: Vec<(usize, usize)>, 
                stopping_rocks: Vec<(usize, usize)>,
                leftwards: bool,
                max_height: usize) -> Vec<(usize, usize)> {
    
    let mut tilt_direction = 1; // leftwards
    let mut updated_rolling_rocks = rolling_rocks.clone();
    let mut reference_rolling_rocks = rolling_rocks.clone();
    if !leftwards{
        tilt_direction = -1;
        // updated_rolling_rocks = rolling_rocks.iter().rev().map(|x| (x.0,x.1)).collect::<Vec<(usize,usize)>>();
        // reference_rolling_rocks = rolling_rocks.iter().rev().map(|x| (x.0,x.1)).collect::<Vec<(usize,usize)>>();
    }

    // println!("rocks {:?}", updated_rolling_rocks);
    // println!("reference rocks {:?}", reference_rolling_rocks);
    let mut prev_dist = updated_rolling_rocks.clone();
    loop{
        for (i, rock) in reference_rolling_rocks.iter().enumerate(){
            // println!("start #{}: {:?}",i,updated_rolling_rocks[i]);
            let mut new_row = rock.1;
            let mut pos_range = 1..rock.1+1;
            if !leftwards{
                pos_range = 1..(max_height-new_row);
            }
            for pos in pos_range{
                if !stopping_rocks
                        .iter()
                        .any(|stopping_rock| stopping_rock.0 == rock.0 && stopping_rock.1 == (rock.1 as isize - (tilt_direction*pos as isize)) as usize)
                    && !updated_rolling_rocks
                        .iter()
                        .any(|rolling_rock| rolling_rock.0 == rock.0 && rolling_rock.1 == (rock.1 as isize - (tilt_direction*pos as isize)) as usize){
                    new_row = (rock.1 as isize - (tilt_direction*pos as isize)) as usize;
                    continue;
                }
                    break
            }
            updated_rolling_rocks[i] = (rock.0, new_row);
            // println!("end: {:?}",updated_rolling_rocks[i]);
        }

        if updated_rolling_rocks == prev_dist{
            break;
        }
        prev_dist = updated_rolling_rocks.clone();
        reference_rolling_rocks = updated_rolling_rocks.clone();
    }


    // if !leftwards{
    //     return updated_rolling_rocks.iter().rev().map(|x| (x.0,x.1)).collect::<Vec<(usize,usize)>>()
    // }
    updated_rolling_rocks
}


fn main() {
    
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }

    let mut rolling_rocks: Vec<(usize, usize)> = vec![];
    let mut stopping_rocks: Vec<(usize, usize)> = vec![];
    for (row, line) in file_lines.iter().enumerate(){
        let mut row_rolling_rocks = line.chars()
                                .enumerate()
                                .filter(|x| x.1.to_string() == "O")
                                .map(|x| (row, x.0))
                                .collect::<Vec<(usize, usize)>>();

        let mut row_stopping_rocks = line.chars()
                                .enumerate()
                                .filter(|x| x.1.to_string() == "#")
                                .map(|x| (row, x.0))
                                .collect::<Vec<(usize, usize)>>();

        rolling_rocks.append(&mut row_rolling_rocks);
        stopping_rocks.append(&mut row_stopping_rocks);
    }

    println!("initial rolling rocks {:?}", rolling_rocks);

    // let mut updated_rolling_rocks = tilt_vertically(rolling_rocks.clone(), stopping_rocks.clone(), true, file_lines.len());
    // println!("{}",print_dist(updated_rolling_rocks.clone(), stopping_rocks.clone(), file_lines.clone()));
    // updated_rolling_rocks = tilt_horizontally(updated_rolling_rocks.clone(), stopping_rocks.clone(), true, file_lines.len());
    // println!("{}",print_dist(updated_rolling_rocks.clone(), stopping_rocks.clone(), file_lines.clone()));
    // updated_rolling_rocks = tilt_vertically(updated_rolling_rocks.clone(), stopping_rocks.clone(), false, file_lines.len());
    // println!("{}",print_dist(updated_rolling_rocks.clone(), stopping_rocks.clone(), file_lines.clone()));
    // updated_rolling_rocks = tilt_horizontally(updated_rolling_rocks.clone(), stopping_rocks.clone(), false, file_lines.len());
    // println!("{}",print_dist(updated_rolling_rocks.clone(), stopping_rocks.clone(), file_lines.clone()));
        

    let mut updated_rolling_rocks = tilt_vertically(rolling_rocks.clone(), stopping_rocks.clone(), true, file_lines.len());
    let mut iter = 0;
    while iter < 1000000000{
        updated_rolling_rocks = tilt_horizontally(updated_rolling_rocks.clone(), stopping_rocks.clone(), true, file_lines.len());
        updated_rolling_rocks = tilt_vertically(updated_rolling_rocks.clone(), stopping_rocks.clone(), false, file_lines.len());
        updated_rolling_rocks = tilt_horizontally(updated_rolling_rocks.clone(), stopping_rocks.clone(), false, file_lines.len());
        // println!("{}",print_dist(updated_rolling_rocks.clone(), stopping_rocks.clone(), file_lines.clone()));
        println!("NORTH BEAN LOAD #{} {:?}",iter, updated_rolling_rocks.iter().map(|x| file_lines.len()-x.0).sum::<usize>());
        
        iter += 1;
        updated_rolling_rocks = tilt_vertically(updated_rolling_rocks.clone(), stopping_rocks.clone(), true, file_lines.len());
        // println!("FINISHED ITER #{} {:?}",iter, updated_rolling_rocks);
        // println!("FINISHED ITER #{}",iter);
        
    }
    // println!("{:?}", updated_rolling_rocks);
    // println!("{:?}", updated_rolling_rocks.iter().map(|x| file_lines.len()-x.0).sum::<usize>());

}