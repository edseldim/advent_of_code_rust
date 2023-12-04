use std::fs;
use std::ops::{Index, IndexMut};

struct game_round {
    green: u8,
    red: u8,
    blue: u8
}

impl game_round {
    fn meet_rules(&self, rules: &game_round) -> bool {
        self.green <= rules.green && 
        self.red <= rules.red && 
        self.blue <= rules.blue
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
        cube = dbg!(cube.trim());
        let cube_color: &str = cube.split(" ").collect::<Vec<_>>()[1];
        let cube_amount: u8 = cube.split(" ").collect::<Vec<_>>()[0].parse().unwrap();
        round_cube_dist[cube_color] = cube_amount;
            
        }
    round_cube_dist
}

fn check_rules_for_round(game_round_str: String, rules: &game_round) -> bool {
    let rounds: Vec<&str> = game_round_str.split(";").collect();
    for round in rounds {
        let round_cube_dist = get_cubes_in_round(String::from(round));
        if !round_cube_dist.meet_rules(&rules) {
            return false
        }
    }
    true
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
        let game_id:u8 = line.split(":").collect::<Vec<_>>()[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap();
        let line_processed = line.split(":").collect::<Vec<_>>()[1].to_string();
        if check_rules_for_round(line_processed, &rules) {
            println!("line {} was valid!", line);
            valid_ids_sum += game_id as i32;
        }
    };

    println!("Total amount of valid round is {}", valid_ids_sum);




}
