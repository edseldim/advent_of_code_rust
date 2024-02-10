use std::fs;
use std::collections::{HashMap, HashSet};
use std::cmp;

fn dfs(pt: &(isize, isize), 
        graph: &HashMap<(isize, isize), HashMap<(isize, isize), isize>>, 
        seen: &mut HashSet<(isize, isize)>, 
        start: &(isize, isize), 
        end: &(isize, isize)) -> isize{
    if pt == end{
        return 0;
    }

    let mut m = -100000000000000;
    seen.insert(pt.clone());
    for (nx, n) in graph.get(pt).unwrap().iter(){
        if !seen.contains(nx){
            m = cmp::max(m, dfs(nx, graph, seen, start, end) +  graph.get(pt).unwrap().get(nx).unwrap());
        }
    }
    seen.remove(&pt);
    return m;
}

fn main() {
        let file_path = "src/puzzle_data.txt";
        let mut grid: Vec<Vec<String>> = vec![];
        for line in fs::read_to_string(file_path).unwrap().lines(){
            let line_string = line.chars()
                                .map(|ch| ch.to_string())
                                .collect::<Vec<String>>();
            grid.push(line_string.clone());
        }

        let start_col = grid[0].iter().position(|col| col == ".").unwrap();
        let start_pos = (0 as isize, start_col as isize);
        let end_col = grid[grid.len()-1].iter().position(|col| col == ".").unwrap();
        let end_pos = ((grid.len()-1) as isize, end_col as isize);
        let mut points: Vec<(isize, isize)> = vec![start_pos, end_pos];
        println!("{:?}", grid);
        println!("{:?}", start_pos);
        println!("{:?}", end_pos);

        for (r, row) in grid.iter().enumerate(){
            for (c, ch) in row.iter().enumerate(){
                if ch == "#"{
                    continue;
                }

                let mut neighbors = 0;
                let next_positions = vec![(r as isize + 1, c as isize), (r as isize - 1, c as isize), (r as isize, c as isize + 1), (r as isize, c as isize - 1)];
                for (next_row, next_col) in &next_positions{
                    if *next_row >= 0 
                        && (*next_row as usize) < grid.len()
                        && *next_col >= 0 
                        && (*next_col as usize) < grid[0].len()
                        && grid[*next_row as usize][*next_col as usize] != "#"
                    {
                        neighbors += 1;
                    }
                }

                if neighbors >= 3{
                    points.push((r as isize, c as isize));
                }
            }
        }

        let mut graph: HashMap<(isize, isize), HashMap<(isize, isize), isize>> = HashMap::new();
        for p in &points{
            graph.entry(*p).or_insert(HashMap::new());
        }
        println!("{:?}",graph);
        let mut dirs: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        dirs.entry("^".to_string()).or_insert(vec![(-1,0)]);
        dirs.entry("v".to_string()).or_insert(vec![(1,0)]);
        dirs.entry("<".to_string()).or_insert(vec![(0,-1)]);
        dirs.entry(">".to_string()).or_insert(vec![(0,1)]);
        dirs.entry(".".to_string()).or_insert(vec![(-1, 0), (1, 0), (0, -1), (0, 1)]);

        for (sr, sc) in &points{
            let mut stack: Vec<(isize, isize, isize)> = vec![(0, *sr, *sc)];
            let mut seen: HashSet<(isize, isize)> = HashSet::new();
            seen.insert((*sr, *sc));

            while stack.len() > 0{
                let (n, r, c);
                (n, r, c) = stack.remove(0);
                if n != 0 && points.contains(&(r, c)){
                    *graph.get_mut(&(*sr, *sc)).unwrap().entry((r, c)).and_modify(|value| *value = n).or_insert(n);
                    continue;
                }

                for (dr, dc) in dirs.get(&grid[r as usize][c as usize]).unwrap(){
                    let nr = r + dr;
                    let nc = c + dc;
                    if nr >= 0 
                        && (nr as usize) < grid.len()
                        && nc >= 0 
                        && (nc as usize) < grid[0].len()
                        && grid[nr as usize][nc as usize] != "#"
                        && !seen.contains(&(nr, nc))
                    {
                        stack.push((n+1, nr, nc));
                        seen.insert((nr, nc));
                    }
                }
            }
        }

        let mut seen: HashSet<(isize, isize)> = HashSet::new();
        println!("{:?}", graph);
        println!("{}",dfs(&start_pos, &graph, &mut seen, &start_pos, &end_pos))
}
