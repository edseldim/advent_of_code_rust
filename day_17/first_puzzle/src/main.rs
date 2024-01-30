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

fn get_neighbours(x: isize, y:isize, 
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
                .map(|node| (node.0 as usize, node.1 as usize, node.2.to_string(), matrix[node.0 as usize][node.1 as usize]))
                .collect::<Vec<(usize, usize, String, usize)>>()
}

fn choose_queue_node(current_queue: &Vec<(usize, usize, String, usize)>,
                    current_node_head: &(usize, usize, String, usize, usize)) -> (usize, usize, String, usize, usize){
    
    let mut copy_current_node_head = current_node_head.clone();
    let available_next_node = current_queue.iter()
                                    .filter(|node| ((node.2 == copy_current_node_head.2 && copy_current_node_head.3 < 4) 
                                            || (node.2 != copy_current_node_head.2))
                                            && (node.2 != get_opposite_direction(&copy_current_node_head.2)))
                                    .nth(0)
                                    .unwrap();

    copy_current_node_head.0 = available_next_node.0;
    copy_current_node_head.1 = available_next_node.1;
    copy_current_node_head.2 = available_next_node.2.clone();
    if available_next_node.2 == current_node_head.2{
        copy_current_node_head.3 += 1;
    } else{
        copy_current_node_head.3 = 0;
    }
    copy_current_node_head.4 += available_next_node.3;
    copy_current_node_head
    
}

fn get_opposite_direction(direction_string: &str) -> String{
    if direction_string == "left"{
        return "right".to_string()
    } else if direction_string == "up"{
        return "down".to_string()
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

    // initiate queues
    let mut visited_nodes: Vec<(usize, usize, String, usize)> = vec![]; // x, y, direction, direction_count
    let mut current_queue: Vec<(usize, usize, String, usize)> = vec![]; // x, y, direction, dist
    let heat_matrix = file_lines.iter()
                                .map(|x| x.chars().collect::<Vec<char>>().iter().map(|pos| pos.to_digit(10).unwrap() as usize).collect::<Vec<usize>>())
                                .collect::<Vec<Vec<usize>>>();
    // get starting nodes
    let target_node = heat_matrix[heat_matrix.len()-1][heat_matrix[heat_matrix.len()-1].len()-1].clone();
    let start_node = heat_matrix[0][0].clone();
    // println!("{:?}", heat_matrix);
    // println!("starting at {}", start_node);
    // println!("ending at {}", target_node);
    let mut current_node_head = (0, 0, "right".to_string(), 1,start_node);
    loop{
        current_queue = vec![];
        let child_neighbours = get_neighbours(current_node_head.0 as isize,
                                            current_node_head.1 as isize,
                                            &heat_matrix,
                                            heat_matrix.len(),
                                            heat_matrix[0].len());
        for child_child_node in child_neighbours{
            if !visited_nodes.iter().any(|node| *node == child_child_node){
                current_queue.push(child_child_node.clone());
                // end condition
                if (child_child_node.0, child_child_node.1) == (heat_matrix.len()-1, heat_matrix[heat_matrix.len()-1].len()-1){
                    break;
                }
            }
        }
        sort_vector(&mut current_queue);
        current_node_head = choose_queue_node(&current_queue, &current_node_head);
        visited_nodes.push((current_node_head.0, current_node_head.1, current_node_head.2.clone(), heat_matrix[current_node_head.0][current_node_head.1]));
        println!("new head: {:?}", current_node_head);
        println!("visited nodes: {:?}", visited_nodes);
        
    }
    
}

/*
TODO:
1. CHOOSE THE NODE WITH THE LEAST HEAT THAT MATCHES THE DIRECTION COUNTER AND THE NON-RETURN (180 degrees) CONSTRAINT AS THE HEAD
2. ONLY ADD TO QUEUE THE HEAD'S NEIGHBOURS (AND CLEAN IT AFTER IT SWITCHES AT EVERY ITERATION). AFTER THE NEW HEAD IS FOUND, THE QUEUE SHOULD BE WIPED (UNTIL THE NEW HEAD'S NEIGHBOURS ARE CALCULATED USING 1.)
3. ADD A FINISH CONDITION WHEN TARGET IS FOUND
*/
