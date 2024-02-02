use std::fs;
use std::collections::HashMap;

fn sort_vector(vector_to_be_sorted: &mut Vec<(usize,usize, String, usize)>){
    for i in 0..vector_to_be_sorted.len(){
        for j in 0..vector_to_be_sorted.len()-i-1{
            if vector_to_be_sorted[j].3 < vector_to_be_sorted[j+1].3{
                let aux = vector_to_be_sorted[j+1].clone();
                vector_to_be_sorted[j+1] = vector_to_be_sorted[j].clone();
                vector_to_be_sorted[j] = aux.clone();
            }
        }
    }
}

fn get_step_counter(current_head_counter: usize, 
                current_direction: String, 
                neighbour_direction: String) -> usize{
    if neighbour_direction == current_direction{
        return current_head_counter+1 // direction count
    } 
    0
}

fn get_neighbours(x: isize, y:isize,
                current_direction: String,
                current_step_direction: usize,
                matrix: &Vec<Vec<usize>>, 
                matrix_rows: usize, 
                matrix_cols: usize) -> Vec<(usize, usize, String, usize)>{
    let left = (0+x,-1+y,"left");
    let up = (-1+x,0+y,"up");
    let right = (0+x,1+y,"right");
    let bottom = (1+x,0+y,"bottom");
    let neighbours_raw = vec![left, up, right, bottom];
    neighbours_raw.iter()
                .filter(|node| (node.0 < matrix_rows as isize && node.0 >= 0) && (node.1 < matrix_cols as isize && node.1 >= 0))
                .map(|node| (node.0 as usize, node.1 as usize, node.2.to_string(), get_step_counter(current_step_direction, current_direction.clone(), node.2.to_string().clone())))
                .filter(|node| node.3 < 3)
                .collect::<Vec<(usize, usize, String, usize)>>()
}  

fn get_opposite_direction(direction_string: &str) -> String{
    if direction_string == "left"{
        return "right".to_string()
    } else if direction_string == "up"{
        return "bottom".to_string()
    } else if direction_string == "right"{
        return "left".to_string()
    } else {
        "up".to_string()
    }
}

fn pop_queue_node(current_queue: &mut Vec<(usize, usize, String, usize)>,
                current_node_head: &(usize, usize, String, usize)) -> (usize, usize, String, usize){
    let mut copy_current_node_head = current_node_head.clone();
    let available_next_node_pos = current_queue.iter().enumerate()
                                    .filter(|&(i, node)| (node.2 == copy_current_node_head.2 
                                            || (node.2 != copy_current_node_head.2))
                                            && (node.2 != get_opposite_direction(&copy_current_node_head.2)))
                                    .nth(0)
                                    .unwrap().0;
    let available_next_node = current_queue.remove(available_next_node_pos);
    copy_current_node_head.0 = available_next_node.0; // x
    copy_current_node_head.1 = available_next_node.1; // y
    copy_current_node_head.2 = available_next_node.2.clone(); // direction
    copy_current_node_head.3 = available_next_node.3; // direction count
    // if available_next_node.2 == current_node_head.2{
    //     copy_current_node_head.3 += 1; // direction count
    // } else{
    //     copy_current_node_head.3 = 0;
    // }
    // copy_current_node_head.4 += available_next_node.3; // heat count
    copy_current_node_head
}

