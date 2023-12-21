use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Route{
    current_movement_state: String,
    start_step: String
}

fn follow_step(current_movement_state:&String, 
                current_movement: i32, 
                movement_map: &HashMap<String,u8>, // {L:0,R:1}
                letter_value_map: &HashMap<String, Vec<String>>, // {AAA: [BBB,CCC]}
                movements: &String) -> String{ // RLRL
    let next_movement = movement_map[&movements.chars().collect::<Vec<_>>()[current_movement as usize].to_string()] as usize;
    // println!("next_movement {}",next_movement);
    let next_movement_state = letter_value_map[current_movement_state][next_movement].to_string();
    next_movement_state
}

fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };

    /* parse movements info */
    let mut movements = file_lines[0].clone();
    let mut letter_value_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut start_steps: Vec<Route> = vec![];
    for option_line in &file_lines[2..]{
        // println!("{}",option_line);
        let movement_ref = option_line
                            .split("=")
                            .map(|x| x.clone().to_string())
                            .collect::<Vec<String>>()[0].clone();

        // println!("{}, last: {}",movement_ref.trim().to_string(),movement_ref.trim().to_string().chars().last().unwrap());
        if movement_ref.trim().to_string().chars().last().unwrap() == 'A'{
            start_steps.push(Route{
                start_step: movement_ref.trim().to_string().clone(),
                current_movement_state: movement_ref.trim().to_string().clone()
            })
        }

        let movement_options = option_line
                        .split("=")
                        .collect::<Vec<&str>>()[1]
                        .split_whitespace()
                        .map(|x| x.replace(|c: char| !c.is_alphanumeric(),""))
                        .collect::<Vec<String>>();
        
        letter_value_map.insert(movement_ref.trim().to_string(),movement_options.clone());

    }

    println!("{:?}", letter_value_map);
    println!("{:?}", start_steps);

    /* traverse the hashmap */
    let mut movement_map: HashMap<String,u8> = HashMap::new();
    movement_map.insert("L".to_string(),0);
    movement_map.insert("R".to_string(),1);
    // let mut current_movement_state = "AAA".to_string();
    let mut steps = 0;
    let mut current_movement_states: Vec<String> = vec![];
    let mut current_movement: i32 = 0;
    loop {
        current_movement_states = vec![];
        for route in &mut start_steps{
            let mut current_movement_state = follow_step(&route.current_movement_state,
                                                                                current_movement.clone(), 
                                                                                &movement_map, // shared
                                                                                &letter_value_map, // shared
                                                                                &movements); // shared
            route.current_movement_state = current_movement_state.clone();
            current_movement_states.push(current_movement_state.clone());
        }
        steps += 1;
        current_movement += 1;
        // println!("{:?}", current_movement_states);
        if current_movement_states.iter().all(|x| x.chars().last().unwrap() == 'Z') == true{
            break;
        }

        if current_movement as usize >= movements.len(){
            current_movement = 0;
        }
    }

    println!("Steps for reaching goal {}",steps);
}
