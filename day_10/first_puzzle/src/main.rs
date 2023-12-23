use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
struct PipeConnection{
    connections: Vec<String>,
    compatible_connections: Vec<String>
}


fn main() {
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }
    println!("{:?}", file_lines);
    /*
    The pipes are arranged in a two-dimensional grid of tiles:
        | is a vertical pipe connecting north and south.
        - is a horizontal pipe connecting east and west.
        L is a 90-degree bend connecting north and east.
        J is a 90-degree bend connecting north and west.
        7 is a 90-degree bend connecting south and west.
        F is a 90-degree bend connecting south and east.
        . is ground; there is no pipe in this tile.
        S is the starting position of the animal; there is a pipe on this tile, 
        but your sketch doesn't show what shape the pipe has.
     */

    let mut pipe_map: HashMap<String,PipeConnection> = HashMap::new();
    pipe_map.insert("|".to_string(),PipeConnection {
        connections: vec!["N".to_string(),"S".to_string()],
        compatible_connections: vec!["S".to_string(),"N".to_string()],
    });
    pipe_map.insert("-".to_string(),PipeConnection {
        connections: vec!["E".to_string(),"W".to_string()],
        compatible_connections: vec!["W".to_string(),"E".to_string()],
    });
    pipe_map.insert("L".to_string(),PipeConnection {
        connections: vec!["N".to_string(),"E".to_string()],
        compatible_connections: vec!["S".to_string(),"W".to_string()],
    });
    pipe_map.insert("J".to_string(),PipeConnection {
        connections: vec!["W".to_string(),"N".to_string()],
        compatible_connections: vec!["E".to_string(),"S".to_string()],
    });
    pipe_map.insert("7".to_string(),PipeConnection {
        connections: vec!["W".to_string(),"S".to_string()],
        compatible_connections: vec!["E".to_string(),"N".to_string()],
    });
    pipe_map.insert("F".to_string(),PipeConnection {
        connections: vec!["E".to_string(),"S".to_string()],
        compatible_connections: vec!["W".to_string(),"N".to_string()],
    });
    pipe_map.insert(".".to_string(),PipeConnection {
        connections: vec!["".to_string(),"".to_string()],
        compatible_connections: vec!["".to_string(),"".to_string()],
    });
    pipe_map.insert("S".to_string(),PipeConnection {
        connections: vec!["N".to_string(),"E".to_string(),"S".to_string(),"W".to_string()],
        compatible_connections: vec!["N".to_string(),"E".to_string(),"S".to_string(),"W".to_string()],
    });
    let mut visited_pipes: Vec<(i32, i32)> = vec![];
    for (row_num, row) in file_lines.iter().enumerate(){
        for (col_num, column) in row.chars().collect::<Vec<char>>().iter().enumerate(){
            if column.to_string() == "S"{
                println!("starting traversing node {:?} at ({},{})", column.to_string(), row_num, col_num);
                let mut base:(i32, i32) = (row_num as i32, col_num as i32);
                let mut base_node_conn = pipe_map[&column.to_string()].clone();
                visited_pipes.push(base.clone());
                let near_nodes = vec![(-1,0),(0,1),(1,0),(0,-1)]; /* top, right, bottom, left */
                let mut end_pipe = false;
                while end_pipe == false{
                    let mut is_in_path = false;
                    for (ref_row, ref_col) in &near_nodes{
                        if ((*ref_row+base.0) as usize) < file_lines.len() && ((*ref_col+base.1) as usize) < row.len(){
                            let pipe_under_check = file_lines[((*ref_row as i32)+base.0) as usize].chars().collect::<Vec<char>>()[((*ref_col as i32)+base.1) as usize].to_string();
                            let current_pipe_conn = pipe_map[&pipe_under_check].clone();
                            let is_already_visited = visited_pipes.iter().any(|x| x.0 == *ref_row+base.0 && x.1 == *ref_col+base.1);
                            let is_compatible = base_node_conn.compatible_connections.iter().any(|conn| current_pipe_conn.connections.contains(conn) && 
                            ((conn == "W" && *ref_row+base.0 == base.0 && *ref_col+base.1 > base.1) 
                                || (conn == "E" && *ref_row+base.0 == base.0 && *ref_col+base.1 < base.1)
                                || (conn == "N" && *ref_row+base.0 > base.0 && *ref_col+base.1 == base.1)
                                || (conn == "S" && *ref_row+base.0 < base.0 && *ref_col+base.1 == base.1)
                            ));
                            if is_already_visited == false && is_compatible == true{
                                println!("traversing to node {:?} at ({},{})",pipe_under_check,*ref_row+base.0,*ref_col+base.1);
                                    base = (base.0+(*ref_row as i32),base.1+(*ref_col as i32));
                                    base_node_conn = pipe_map[&file_lines[base.0 as usize].chars().collect::<Vec<char>>()[base.1 as usize].to_string()].clone();
                                    visited_pipes.push(base.clone());
                                    is_in_path = true;
                                    break;
                            }
                        }
                    }
                    if is_in_path == true{
                        continue;
                    }
                    end_pipe = true;
                }
            }
        }
    }
    let pipe_loop = &visited_pipes[1..];
    println!("the pipe is made up of the nodes: {:?}", pipe_loop);
    println!("farthest node from start requires {} steps", pipe_loop.len());

}
