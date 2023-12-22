use std::fs;

#[derive(Debug)]
#[derive(Clone)]
struct ReportSequence {
    sequences: Vec<Vec<i32>>
}


fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };

    /* Parse sequences */
    let mut environment_sequences: Vec<ReportSequence> = vec![];
    for report_sequence in &file_lines{
        let mut rs = ReportSequence{
                    sequences: vec![]
                };
        rs.sequences.push(report_sequence
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<i32>>());
        environment_sequences.push(rs);
    }

    println!("Detected {} sequence(s)", environment_sequences.len());

    /* Differentiate sequences */
    for report_sequence in &mut environment_sequences{
        let mut last_differentiated_sequence: Vec<i32> = report_sequence.sequences[0].clone(); 
        loop{
            let mut differentiated_sequence: Vec<i32> = vec![];
            let mut prev_number: i32 = 0;
            for (i, current_number) in last_differentiated_sequence.iter().enumerate(){
                println!("{}, {:?}", current_number, report_sequence);
                if i >= 1{
                    differentiated_sequence.push(dbg!((*current_number) - prev_number));
                }
                prev_number = (*current_number);
            }

            last_differentiated_sequence = differentiated_sequence.clone();
            report_sequence.sequences.push(differentiated_sequence.clone());

            if differentiated_sequence.iter().all(|x| *x == 0) == true{
                break
            }
        };
    }

    println!("Differentiated sequences: {:?}", environment_sequences);

    /* predict next value */

    for (i, report_sequence) in environment_sequences.iter_mut().enumerate(){
        let mut reversed_seq = report_sequence.sequences.iter_mut().rev().collect::<Vec<&mut Vec<i32>>>();

        // let mut prev_below_level_value: i32 = 0;
        for seq_num in 0..reversed_seq.len(){
            if seq_num >= 1{
                let prev_below_level_value: i32 = *reversed_seq[seq_num-1].iter().last().unwrap();
                let prev_same_level_value: i32 = reversed_seq[seq_num][reversed_seq[seq_num].len() - 1 as usize];
                let same_level_pred_value:i32 = prev_below_level_value + prev_same_level_value;
                reversed_seq[seq_num].push(same_level_pred_value);
            }
            
        }
    }
    println!("modified report sequences {:?}", environment_sequences);
    let pred_sum: i64 = environment_sequences.iter().map(|x| *x.sequences.first().unwrap().last().unwrap() as i64).sum();
    println!("sum of predictions is {}", pred_sum);
}
