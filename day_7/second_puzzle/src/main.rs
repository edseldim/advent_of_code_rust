use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
struct Hand {
    hand: String,
    hand_type: String,
    bid: i64,
    score: i64,
}

fn classify_hand_dist(hand_dist_raw: &Vec<(String, u8)>) -> String {

    let mut amt_jokers = 0;

    let mut has_joker = 0;

    if hand_dist_raw.iter().filter(|x| x.0 == "J").map(|x| x.1).collect::<Vec<_>>().len() > 0{
        amt_jokers = hand_dist_raw
                        .iter()
                        .filter(|x| x.0 == "J")
                        .map(|x| x.1)
                        .collect::<Vec<u8>>()[0] as u8;

        has_joker = 1;
    }

    let hand_letter_dist_numbers = hand_dist_raw
                                    .iter()
                                    .filter(|x| x.0 != "J")
                                    .map(|x| x.1)
                                    .collect::<Vec<u8>>();

    if (hand_letter_dist_numbers.len() as i32) == 1 || (hand_letter_dist_numbers.len() as i32) == 0{
        "five-o-kind".to_string()
    } else if (hand_letter_dist_numbers.len() as i32) == 2 {
        println!("hand dist: {:?} max amt: {} amt jokers: {} has jokers: {}",hand_dist_raw, *hand_letter_dist_numbers.iter().max().unwrap(), amt_jokers, has_joker);
        if *hand_letter_dist_numbers.iter().max().unwrap()+amt_jokers == 4 {
            "four-o-kind".to_string()
        } else {
            "full-house".to_string()
        }
    } else if (hand_letter_dist_numbers.len() as i32) == 3 {
        if *hand_letter_dist_numbers.iter().max().unwrap()+amt_jokers == 3 {
            "three-o-kind".to_string()
        } else {
            "two-pair".to_string()
        }
    } else if (hand_letter_dist_numbers.len() as i32) == 4 {
        "one-pair".to_string()
    } else {
        "high-card".to_string()
    }
}

fn parse_letter_dist(hand: &String) -> Vec<(String, u8)>{
    let mut letter_to_strength: Vec<(String, u8)> = vec![]; 
    for letter in hand.chars(){
        let mut was_found = false;
        for (i, letter_tuple) in letter_to_strength.iter_mut().enumerate(){
            if letter_tuple.0 == letter.to_string(){
                letter_tuple.1 += 1;
                was_found = true;
            }
        }

        if was_found == false {
            letter_to_strength.push((letter.to_string(),1));
        }
    }

    letter_to_strength
}

fn parse_hand_type(hand: &String) -> String{
    let hand_letter_dist = parse_letter_dist(hand);
    classify_hand_dist(&hand_letter_dist)
    
}

fn calculate_score_letters(hand: &String) -> i64{
    let mut letter_value_map: HashMap<String, i64> = HashMap::new();
    let cards = vec!["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J"];
    for (i, letter) in cards.iter().enumerate(){
        letter_value_map.insert(letter.to_string().clone(),(cards.len()-i) as i64);
    }
    let mut acc_score = 0;
    for (i,letter) in hand.chars().enumerate(){
        acc_score += (13 as i64).pow((4-i )as u32)*letter_value_map.get(&letter.clone().to_string()).unwrap();
    }
    acc_score
}

fn parse_hands(hand_raw: String) -> Hand {
    let hand = hand_raw
            .split_whitespace()
            .collect::<Vec<&str>>()[0]
            .to_string();
    
    let bid: i64 = hand_raw
            .split_whitespace()
            .collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

    let hand_type = parse_hand_type(&hand);

    let score = calculate_score_letters(&hand);

    Hand {
        hand: hand,
        hand_type: hand_type,
        bid: bid,
        score: score,
    }
    
}


fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };

    /* parse hands */
    let mut hands: Vec<Hand> = vec![];
    for line in file_lines {
        hands.push(parse_hands(line));
    }
    // println!("{:?}", hands);

    /* rank hands */
    // let mut sorted_hands: Vec<i64> = vec![];
    let mut same_type_ranks: Vec<Hand> = vec![];
    let mut max_rank = 0;
    let mut acc_score = 0;
    for hand_type in ["high-card", "one-pair","two-pair","three-o-kind","full-house","four-o-kind","five-o-kind"] {
        for hand in &hands{
            if hand.hand_type == hand_type {
                same_type_ranks.push(hand.clone());
            }
        }

        if same_type_ranks.len() == 0 {
            continue
        }
        let mut swapped = false;
        for i in 0..(same_type_ranks.len().clone()-1){
            swapped = false;
            for j in 0..(same_type_ranks.len()-i-1).clone(){
                if same_type_ranks[j].score > same_type_ranks[j+1].score {
                    let aux_right = same_type_ranks[j+1].clone();
                    let aux_left = same_type_ranks[j].clone();
                    same_type_ranks[j] = aux_right;
                    same_type_ranks[j+1] = aux_left;
                    swapped = true;
                }
            }

            if swapped == false {
                break;
            }
        }

        println!("sorted ranks {:?}", same_type_ranks);

        for (i, hand) in same_type_ranks.iter().enumerate(){
            println!("{:?} * {}",hand.bid,i+1+max_rank);
            acc_score += (i+1+max_rank) as i64*hand.bid;
        }
        max_rank += same_type_ranks.len();
        same_type_ranks = vec![];
    }

    println!("Winnings are: {}", acc_score);
}
