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
    // println!("Processing... {} {}", card_id, card_numbers);
    let mut processed_line = process_numbers(card_numbers.to_string());
    let winning_numbers = &processed_line[0];
    let card_numbers = &processed_line[1];
    let (card_score, winner_numbers) = get_card_score(card_numbers.clone(), winning_numbers.clone());
    // println!("{} score is {}", card_id, card_score);
    Card {
        card_id: card_id.to_string().clone(),
        winning_numbers: winning_numbers.clone(),
        potential_winner_numbers: card_numbers.clone(),
        score: card_score.clone(),
        winner_numbers: winner_numbers.clone()
    }
}

fn update_deck(card_id: usize, amt_winner_numbers: usize, deck: &mut Vec<i32>) {
    /* if card not in deck, i.e, card #2 and deck(card_1:1) */
    if (card_id+1) > deck.len() {
        /* add original card in deck */
        deck.push(1);
    } else {
        /* if it is, update with the original card */
        deck[card_id] += 1
    }

    /* add n copy cards in current card to deck */
    for i in 1..=amt_winner_numbers {
        /* if copy card not in deck */
        if ((card_id+1)+i) > deck.len() {
            /* add copy card in deck */
            deck.push(1);
        } else {
            /* if it is, update the card amount in deck */
            deck[card_id+i] += deck[card_id]
        }
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
    let mut deck: Vec<i32> = vec![];
    for (i, line) in &mut file_lines.iter().enumerate() {
        let processed_card = process_card(line.to_string());
        total_card_score_sum += processed_card.score;
        println!("{}: Winners {:?}", processed_card.card_id.trim(), processed_card.winner_numbers);
        update_deck(i, processed_card.winner_numbers.len(), &mut deck);
        println!("Partial deck {:?}", deck);
    }
    println!("Total deck {:?}", deck);
    println!("Total deck length {}", deck.len());
    println!("Total amount of scratchcards {}", {let mut i = 0; for card_amount in deck {i+=card_amount;} i});
    println!("First puzzle score {}", total_card_score_sum)
    
    
}
