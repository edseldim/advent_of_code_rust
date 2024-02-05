use std::fs;
use regex::Regex;
#[derive(Debug)]
struct VecMetadata{
    direction: String,
    value: usize
}

fn get_direction_from_number(direction: usize) -> String{
    if direction == 0{
        return "R".to_string()
    } else if direction == 1{
        return "D".to_string()
    } else if direction == 2{
        return "L".to_string()
    } else {
        return "U".to_string()
    }
}

fn get_unit_vector_from_direction(direction: &String) -> (isize, isize){
    if direction == "R"{
        return (1,0)
    } else if direction == "L"{
        return (-1,0)
    } else if direction == "U"{
        return (0,1)
    } else {
        return (0,-1)
    }
}

// fn extract_vector_values(raw_str: &String) -> (isize, String){
//     let re = Regex::new(r"[0-9a-zA-Z]+").unwrap();
//     let hex_str = &re.find(raw_str).unwrap().as_str(); 
//     // let hex_chars_vector = hex_str.chars().map(|hex_value| hex_value.to_string()).collect::<Vec<String>>();
//     let vector_value = hex_str[0..hex_str.len()-1];
//     let direction_number = hex_str[hex_str.len()-1];
//     (isize::from_str_radix(vector_value,16), get_direction_from_number(&direction_number))
// }

fn main() {
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }
    
    let re = Regex::new(r"#[0-9a-zA-Z]+").unwrap();
    let vectors_metadata = file_lines.iter()
                                    .map(|str_line| str_line.split("#").nth(1).unwrap().chars().collect::<Vec<char>>())
                                    .map(|hex_str|
                                        VecMetadata{
                                            direction: get_direction_from_number(hex_str[hex_str.len()-1].to_string().parse().unwrap()),
                                            value: usize::from_str_radix(&hex_str[0..hex_str.len()-1].iter().collect::<String>(),16).unwrap() //vector_str[1].parse().unwrap(),
                                        })
                                    .collect::<Vec<VecMetadata>>();
    println!("{:?}", vectors_metadata);
    let mut vectors: Vec<(isize, isize)> = vec![];
    let mut vector = (0,0);
    for (i, metadata) in vectors_metadata.iter().enumerate(){
        let unit_vector = get_unit_vector_from_direction(&metadata.direction);
        vector = (unit_vector.0*metadata.value as isize+vector.0, unit_vector.1*metadata.value as isize+vector.1);
        vectors.push(vector);
    }

    vectors.push((vectors[0].clone()));
    let mut area_in_poligon: isize = 0;
    for i in 1..vectors.len(){
            area_in_poligon += vectors[i].0 * vectors[i-1].1 - vectors[i-1].0 * vectors[i].1;
    }
    area_in_poligon = (area_in_poligon).abs()/2;
    let border = vectors_metadata.iter().map(|vector| vector.value as isize).sum::<isize>();
    let inner_points = area_in_poligon + ((border/2) as isize) + 1;
    // println!("{:?}", vectors);
    println!("total dug area {}", inner_points);
}
