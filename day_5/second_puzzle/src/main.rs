use std::fs;
#[derive(Debug)]
struct MapRange {
    src_start: i64,
    dst_start: i64,
    rng: i64
}
#[derive(Debug)]
struct MapDesc{
    maps: Vec<MapRange>,
    src_map_name: String,
    dst_map_name: String,
}


fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };

    let src_seeds: Vec<i64> = file_lines[0]
                    .split(":")
                    .collect::<Vec<&str>>()[1]
                    .split(" ")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse().unwrap())
                    .collect();
    println!("source seeds: {:?}",src_seeds);

    /* load maps */
    let mut map_current_head = String::from("");
    let mut file_maps = vec![];
    for line in &file_lines {
        if line.contains("map") { /* set new head */
            println!("map detected! {}",line);
            let src_desc = line
                        .split(" ")
                        .collect::<Vec<&str>>()[0]
                        .split("-")
                        .collect::<Vec<_>>()[0];

            let dst_desc = line
                        .split(" ")
                        .collect::<Vec<&str>>()[0]
                        .split("-")
                        .collect::<Vec<_>>()[2];
            
            map_current_head = format!("{src_desc}-{dst_desc}");

            file_maps.push(MapDesc {
                maps: vec![],
                src_map_name: src_desc.to_string(),
                dst_map_name: dst_desc.to_string(),
            });

            
            continue;
        }

        if line.len() > 0 && map_current_head.len() > 0 { /* there's a head set */
            println!("processing... {}",map_current_head);
            let src_start: i64 = line
                            .split_whitespace()
                            .filter(|x| x.len() > 0 )
                            .collect::<Vec<&str>>()[1]
                            .parse()
                            .unwrap()
                            ;
            let dst_start: i64 = line
                            .split_whitespace()
                            .filter(|x| x.len() > 0)
                            .collect::<Vec<&str>>()[0]
                            .parse()
                            .unwrap()
                            ; 
            let rng: i64 = line
                            .split_whitespace()
                            .filter(|x| x.len() > 0)
                            .collect::<Vec<&str>>()[2]
                            .parse()
                            .unwrap()
                            ;
            let max_maps_stored = file_maps.len();
            file_maps[max_maps_stored-1].maps.push(MapRange {
                src_start: src_start,
                dst_start: dst_start,
                rng: rng 
            })
        }
    }

    println!("maps parsed: {:?}", file_maps);

    /* find locations */

    let mut src = 0;
    let mut seeds_path = vec![];
    let mut seeds_dst = vec![];
    for (i, start_rng_seed) in src_seeds.iter().enumerate() {
        if i % 2 == 0 {
            for mut src in start_rng_seed.clone()..start_rng_seed.clone()+src_seeds[i+1].clone()
            {
                // src = *rng_seed as i64;
                seeds_path.push(vec![src.clone()]);
                for map_info in &file_maps {
                    for range_info in &map_info.maps {
                        // src => 98
                        if (src >= range_info.src_start) & (src - range_info.src_start + 1 <= range_info.rng) { // 98 >= 98 AND (98-98+1) <= 2 => YES AND YES
                            // dest_shift => (98-98)
                            // 50 + 0 => 51
                            src = (src - range_info.src_start) + range_info.dst_start; // destination becomes src for the next map
                            break;
                        }
                    }
                    seeds_path[i].push(src.clone());
                }
                seeds_dst.push(src.clone());
            }
        }
    }

    println!("sources {:?} \ndestinations: {:?}",src_seeds, seeds_dst);
    println!("min location in destinations is: {}", seeds_dst.iter().min().unwrap())
    

    
}
