use std::fs;
// use itertools::Itertools;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
struct NumberMetadata {
    number: String,
    end: i32,
    len: i32,
    height: i32,
    id: i32
}

#[derive(Debug)]
struct SymbolMetadata {
    symbol: String,
    end: i32,
    height: i32,
    adjacent_numbers: Vec<i32>
}

struct EuclideanPos {
    pos_x: i32,
    pos_y: i32
}

impl SymbolMetadata {
    fn is_adjacent(&self, another_point: &EuclideanPos) -> bool {
        let adjacent_pos = vec![
            EuclideanPos {pos_x: self.end - 1, pos_y: self.height - 1}, //
            EuclideanPos {pos_x: self.end, pos_y: self.height - 1},
            EuclideanPos {pos_x: self.end + 1, pos_y: self.height - 1},
            EuclideanPos {pos_x: self.end - 1, pos_y: self.height},
            EuclideanPos {pos_x: self.end, pos_y: self.height},
            EuclideanPos {pos_x: self.end + 1, pos_y: self.height},
            EuclideanPos {pos_x: self.end - 1, pos_y: self.height + 1},
            EuclideanPos {pos_x: self.end, pos_y: self.height + 1},
            EuclideanPos {pos_x: self.end + 1, pos_y: self.height + 1},
        ];

        for point in adjacent_pos {
            if (point.pos_x == another_point.pos_x) && (point.pos_y == another_point.pos_y) {
                return true;
            }
        }

        false

    }
}

fn parse_line_symbols(line: &mut String, height: i32) -> Vec<SymbolMetadata> {
    let line_processed = line.chars();
    let mut symbols_struct = vec![];
    for (i, letter) in line_processed.enumerate() {        
        const RADIX: u32 = 10;
        if !letter.is_digit(RADIX) && letter.to_string() != String::from(".") {
            symbols_struct.push(SymbolMetadata{
                symbol: letter.to_string(),
                end: i as i32,
                height: height,
                adjacent_numbers: vec![]
            });
        }
    }

    symbols_struct
}

fn parse_line_numbers(line: &mut String, height: i32) -> Vec<NumberMetadata> {
    let line_processed = line.chars();
    let mut number_buffer = String::from("");
    let mut numbers_struct = vec![];
    for (i, letter) in line_processed.enumerate() {
        const RADIX: u32 = 10;
        if letter.is_digit(RADIX) {
            number_buffer.push(letter);
            if i == line.len() - 1 {
                numbers_struct.push(NumberMetadata{
                    number: number_buffer.clone(),
                    end: (i) as i32,
                    len: number_buffer.clone().len() as i32,
                    height: height,
                    id: height * 1000 + (i) as i32
                });
                number_buffer = "".to_string();
            }
        } else {
            if number_buffer.len() > 0 {
                numbers_struct.push(NumberMetadata{
                    number: number_buffer.clone(),
                    end: (i-1) as i32,
                    len: number_buffer.clone().len() as i32,
                    height: height,
                    id: height * 1000 + (i-1) as i32
                });
                number_buffer = "".to_string();
            }
            continue;
        };
    }

    numbers_struct
}

fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };

    let mut parsed_numbers = vec![];
    let mut parsed_symbols = vec![];
    for (i, line) in file_lines.iter().enumerate() {
        let numbers_in_line = parse_line_numbers(&mut line.clone(), i as i32);
        let symbols_in_line = parse_line_symbols(&mut line.clone(), i as i32);
        for number_struct in numbers_in_line {
            parsed_numbers.push(number_struct);
        }

        for symbol_struct in symbols_in_line {
            parsed_symbols.push(symbol_struct);
        }
    }

    let mut unique_numbers = vec![];
    for symbol in &mut parsed_symbols {
        for (i, number) in parsed_numbers.clone().iter().enumerate() {
            if !unique_numbers.contains(&number.id) {
                let symbol_pos = EuclideanPos {pos_x: symbol.end, pos_y: symbol.height};
                let number_pos = EuclideanPos {pos_x: number.end, pos_y: number.height};
                let number_pos_2 = EuclideanPos {pos_x: {if (number.end - number.len) < 0 {0} else {number.end - number.len + 1}}, pos_y: number.height};
                if symbol.is_adjacent(&number_pos) || symbol.is_adjacent(&number_pos_2){
                        unique_numbers.push(number.id);
                        symbol.adjacent_numbers.push(number.number.parse::<i32>().unwrap());
                }
            }
        }
    }

    let mut adjacent_numbers_by_star = 1;
    let mut adjacent_numbers_by_other_sym = 0;
    let mut total_engine_ratio_sum = 0;
    for symbol in &parsed_symbols {
        if symbol.symbol == "*"{
            if symbol.adjacent_numbers.len() > 1 {
                for number in &symbol.adjacent_numbers {
                    adjacent_numbers_by_star *= number
                }
                println!("{:?} engine ratio is {}", symbol, adjacent_numbers_by_star);
                total_engine_ratio_sum += adjacent_numbers_by_star;
            }
        }
        adjacent_numbers_by_star = 1;
        
    }
    println!("total sum is {}", total_engine_ratio_sum)
}
