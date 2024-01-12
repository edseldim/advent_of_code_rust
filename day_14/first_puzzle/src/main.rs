use std::fs;

fn main() {
    
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }

    let mut rolling_rocks: Vec<(usize, usize)> = vec![];
    let mut stopping_rocks: Vec<(usize, usize)> = vec![];
    for (row, line) in file_lines.iter().enumerate(){
        let mut row_rolling_rocks = line.chars()
                                .enumerate()
                                .filter(|x| x.1.to_string() == "O")
                                .map(|x| (row, x.0))
                                .collect::<Vec<(usize, usize)>>();

        let mut row_stopping_rocks = line.chars()
                                .enumerate()
                                .filter(|x| x.1.to_string() == "#")
                                .map(|x| (row, x.0))
                                .collect::<Vec<(usize, usize)>>();
        // println!("{:?}", row_rolling_rocks);
        rolling_rocks.append(&mut row_rolling_rocks);
        stopping_rocks.append(&mut row_stopping_rocks);
    }

    // println!("{:?}", rolling_rocks);
    // println!("{:?}", stopping_rocks);

    let mut updated_rolling_rocks = rolling_rocks.clone();
    for (i, rock) in rolling_rocks.iter().enumerate(){
        let mut new_column = rock.0;
        for pos in 1..=rock.0{
            if !stopping_rocks
                    .iter()
                    .any(|stopping_rock| stopping_rock.0 == (rock.0 - pos) && stopping_rock.1 == rock.1)
                && !updated_rolling_rocks
                    .iter()
                    .any(|rolling_rock| rolling_rock.0 == (rock.0 - pos) && rolling_rock.1 == rock.1){
                new_column = rock.0 - pos;
                continue;
            }
                break
        }
        updated_rolling_rocks[i] = (new_column, rock.1);
    }
    println!("{:?}", updated_rolling_rocks);
    println!("{:?}", updated_rolling_rocks.iter().map(|x| file_lines.len()-x.0).sum::<usize>());

}