use std::fs;

#[derive(Debug)]
struct Card {
    card_id: String,
    winning_numbers: Vec<String>,
    potential_winner_numbers: Vec<String>,
    score: i64,
    winner_numbers: Vec<String>
}

fn process_numbers(card_numbers: String) -> Vec<Vec<String>> {
    let winning_numbers: Vec<String> = card_numbers.split("|")
                                        .map(|x| x.to_string())
                                        .collect::<Vec<String>>()[0]
                                        .split(" ")
                                        .filter(|x| x.trim().len() > 0)
                                        .map(|x| x.to_string()).collect();
    let card_numbers: Vec<String> = card_numbers.split("|")
                                        .map(|x| x.to_string())
                                        .collect::<Vec<String>>()[1]
                                        .split(" ")
                                        .filter(|x| x.trim().len() > 0)
                                        .map(|x| x.to_string()).collect();
    vec![winning_numbers,card_numbers]

}

fn get_card_score(mut potential_winner_numbers: Vec<String>, winning_numbers: Vec<String>) -> (i64, Vec<String>){
    let mut card_winner_numbers = vec![]; 
    let mut winner_numbers_amount = 0;
    for number in &mut potential_winner_numbers {
        if winning_numbers.contains(number) {
            winner_numbers_amount += 1;
            card_winner_numbers.push(number.to_string());
        }
    }
    (1*{if winner_numbers_amount > 0 {1} else {0}}*(2 as i64).pow({if winner_numbers_amount > 0 {winner_numbers_amount-1} else {0}}), card_winner_numbers)


}

fn process_card(line: String) -> Card{
    let splitted_line = line.clone();
    let card_id = splitted_line.split(":").collect::<Vec<_>>()[0];
    let card_numbers = splitted_line.split(":").collect::<Vec<_>>()[1];
    println!("Processing... {} {}", card_id, card_numbers);
    let mut processed_line = process_numbers(card_numbers.to_string());
    let winning_numbers = &processed_line[0];
    let card_numbers = &processed_line[1];
    let (card_score, winner_numbers) = get_card_score(card_numbers.clone(), winning_numbers.clone());
    println!("{} score is {}", card_id, card_score);
    Card {
        card_id: card_id.to_string().clone(),
        winning_numbers: winning_numbers.clone(),
        potential_winner_numbers: card_numbers.clone(),
        score: card_score.clone(),
        winner_numbers: winner_numbers.clone()
    }
}

fn main() {
    let file_path = "src/puzzle_data.txt";
    println!("In file {}", file_path);

    let mut file_lines = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        file_lines.push(line.to_string());
    };

    let mut total_card_score_sum: i64 = 0;
    for line in &file_lines {
        let processed_card = process_card(line.to_string());
        total_card_score_sum += processed_card.score;
        println!("Summary of card {:?}", processed_card);
    }
    println!("Total score sum {}", total_card_score_sum);
    
}
