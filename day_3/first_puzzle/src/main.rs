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
    height: i32
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
                height: height
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

    println!("{:?}",parsed_numbers);
    println!("{:?}",parsed_symbols);

    println!("Numbers to be added initial len {}", parsed_numbers.len());
    let mut unique_numbers = vec![];
    let mut total_parts_sum = 0;
    for symbol in parsed_symbols {
        for (i, number) in parsed_numbers.clone().iter().enumerate() {
            if !unique_numbers.contains(&number.id) {
                let symbol_pos = EuclideanPos {pos_x: symbol.end, pos_y: symbol.height};
                let number_pos = EuclideanPos {pos_x: number.end, pos_y: number.height};
                let number_pos_2 = EuclideanPos {pos_x: {if (number.end - number.len) < 0 {0} else {number.end - number.len + 1}}, pos_y: number.height};
                if symbol.is_adjacent(&number_pos) || symbol.is_adjacent(&number_pos_2){
                        println!("number {} (id: {}): ({},{}) is adjacent to symbol: {} ({},{})", number.number, number.id, number_pos.pos_x, number_pos.pos_y, symbol.symbol, symbol_pos.pos_x, symbol_pos.pos_y);
                        unique_numbers.push(number.id);
                        total_parts_sum += number.number.parse::<i64>().unwrap();
                }
            }
        }
    }

    println!("Numbers to be added non-repeated values len {}", unique_numbers.len());
    let mut excluded_numbers = vec![];
    for number in parsed_numbers.clone() {
        if !unique_numbers.contains(&number.id) {
            excluded_numbers.push(number);
        }
    }
    println!("Non-adjacent numbers: {:?}", excluded_numbers);

    println!("total sum is {}", total_parts_sum)
}
