use std::fs;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
#[derive(Debug)]
struct game_round {
    green: u8,
    red: u8,
    blue: u8
}

impl game_round {
    fn compare_max_and_assign(&mut self, other_game_round: &game_round) -> game_round {
        for cube_color in vec!["red","green","blue"] {
            if (self[cube_color] < other_game_round[cube_color]){
                self[cube_color] = other_game_round[cube_color];
            }
        }
        self.clone()
    }
}

impl Index<&'_ str> for game_round {
    type Output = u8;
    fn index(&self, s: &str) -> &u8 {
        match s {
            "red" => &self.red,
            "green" => &self.green,
            "blue" => &self.blue,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ str> for game_round {
    fn index_mut(&mut self, s: &str) -> &mut u8 {
        match s {
            "red" => &mut self.red,
            "green" => &mut self.green,
            "blue" => &mut self.blue,
            _ => panic!("unknown field: {}", s),
        }
    }
}

fn get_cubes_in_round(round_cubes: String) -> game_round {
    let mut cubes: Vec<&str> = round_cubes.split(",").collect();
    let mut round_cube_dist = game_round{green:0, red:0, blue:0};
    for mut cube in cubes {
        cube = cube.trim();
        let cube_color: &str = cube.split(" ").collect::<Vec<_>>()[1];
        let cube_amount: u8 = cube.split(" ").collect::<Vec<_>>()[0].parse().unwrap();
        round_cube_dist[cube_color] = cube_amount;
            
        }
    round_cube_dist
}

fn get_min_dist(game_round_str: String) -> game_round {
    let rounds: Vec<&str> = game_round_str.split(";").collect();
    let mut min_game_dist = game_round{green:0,blue:0,red:0};
    for (i, round) in rounds.iter().enumerate() {
        let round_cube_dist = get_cubes_in_round(round.to_string());
        if i == 0 {
            min_game_dist = round_cube_dist.clone();
        }

        min_game_dist.compare_max_and_assign(&round_cube_dist);

        
    }
    dbg!(min_game_dist)
}

fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };

    let rules = game_round {
        red: 12,
        green: 13,
        blue: 14
    };

    let mut valid_ids_sum: i32 = 0;
    for line in file_lines {
        let line_processed = line.split(":").collect::<Vec<_>>()[1].to_string();
        let min_dist =  get_min_dist(line_processed);
        valid_ids_sum += min_dist.red as i32 * min_dist.blue as i32 * min_dist.green as i32;

    };

    println!("Total amount of valid round is {}", valid_ids_sum);




}
