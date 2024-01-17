use std::fs;

fn from_string_to_direction(direction_string: &str) -> (i32, i32){
    if direction_string == "left"{
        return (0,-1)
    } else if direction_string == "up"{
        return (-1, 0)
    } else if direction_string == "right"{
        return (0, 1)
    } else {
        (1, 0)
    }
}

fn from_direction_to_string(direction: &(i32, i32)) -> String{
    if *direction == (0,-1){
        return "left".to_string()
    } else if *direction == (-1, 0){
        return "up".to_string()
    } else if *direction == (0, 1){
        return "right".to_string()
    } else {
        "down".to_string()
    }
}

fn next_mov(current_pos: &(i32, i32), 
            direction: &(i32, i32), // left (0,-1) up (-1,0) right (0,1) down (-1,0) row as i32 x column
            matrix: &Vec<String>,
            traversed_positions: &mut Vec<(i32, i32)>,
            cache: &mut Vec<(i32, i32, i32, i32)>) -> (){ // (current pos x, current pos y, direction x, direction y)
    
    // let mut next_movement = (current_pos.0, current_pos.1);

    if !cache.iter().any(|x| *x == (current_pos.0, current_pos.1, direction.0, direction.1)){
        cache.push((current_pos.0, current_pos.1, direction.0, direction.1))
    } else {
        return ()
    }

    if current_pos.0 >= matrix.len() as i32 || current_pos.0 < 0 || current_pos.1 >= matrix[0].len() as i32 || current_pos.1 < 0{
        return ()
    }
    if !traversed_positions.iter().any(|x| x == current_pos){
        traversed_positions.push(*current_pos)
    }
    let next_refraction = matrix[current_pos.0 as usize].chars().collect::<Vec<char>>()[current_pos.1 as usize];
    if next_refraction == '/'{ // exchange positions
        if from_direction_to_string(direction) == "left"{
            let next_direction = &from_string_to_direction("bottom");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("bottom"), matrix, traversed_positions, cache);
        } else if from_direction_to_string(direction) == "up" {
            let next_direction = &from_string_to_direction("right");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("right"), matrix, traversed_positions, cache);
        } else if from_direction_to_string(direction) == "right" {
            let next_direction = &from_string_to_direction("up");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("up"), matrix, traversed_positions, cache);
        } else {
            let next_direction = &from_string_to_direction("left");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("left"), matrix, traversed_positions, cache);
        }
    } else if next_refraction == '\\'{ // exchange and multiply by -1
        if from_direction_to_string(direction) == "left"{
            let next_direction = &from_string_to_direction("up");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("up"), matrix, traversed_positions, cache);
        } else if from_direction_to_string(direction) == "up" {
            let next_direction = &from_string_to_direction("left");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("left"), matrix, traversed_positions, cache);
        } else if from_direction_to_string(direction) == "right" {
            let next_direction = &from_string_to_direction("bottom");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("bottom"), matrix, traversed_positions, cache);
        } else {
            let next_direction = &from_string_to_direction("right");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("right"), matrix, traversed_positions, cache);
        }
    } else if next_refraction == '|'{
        if from_direction_to_string(direction) == "left"{
            let next_direction = &from_string_to_direction("bottom");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("bottom"), matrix, traversed_positions, cache);
            let next_direction = &from_string_to_direction("up");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("up"), matrix, traversed_positions, cache);
        } else if from_direction_to_string(direction) == "up" {
            let next_direction = &from_string_to_direction("up");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, direction, matrix, traversed_positions, cache);
        } else if from_direction_to_string(direction) == "right" {
            let next_direction = &from_string_to_direction("bottom");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("bottom"), matrix, traversed_positions, cache);
            let next_direction = &from_string_to_direction("up");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("up"), matrix, traversed_positions, cache);
        } else {
            let next_direction = &from_string_to_direction("bottom");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, direction, matrix, traversed_positions, cache);
        }
    } else if next_refraction == '-'{
        if from_direction_to_string(direction) == "left"{
            let next_direction = &from_string_to_direction("left");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, direction, matrix, traversed_positions, cache);
        } else if from_direction_to_string(direction) == "up" {
            let next_direction = &from_string_to_direction("right");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("right"), matrix, traversed_positions, cache);
            let next_direction = &from_string_to_direction("left");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("left"), matrix, traversed_positions, cache);
        } else if from_direction_to_string(direction) == "right" {
            let next_direction = &from_string_to_direction("right");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, direction, matrix, traversed_positions, cache);
        } else {
            let next_direction = &from_string_to_direction("right");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("right"), matrix, traversed_positions, cache);
            let next_direction = &from_string_to_direction("left");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, &from_string_to_direction("left"), matrix, traversed_positions, cache);
        }
    } else { //. just keep moving
        if from_direction_to_string(direction) == "left"{
            let next_direction = &from_string_to_direction("left");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, direction, matrix, traversed_positions, cache);
        } else if from_direction_to_string(direction) == "up" {
            let next_direction = &from_string_to_direction("up");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, direction, matrix, traversed_positions, cache);
        } else if from_direction_to_string(direction) == "right" {
            let next_direction = &from_string_to_direction("right");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, direction, matrix, traversed_positions, cache);
        } else {
            let next_direction = &from_string_to_direction("bottom");
            let next_movement = (current_pos.0+next_direction.0, current_pos.1+next_direction.1);
            next_mov(&next_movement, direction, matrix, traversed_positions, cache);
        }
    }
}

