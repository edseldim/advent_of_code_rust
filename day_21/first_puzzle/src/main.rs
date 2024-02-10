use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "src/puzzle_data.txt";
    let mut lines: Vec<String> = vec![];
    let mut start_pos: (isize, isize) = (0,0);
    for (row_num, line) in fs::read_to_string(file_path).unwrap().lines().enumerate(){
        for (col_num, col) in line.chars().enumerate(){
            if col == 'S'{
                start_pos = (row_num as isize, col_num as isize);
            }
        }
        lines.push(line.to_string());
    }

    println!("{:?}", start_pos);
    let mut grid = lines.iter()
                        .map(|row| row.chars().map(|ch| ch.to_string()).collect::<Vec<String>>())
                        .collect::<Vec<Vec<String>>>();    
    let mut answer: HashSet<(isize, isize)> = HashSet::new();
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut q: Vec<(isize, isize, isize)> = vec![(start_pos.0, start_pos.1, 64)];
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
                || *next_row as usize>= lines.len()
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

    println!("{}", answer.len())
}
