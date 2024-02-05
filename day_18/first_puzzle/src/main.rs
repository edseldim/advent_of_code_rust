use std::fs;

#[derive(Debug)]
struct VecMetadata{
    direction: String,
    value: usize,
    color: String
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

fn main() {
    let mut file_lines: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        file_lines.push(line.to_string());
    }
    // println!("{:?}", file_lines);
    let vectors_metadata = file_lines.iter()
                                    .map(|str_line| str_line.split_whitespace().map(|data| data.to_string()).collect::<Vec<String>>())
                                    .map(|vector_str|
                                        VecMetadata{
                                            direction: vector_str[0].clone(),
                                            value: vector_str[1].parse().unwrap(),
                                            color: vector_str[2].clone()
                                        })
                                    .collect::<Vec<VecMetadata>>();
    println!("{:?}", vectors_metadata);
    let mut vectors: Vec<(isize, isize)> = vec![];
    for (i, metadata) in vectors_metadata.iter().enumerate(){
        let unit_vector = get_unit_vector_from_direction(&metadata.direction);
        let vector = (unit_vector.0*metadata.value as isize, unit_vector.1*metadata.value as isize);
        vectors.push(vector);
    }

    vectors.push((vectors[0].clone()));
    let mut area_in_poligon: isize = 0;
    for i in 1..vectors.len(){
            area_in_poligon += vectors[i].0 * vectors[i-1].1 - vectors[i-1].0 * vectors[i].1;
    }
    area_in_poligon = (area_in_poligon).abs()/2;
    let inner_points = area_in_poligon - ((vectors.len()/2) as isize) + 1;
    println!("{:?}", vectors);
    println!("inner points {}", inner_points);
    println!("total area {}", inner_points + vectors_metadata.iter().map(|vector| vector.value as isize).sum::<isize>());
}