fn main() {
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }
    println!("{:?}", file_lines);

    // initiate queues
    let mut visited_nodes: Vec<(usize, usize, String, usize)> = vec![]; // x, y, direction, direction_count
    let mut current_queue: Vec<(usize, usize, String, usize)> = vec![]; // x, y, direction, dist
    let mut dist_map: HashMap<(usize, usize, String, usize), i32> = HashMap::new();
    let mut dist_node_map: HashMap<(usize, usize, String, usize), (usize, usize, String, usize)> = HashMap::new();
    let heat_matrix = file_lines.iter()
                                .map(|x| x.chars().collect::<Vec<char>>().iter().map(|pos| pos.to_digit(10).unwrap() as usize).collect::<Vec<usize>>())
                                .collect::<Vec<Vec<usize>>>();
    // get starting nodes
    let target_node = heat_matrix[heat_matrix.len()-1][heat_matrix[heat_matrix.len()-1].len()-1].clone();
    let start_node = heat_matrix[0][0].clone();
    // println!("{:?}", heat_matrix);
    // println!("starting at {}", start_node);
    // println!("ending at {}", target_node);
    let mut current_node_head = (0, 0, "right".to_string(), 0);
    dist_map.insert(current_node_head.clone(),0);
    dist_node_map.insert(current_node_head.clone(),current_node_head.clone());
    loop{
        let child_neighbours = get_neighbours(current_node_head.0 as isize,
                                            current_node_head.1 as isize,
                                            current_node_head.2.clone(),
                                            current_node_head.3,
                                            &heat_matrix,
                                            heat_matrix.len(),
                                            heat_matrix[0].len());
        for child_child_node in child_neighbours{
            if !visited_nodes.iter().any(|node| *node == child_child_node){
                current_queue.push(child_child_node.clone());
                visited_nodes.push(child_child_node.clone());
                
                if dist_map.contains_key(&child_child_node){
                    if *dist_map.get(&child_child_node).unwrap() > heat_matrix[child_child_node.0][child_child_node.1] as i32{
                        dist_map.remove(&child_child_node.clone());
                        dist_node_map.remove(&child_child_node.clone());
                        dist_map.insert(child_child_node.clone(), heat_matrix[child_child_node.0][child_child_node.1] as i32);
                        dist_node_map.insert(child_child_node.clone(), current_node_head.clone());
                    }
                } else {
                    dist_map.insert(child_child_node.clone(),heat_matrix[child_child_node.0][child_child_node.1] as i32);
                    dist_node_map.insert(child_child_node.clone(), current_node_head.clone());
                }
            }
        }
        if current_queue.len() == 0{
            break;
        }
        // sort_vector(&mut current_queue);
        // current_node_head = choose_queue_node(&current_queue, &current_node_head);
        // visited_nodes.push((current_node_head.0, current_node_head.1, current_node_head.2.clone(), heat_matrix[current_node_head.0][current_node_head.1]));
        current_node_head = pop_queue_node(&mut current_queue,
                                            &current_node_head);
        println!("new head: {:?}", current_node_head);
        // println!("visited nodes: {:?}", visited_nodes);
        println!("queue nodes: {:?}", current_queue);
        // println!("node dist: {:?}", dist_map);
        // end condition
        // if (current_node_head.0, current_node_head.1) == (heat_matrix.len()-1, heat_matrix[heat_matrix.len()-1].len()-1){
        //     break;
        // }
        
    }
    println!("end nodes: {:?}",dist_node_map);
    for current_node_head in vec![(12, 12, "bottom".to_string(), 0),
                                (12, 12, "bottom".to_string(), 1),
                                (12, 12, "bottom".to_string(), 2),
                                (12, 12, "right".to_string(), 0),
                                (12, 12, "right".to_string(), 1),
                                (12, 12, "right".to_string(), 2),
    ]{
        let mut last_node_backtrack = dist_node_map.get(&current_node_head).unwrap().clone();
        let mut shortest_path_heat = heat_matrix[last_node_backtrack.0][last_node_backtrack.1];
        loop{
            if (last_node_backtrack.0, last_node_backtrack.1) == (0,0){
                break;
            }
            last_node_backtrack = dist_node_map.get(&last_node_backtrack).unwrap().clone();
            shortest_path_heat += heat_matrix[last_node_backtrack.0][last_node_backtrack.1];
            // println!("{:?}", last_node_backtrack);
        }
        println!("total heat sum {}", shortest_path_heat);
    }
    
}

/*
Add a way to sort queue by matrix heat while keeping the position at index 3 as the direction count rather than
the neighbours have it for heap and the node head for direction count
*/