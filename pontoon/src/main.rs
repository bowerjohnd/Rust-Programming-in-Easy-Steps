use clearscreen::clear;
use std::io;
use std::io::Write;
use playing_cards::{Card, new_deck, print_cards, hit, pontoon::score_hand};

mod playing_cards;

fn input_string (prompt: &str) -> String {

    // loop until input is a valid string (not empty)
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("Failed to print prompt");

        let mut line = String::new();
        if let Err(e) = io::stdin().read_line(&mut line) {
            panic!("Error reading user input: {e}");
        }

        if ! line.is_empty() {
            return line.trim().to_string();
        }
    }
}

fn auto_play(deck: &mut Vec<Card>, hand: &mut Vec<Card>) -> bool{
    
    if score_hand(hand) < 18 {
        println!("Dealer gets {}", hit(deck, hand).to_string());
        true
    }
    else {
        false
    }
}

fn manual_play(deck: &mut Vec<Card>, hand: &mut Vec<Card>) -> bool{

    if input_string("Hit? (y/n): ")
        .to_uppercase()
        .starts_with("Y") {
            println!("Player gets {}", hit(deck, hand).to_string());
            true
        }
        else {
            false
        }
}

fn print_all_cards(dealer: &Vec<Card>, player: &Vec<Card>) {
    
    println!("Dealer's hand: ");
    print_cards(dealer);
    println!("Player's Hand: ");
    print_cards(player);
}

fn play_game(deck: &mut Vec<Card>,
    dealer: &mut Vec<Card>,
    player: &mut Vec<Card>) -> String {

        // deal two cards each
        hit(deck, player);
        hit(deck, dealer);
        hit(deck, player);
        hit(deck, dealer);

        // immediate win?
        if score_hand(dealer) == 21 {
            return "Dealer scores 21 - dealer wins".to_string();
        }
        if score_hand(player) == 21 {
            return "Player scores 21 - player wins".to_string();
        }

        // continue until neither player/dealer takes a card
        let mut player_hit = true;
        let mut dealer_hit = true;

        while player_hit || dealer_hit {

            dealer_hit = auto_play(deck, dealer);
            if score_hand(dealer) > 21 {
                return "Dealer Bust - player wins".to_string();
            }

            print_all_cards(dealer, player);

            if player_hit {
                player_hit = manual_play(deck, player);

                if score_hand(player) > 21 {
                    return "Player Bust - dealer wins".to_string();
                }
                if player.len() >= 5 {
                    return "Five card trick - player wins".to_string();
                }
            }
        }

        // game finished, calculate scores and determine winner

        let player_score = score_hand(player);
        let dealer_score = score_hand(dealer);

        if dealer_score > player_score {
            return "Dealer wins".to_string();
        }
        else if dealer_score < player_score {
            return "Player wins".to_string();
        }
        else {
            return "The game is a draw".to_string();
        }
        
    }

fn main() {

    let mut continue_playing: bool = true;
    
    let mut deck = new_deck();
    let mut player = new_deck();
    let mut dealer = new_deck();


    while continue_playing {
        clearscreen::clear().expect("Failed to clear screen");

        let result = play_game(&mut deck, &mut dealer, &mut player);

        println!("---");
        print_all_cards(&dealer, &player);
        println!("Dealer scores {}, Player scores {}. {}",
            score_hand(&dealer),
            score_hand(&player),
            result);
        println!("---");
        if input_string("Play again? (yes/no)")
        .to_uppercase()
        .starts_with("N") {
            println!("See you next game.");
            continue_playing = false;
        }
        else {
            dealer.clear();
            player.clear();
            println!("New Game.");
        }
    }
}
