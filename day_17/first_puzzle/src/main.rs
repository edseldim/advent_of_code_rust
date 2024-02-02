use std::fs;
use std::collections::HashMap;

fn direction_to_arrow(direction: String) -> String{
    if direction == "left"{
        return "<".to_string()
    } else if direction == "up"{
        return "^".to_string()
    } else if direction == "right"{
        return ">".to_string()
    } else {
        "v".to_string()
    }
}

fn print_heat_path(path: Vec<(usize, usize, String, usize)>, matrix: Vec<Vec<usize>>) -> String{
    let mut matrix_with_path = matrix.clone()
                                    .iter()
                                    .map(|row| row.iter().map(|col| col.to_string()).collect::<Vec<String>>())
                                    .collect::<Vec<Vec<String>>>();
    
    for node in path{
        matrix_with_path[node.0][node.1] = direction_to_arrow(node.2.clone());
    }
    
    let mut heat_map_2d = String::new();
    for row in matrix_with_path{
        let mut row_str = row.join("").clone();
        row_str.push_str("\n");
        heat_map_2d += &row_str;
    }
    heat_map_2d
}

fn sort_vector(vector_to_be_sorted: &mut Vec<(usize,usize, String, usize)>){
    // add change index 3 to heat
    for i in 0..vector_to_be_sorted.len(){
        for j in 0..vector_to_be_sorted.len()-i-1{
            if vector_to_be_sorted[j].3 < vector_to_be_sorted[j+1].3{
                let aux = vector_to_be_sorted[j+1].clone();
                vector_to_be_sorted[j+1] = vector_to_be_sorted[j].clone();
                vector_to_be_sorted[j] = aux.clone();
            }
        }
    }
    // add change it back to step count with sorted vector
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
    // return neighbours with correct step count relative to current state
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
    // find first node that meets requirements
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
                // update distance and backtrack map
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
        // after all nodes in queue have been visited, stop
        if current_queue.len() == 0{
            break;
        }
        // sort_vector(&mut current_queue);
        current_node_head = pop_queue_node(&mut current_queue,
                                            &current_node_head);
        // end condition
        // if (current_node_head.0, current_node_head.1) == (heat_matrix.len()-1, heat_matrix[heat_matrix.len()-1].len()-1){
        //     break;
        // }
        
    }
    println!("end nodes: {:?}",dist_node_map);
    // visit the 6 different ways the target node can be visited
    for current_node_head in vec![(heat_matrix.len()-1, heat_matrix[heat_matrix.len()-1].len()-1, "bottom".to_string(), 0),
                                (heat_matrix.len()-1, heat_matrix[heat_matrix.len()-1].len()-1, "bottom".to_string(), 1),
                                (heat_matrix.len()-1, heat_matrix[heat_matrix.len()-1].len()-1, "bottom".to_string(), 2),
                                (heat_matrix.len()-1, heat_matrix[heat_matrix.len()-1].len()-1, "right".to_string(), 0),
                                (heat_matrix.len()-1, heat_matrix[heat_matrix.len()-1].len()-1, "right".to_string(), 1),
                                (heat_matrix.len()-1, heat_matrix[heat_matrix.len()-1].len()-1, "right".to_string(), 2),
    ]{
        if dist_node_map.contains_key(&current_node_head){
            let mut last_node_backtrack = dist_node_map.get(&current_node_head).unwrap().clone();
            let mut shortest_path_heat = heat_matrix[last_node_backtrack.0][last_node_backtrack.1];
            let mut nodes_in_path: Vec<(usize, usize, String, usize)> = vec![];
            nodes_in_path.push(last_node_backtrack.clone());
            loop{
                if (last_node_backtrack.0, last_node_backtrack.1) == (0,0){
                    break;
                }
                last_node_backtrack = dist_node_map.get(&last_node_backtrack).unwrap().clone();
                nodes_in_path.push(last_node_backtrack.clone());
                shortest_path_heat += heat_matrix[last_node_backtrack.0][last_node_backtrack.1];
                // println!("{:?}", last_node_backtrack);
            }
            println!("total heat sum {}", shortest_path_heat);
            println!("{}", print_heat_path(nodes_in_path, heat_matrix.clone()))
        }
    }
    
}