fn main() {
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }
    println!("{:?}", file_lines);
    let mut energized_tiles_per_conf: Vec<usize> =  vec![];
    
    for column in 0..file_lines[0].len(){
        let mut traversed_positions: Vec<(i32, i32)> = vec![];
        let mut cache: Vec<(i32, i32, i32, i32)> = vec![];
        next_mov(
            &(0,column as i32),
            &from_string_to_direction("bottom"),
            &file_lines,
            &mut traversed_positions,
            &mut cache
        );
        // println!("{:?}",traversed_positions);
        // println!("{}",traversed_positions.len());
        energized_tiles_per_conf.push(traversed_positions.len());
    }

    for column in 0..file_lines[0].len(){
        let mut traversed_positions: Vec<(i32, i32)> = vec![];
        let mut cache: Vec<(i32, i32, i32, i32)> = vec![];
        next_mov(
            &((file_lines.len()-1) as i32,column as i32),
            &from_string_to_direction("up"),
            &file_lines,
            &mut traversed_positions,
            &mut cache
        );
        // println!("{:?}",traversed_positions);
        // println!("{}",traversed_positions.len());
        energized_tiles_per_conf.push(traversed_positions.len());
    }

    for row in 0..file_lines.len(){
        let mut traversed_positions: Vec<(i32, i32)> = vec![];
        let mut cache: Vec<(i32, i32, i32, i32)> = vec![];
        next_mov(
            &(row as i32,0),
            &from_string_to_direction("right"),
            &file_lines,
            &mut traversed_positions,
            &mut cache
        );
        // println!("{:?}",traversed_positions);
        // println!("{}",traversed_positions.len());
        energized_tiles_per_conf.push(traversed_positions.len());
    }

    for row in 0..file_lines.len(){
        let mut traversed_positions: Vec<(i32, i32)> = vec![];
        let mut cache: Vec<(i32, i32, i32, i32)> = vec![];
        next_mov(
            &(row as i32,(file_lines[0].len()-1) as i32),
            &from_string_to_direction("left"),
            &file_lines,
            &mut traversed_positions,
            &mut cache
        );
        // println!("{:?}",traversed_positions);
        // println!("{}",traversed_positions.len());
        energized_tiles_per_conf.push(traversed_positions.len());
    }

    println!("max energized tiles conf is {}", energized_tiles_per_conf.iter().max().unwrap())



}
