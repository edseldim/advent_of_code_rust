use std::fs;
use std::cmp;
use std::collections::{HashMap, HashSet};

fn overlaps(a: &Vec<usize>, b: &Vec<usize>) -> bool{
    //return max(a[0], b[0]) <= min(a[3], b[3]) and max(a[1], b[1]) <= min(a[4], b[4])
    return cmp::max(a[0], b[0]) <= cmp::min(a[0], b[0]) && cmp::max(a[1], b[1]) <= cmp::min(a[4], b[4])
}

fn main() {
    let file_path = "src/puzzle_data.txt";
    let mut bricks: Vec<Vec<usize>> = vec![];
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let line_numbers = line.replace("~",",")
                        .split(",")
                        .map(|number| number.parse().unwrap())
                        .collect::<Vec<usize>>();
        bricks.push(line_numbers.clone());
    }

    bricks.sort_by(|a, b| b[2].cmp(&a[2]));
    println!("{:?}", bricks);

    let mut bricks_mut = bricks.clone();
    for (idx, brick) in bricks_mut.iter_mut().enumerate(){
        let mut max_z = 1;
        for check in &bricks[..idx]{
            if overlaps(brick, check){
                max_z = cmp::max(max_z, check[5] + 1);
            }
        }

        brick[5] -= brick[2] - max_z;
        brick[2] = max_z;
    }

    bricks_mut.sort_by(|a, b| b[2].cmp(&a[2]));

    let mut k_supports_v: HashMap<usize,HashSet<usize>> = HashMap::new();
    let mut v_supports_k: HashMap<usize,HashSet<usize>> = HashMap::new();

    for i in 0..bricks.len(){
        k_supports_v.entry(i).or_insert(HashSet::new());
        v_supports_k.entry(i).or_insert(HashSet::new());
    }

    for (j, upper) in bricks_mut.iter().enumerate(){
        for (i, lower) in bricks_mut[..j].iter().enumerate(){
            if overlaps(lower, upper) && upper[2] == lower[5] + 1{
                k_supports_v.get_mut(&i).unwrap().insert(j);
                v_supports_k.get_mut(&j).unwrap().insert(i);
            }
        }
    }

    let mut total = 0;

    for i in 0..bricks.len(){
        
    }
}