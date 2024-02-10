use std::fs;

#[derive(Debug)]
struct Hailstone{
    sx: i128,
    sy: i128,
    sz: i128,
    vx: i128,
    vy: i128,
    vz: i128,
    a: i128,
    b: i128,
    c: i128
}

impl Hailstone{
    fn new(sx: i128,
            sy: i128,
            sz: i128,
            vx: i128,
            vy: i128,
            vz: i128) -> Self{
        
        Hailstone{
            sx: sx,
            sy: sy,
            sz: sz,
            vx: vx,
            vy: vy,
            vz: vz,
            a: vy,
            b: -vx,
            c: vy * sx - vx * sy
        }
    }
}

fn main() {
    let file_path = "src/puzzle_data.txt";
    let mut hailstones: Vec<Hailstone> = vec![];
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let values = line.replace("@",",")
                            .split(",")
                            .map(|ch| ch.trim().parse::<i128>().unwrap())
                            .collect::<Vec<i128>>();
        hailstones.push(Hailstone::new(values[0],
                        values[1],
                        values[2],
                        values[3],
                        values[4],
                        values[5],
                        ))
        // grid.push(line_string.clone());
    }
    println!("{:?}", hailstones);
    let mut total = 0;
    for (i, hs1) in hailstones.iter().enumerate(){
        for hs2 in &hailstones[..i]{
            let (a1, b1, c1);
            let (a2, b2, c2);

            (a1, b1, c1) = (hs1.a, hs1.b, hs1.c);
            (a2, b2, c2) = (hs2.a, hs2.b, hs2.c);
            if a1 * b2 == b1 * a2{
                continue;
            }
            let x = (c1 * b2 - c2 * b1) as f64/ (a1 * b2 - a2 * b1) as f64;
            let y = (c2 * a1 - c1 * a2) as f64/ (a1 * b2 - a2 * b1) as f64;
            if 200000000000000.0 <= x && x <= 400000000000000.0 && 200000000000000.0 <= y && y <= 400000000000000.0{
                if vec![hs1, hs2].iter().all(|hs| (x - hs.sx as f64) * hs.vx as f64 >= 0.0 && (y - hs.sy as f64) * hs.vy as f64>= 0.0){
                    total += 1;
                }
            }
                
        }
    }

    println!("{}",total)
}
