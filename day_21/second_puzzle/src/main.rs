use std::fs;
use std::collections::HashSet;

fn fill(start_pos_x: usize, 
        start_pos_y: usize, 
        ss: usize,
        grid: &Vec<Vec<String>>) -> usize{
    
    let start_pos = (start_pos_x as isize, start_pos_y as isize);
    let mut answer: HashSet<(isize, isize)> = HashSet::new();
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut q: Vec<(isize, isize, isize)> = vec![(start_pos.0 as isize, start_pos.1 as isize, ss as isize)];
    seen.insert(start_pos.clone());

    while q.len() > 0{
        let (row, col, s);
        (row, col, s) = q.remove(0);

        if s%2==0{
            answer.insert((row, col));
        }

        if s == 0{
            continue;
        }

        let next_positions = vec![(row + 1, col), (row - 1, col), (row, col + 1), (row, col - 1)];
        for (next_row, next_col) in &next_positions{
            if *next_row < 0 
                || *next_row as usize>= grid.len()
                || *next_col < 0 
                || *next_col as usize >= grid[0].len()
                || grid[*next_row as usize][*next_col as usize] == "#"
                || seen.contains(&(*next_row, *next_col))
            {
                continue;
            }

            seen.insert((*next_row, *next_col));
            q.push((*next_row, *next_col, s-1));
        }
    }

    return answer.len()
}

fn main() {
    let file_path = "src/puzzle_data.txt";
    let mut lines: Vec<String> = vec![];
    let mut start_pos: (usize, usize) = (0,0);
    for (row_num, line) in fs::read_to_string(file_path).unwrap().lines().enumerate(){
        for (col_num, col) in line.chars().enumerate(){
            if col == 'S'{
                start_pos = (row_num as usize, col_num as usize);
            } 
        }
        lines.push(line.to_string());
    }

    println!("{:?}", start_pos);
    let sr = start_pos.0;
    let sc = start_pos.1;
    let mut grid = lines.iter()
                        .map(|row| row.chars().map(|ch| ch.to_string()).collect::<Vec<String>>())
                        .collect::<Vec<Vec<String>>>();
    println!("{}",fill(start_pos.0, start_pos.1, 64, &grid));
    let size = grid.len();
    let steps = 26501365;

    if grid.len() != grid[0].len(){
        return
    }

    if sc != (size as f64/2.0).floor() as usize || sr != (size as f64/2.0).floor() as usize{
        return
    }

    if steps%size != (size as f64/2.0).floor() as usize{
        return
    }

    println!("{}", (steps as f64/size as f64).floor());

    let grid_width = (steps as f64 / size as f64).floor() as usize - 1;

    let odd = ((grid_width as f64/ 2.0).floor() as usize * 2 + 1).pow(2);
    let even = (((grid_width + 1) as f64/ 2.0).floor() as usize * 2).pow(2);

    let odd_points = fill(sr, sc, size * 2 + 1, &grid);
    let even_points = fill(sr, sc, size * 2, &grid);

    let corner_t = fill(size - 1, sc, size - 1, &grid);
    let corner_r = fill(sr, 0, size - 1, &grid);
    let corner_b = fill(0, sc, size - 1, &grid);
    let corner_l = fill(sr, size - 1, size - 1, &grid);

    let small_tr = fill(size - 1, 0, (size as f64/ 2.0).floor() as usize - 1, &grid);
    let small_tl = fill(size - 1, size - 1, (size as f64/ 2.0).floor() as usize - 1, &grid);
    let small_br = fill(0, 0, (size as f64/ 2.0).floor() as usize - 1, &grid);
    let small_bl = fill(0, size - 1, (size as f64/ 2.0).floor() as usize - 1, &grid);

    let large_tr = fill(size - 1, 0, ((size as f64 * 3.0)/ 2.0).floor() as usize - 1, &grid);
    let large_tl = fill(size - 1, size - 1, ((size as f64 * 3.0)/ 2.0).floor() as usize - 1, &grid);
    let large_br = fill(0, 0, ((size as f64 * 3.0)/ 2.0).floor() as usize - 1, &grid);
    let large_bl = fill(0, size - 1, ((size as f64 * 3.0)/ 2.0).floor() as usize - 1, &grid);

    println!("{}",
        odd * odd_points +
        even * even_points +
        corner_t + corner_r + corner_b + corner_l +
        (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)  +
        grid_width * (large_tr + large_tl + large_br + large_bl) 
    )
}
