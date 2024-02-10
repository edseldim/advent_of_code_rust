use std::fs;
use std::cmp;
use std::collections::{HashMap, HashSet};

fn overlaps(a: &Vec<isize>, b: &Vec<isize>) -> bool{
    //return max(a[0], b[0]) <= min(a[3], b[3]) and max(a[1], b[1]) <= min(a[4], b[4])
    return cmp::max(a[0], b[0]) <= cmp::min(a[3], b[3]) && cmp::max(a[1], b[1]) <= cmp::min(a[4], b[4])
}

fn main() {
    let file_path = "src/puzzle_data.txt";
    let mut bricks: Vec<Vec<isize>> = vec![];
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let line_numbers = line.replace("~",",")
                        .split(",")
                        .map(|number| number.parse::<isize>().unwrap())
                        .collect::<Vec<isize>>();
        bricks.push(line_numbers.clone());
    }

    bricks.sort_by(|a, b| a[2].cmp(&b[2]));
    println!("{:?}", bricks);

    let mut bricks_mut = bricks.clone();
    let mut idx = 0;
    // for (idx, brick) in bricks_mut.iter_mut().enumerate(){
    while idx < bricks_mut.len(){
        let mut max_z = 1;
        let mut brick = &bricks_mut[idx];
        for check in &bricks_mut[..idx]{
            if overlaps(brick, check){
                max_z = cmp::max(max_z, check[5] + 1);
            }
        }
        
        bricks_mut[idx][5] -= bricks_mut[idx][2] - max_z;
        bricks_mut[idx][2] = max_z;
        idx += 1;
    }

    bricks_mut.sort_by(|a, b| a[2].cmp(&b[2]));
    println!("{:?}", bricks_mut);
    let mut k_supports_v: HashMap<usize,HashSet<usize>> = HashMap::new();
    let mut v_supports_k: HashMap<usize,HashSet<usize>> = HashMap::new();

    for i in 0..bricks_mut.len(){
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

    println!("k-v {:?}", k_supports_v);
    println!("v-k {:?}", v_supports_k);

    let mut total = 0;

    for i in 0..bricks_mut.len(){
        let mut q = k_supports_v.get(&i).unwrap().iter().filter(|j| v_supports_k.get(&j).unwrap().len() == 1)
                                                    .map(|number| *number)
                                                    .collect::<Vec<usize>>();
        let mut falling: HashSet<usize> = HashSet::new();
        for number in &q{
            falling.insert(*number);
        }
        falling.insert(i);

        while q.len() > 0{
            let j = q.remove(0);
            let copy_falling = falling.clone();
            for k in k_supports_v.get(&j).unwrap().difference(&copy_falling){
                if v_supports_k.get(&k).unwrap().is_subset(&copy_falling){
                    q.push(*k);
                    falling.insert(*k);
                }
            }
        }

        total += falling.len() as isize - 1

        
    }

    println!("{}", total);
}