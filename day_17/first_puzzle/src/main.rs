use std::fs;

fn get_step_counter(current_head_counter: usize, 
                current_direction: String, 
                neighbour_direction: String) -> usize{
    if neighbour_direction == current_direction{
        return current_head_counter+1 // direction count
    } 
    0
}

fn get_neighbours(current_node_head: &(isize, isize, String, isize, isize),
                matrix: &Vec<Vec<isize>>) -> Vec<(isize, isize, String, isize, isize)>{
    let left = (0+current_node_head.0,(-1+current_node_head.1 as isize),"left");
    let up = ((-1+current_node_head.0 as isize),0+current_node_head.1,"up");
    let right = (0+current_node_head.0,1+current_node_head.1,"right");
    let bottom = (1+current_node_head.0,0+current_node_head.1,"bottom");
    let neighbours_raw = vec![left, up, right, bottom];
    // return neighbours with correct step count relative to current state
    neighbours_raw.iter()
                .filter(|node| (node.0 < matrix.len() as isize && node.0 >= 0) && (node.1 < matrix[(matrix.len()-1) as usize].len() as isize && node.1 >= 0))
                .map(|node| 
                    (node.0, 
                    node.1, 
                    node.2.to_string(), 
                    get_step_counter(current_node_head.3 as usize, current_node_head.2.clone(), node.2.to_string().clone()) as isize,
                    current_node_head.4 + matrix[node.0 as usize][node.1 as usize] as isize))
                .filter(|node| node.3 < 3 && node.2 != get_opposite_direction(&current_node_head.2))
                .collect::<Vec<(isize, isize, String, isize, isize)>>()
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

fn main() {
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }
    println!("{:?}", file_lines);

    let heat_matrix = file_lines.iter()
                                .map(|x| x.chars().collect::<Vec<char>>().iter().map(|pos| pos.to_digit(10).unwrap() as isize).collect::<Vec<isize>>())
                                .collect::<Vec<Vec<isize>>>();

    // initiate queues
    let mut visited_nodes: Vec<(isize, isize, String, isize)> = vec![]; // x, y, direction, direction_count
    let mut current_queue: Vec<(isize, isize, String, isize, isize)> = vec![]; // x, y, direction, dist, heat sum

    // get starting nodes
    let mut current_node_head = (0, 0, "".to_string(), 0, 0);
    current_queue.push(current_node_head.clone());

    while current_queue.len() > 0{
        current_node_head = current_queue.remove(0);
        
        // end condition
        if (current_node_head.0 as usize, current_node_head.1 as usize) == (heat_matrix.len()-1, heat_matrix[heat_matrix.len()-1].len()-1){
            break;
        }
        
        if visited_nodes.iter().any(|node| *node == (current_node_head.0, current_node_head.1, current_node_head.2.clone(), current_node_head.3)){
            continue;
        }
        visited_nodes.push((current_node_head.0, current_node_head.1, current_node_head.2.clone(), current_node_head.3)); 

        let child_neighbours = get_neighbours(&current_node_head, &heat_matrix);
        for child_child_node in child_neighbours{
            current_queue.push(child_child_node.clone());
        }

        current_queue.sort_unstable_by_key(|node| node.4);
    }
    println!("min heat: {:?}",current_node_head.4);
}