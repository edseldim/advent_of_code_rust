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
    // println!("{:?}", file_lines);
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
    /* traversing the loop */
    let mut loop_pipes: Vec<(i32, i32)> = vec![];
    for (row_num, row) in file_lines.iter().enumerate(){
        for (col_num, column) in row.chars().collect::<Vec<char>>().iter().enumerate(){
            if column.to_string() == "S"{
                // println!("starting traversing node {:?} at ({},{})", column.to_string(), row_num, col_num);
                let mut base:(i32, i32) = (row_num as i32, col_num as i32);
                let mut base_node_conn = pipe_map[&column.to_string()].clone();
                loop_pipes.push(base.clone());
                let near_nodes = vec![(-1,0),(0,1),(1,0),(0,-1)]; /* top, right, bottom, left */
                let mut end_pipe = false;
                while end_pipe == false{
                    let mut is_in_path = false;
                    for (ref_row, ref_col) in &near_nodes{
                        /* if reference node is in the grid */
                        let ref_base = (*ref_row+base.0,*ref_col+base.1);
                        if ((ref_base.0) as usize) < file_lines.len() && ((ref_base.1) as usize) < row.len(){
                            /* retrieve the pipe symbol */
                            let pipe_under_check = file_lines[((*ref_row as i32)+base.0) as usize].chars().collect::<Vec<char>>()[((*ref_col as i32)+base.1) as usize].to_string();
                            /* check its possible connections */
                            let current_pipe_conn = pipe_map[&pipe_under_check].clone();
                            /* check it hasn't already been visited */
                            let is_already_visited = loop_pipes.iter().any(|visited_pipe| visited_pipe.0 == ref_base.0 && visited_pipe.1 == ref_base.1);
                            /* check the ref pipe could connect to the base one */
                            let is_compatible = base_node_conn.compatible_connections.iter().any(|conn| current_pipe_conn.connections.contains(conn) && 
                                                                                                        /* for instance, 'F-' (- connects from west) */
                                                                                                        ((conn == "W" && ref_base.0 == base.0 && ref_base.1 > base.1)
                                                                                                        /* for instance, '-J' (- connects from east) */
                                                                                                        || (conn == "E" && ref_base.0 == base.0 && ref_base.1 < base.1)
                                                                                                        /* for instance, 'F' (| connects from north) 
                                                                                                                         '|'                           */
                                                                                                        || (conn == "N" && ref_base.0 > base.0 && ref_base.1 == base.1)
                                                                                                        /* for instance, '|' (| connects from south) 
                                                                                                                         'J'                           */
                                                                                                        || (conn == "S" && ref_base.0 < base.0 && ref_base.1 == base.1)
                                                                                                        ));
                            if is_already_visited == false && is_compatible == true{
                                // println!("traversing to node {:?} at ({},{})",pipe_under_check,ref_base.0,ref_base.1);
                                    base = (ref_base.0 as i32, ref_base.1 as i32);
                                    base_node_conn = pipe_map[&file_lines[base.0 as usize].chars().collect::<Vec<char>>()[base.1 as usize].to_string()].clone();
                                    loop_pipes.push(base.clone());
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
    let pipe_loop = &loop_pipes[1..];
    println!("the pipe is made up of the nodes: {:?}", pipe_loop);
    println!("farthest node from start requires {} steps", (pipe_loop.len()+(pipe_loop.len()%2))/2);

    /* Draw loop */
    for (row_num, row) in file_lines.iter().enumerate(){
        let mut row_to_draw: Vec<char> = vec![];
        for (col_num, column) in file_lines[row_num].chars().enumerate(){
            if loop_pipes.iter().any(|x| x.0==row_num as i32 && x.1==col_num as i32){
                row_to_draw.push(column);
                continue;
            }
            row_to_draw.push(' ');
        }
        println!("{:?}",row_to_draw.iter().collect::<String>());
    }

    /* checking points in loop 
    
    To find which points are part of a closed curve, one can use Jordan's curve Theorem. 
    
    To sum it all up, if one draws a line from the point under evaluation to the outside of the grid and
    counts how many times it touches the curve, then if it's odd, it's in otherwise it's outside*/

    let mut total_loop_points = 0;
    for (row_num, row) in file_lines.iter().enumerate(){
        for (col_num, column) in file_lines[row_num].chars().enumerate(){
            if loop_pipes.iter().any(|x| x.0==row_num as i32 && x.1==col_num as i32) == false{
                println!("------------------ for {} at ({},{})",column, row_num, col_num);
                println!("checking downwards");
                let mut loop_cross_count_down = 0;
                let mut loop_cross_count_up = 0;
                for ref_row in 1..(file_lines.len()-row_num){
                    let pipe_under_check = file_lines[ref_row+row_num].chars().collect::<Vec<char>>()[col_num].to_string();
                    println!("checking tile {} at ({},{})", pipe_under_check, (row_num+ref_row), col_num);
                    if loop_pipes.iter().any(|x| x.0==(row_num+ref_row) as i32 && x.1==(col_num as i32)){
                        println!("match down! at ({},{})", (row_num+ref_row), col_num);
                        loop_cross_count_down += 1;
                    }
                }
                println!("checking upwards");
                for ref_row in 1..row_num{
                    let pipe_under_check = file_lines[((row_num as i32)-(ref_row as i32)) as usize].chars().collect::<Vec<char>>()[col_num].to_string();
                    println!("checking tile {} at ({},{})", pipe_under_check, ((row_num as i32)-(ref_row as i32)), col_num);
                    if loop_pipes.iter().any(|x| x.0==((row_num as i32)-(ref_row as i32)) as i32 && x.1==(col_num as i32)){
                        println!("match up! at ({},{})", ((row_num as i32)-(ref_row as i32)), col_num);
                        loop_cross_count_up += 1;
                    }
                }
                if loop_cross_count_down%2 != 0 && loop_cross_count_up %2 != 0{
                    total_loop_points += 1;
                }
            }
        }
    }
    println!("the # of points in loop is {}", total_loop_points);


}
