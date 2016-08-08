use std::collections::HashSet;
use std::mem;
use std::io::{self};
use player::*;
use deck::*;
use card::*;
use card::Rank::*;

pub const DIVIDER: &'static str = "================================================================================";

pub fn run(mut active_player: Player, mut opponent: Player) {
    let mut deck = Deck::new();
    let mut completed_books: HashSet<Rank> = HashSet::with_capacity(13);
    let all_books = Rank::all_ranks();

    deck.shuffle();
    active_player.draw_starting_hand(&mut deck);
    opponent.draw_starting_hand(&mut deck);
    while !all_books.difference(&completed_books).collect::<Vec<&Rank>>().is_empty() {
        handle_turn(&mut active_player, &mut opponent, &mut deck, &mut completed_books);
        mem::swap(&mut active_player, &mut opponent);
    }
    if opponent.books.len() > active_player.books.len() {
        mem::swap(&mut active_player, &mut opponent);
    }
    display_final_score(active_player, opponent);
}

fn handle_turn(active_player: &mut Player, opponent: &mut Player, deck: &mut Deck, completed_books: &mut HashSet<Rank>) {
    println!("{}", DIVIDER);
    println!("{}'s turn!", active_player.name);
    active_player.print_books();
    println!("{}", DIVIDER);
    opponent.print_books();
    println!("{}", DIVIDER);
    active_player.print_hand();
    println!("{}", DIVIDER);
    let guess = handle_guess(&active_player.name);
    active_player.ask_for(guess, deck, opponent);
    if let Some(ranks) = active_player.score_books(deck) {
        for rank in ranks {
            completed_books.insert(rank);
        }
    };
}

fn handle_guess(player_name: &str) -> Rank {
    println!("{}, what will you ask your opponent for? (2-10,J,Q,K,A)?", player_name);
    let mut guess = String::new();
    let _ = io::stdin().read_line(&mut guess);
    match guess.trim() {
        "2" => Two,
        "3" => Three,
        "4" => Four,
        "5" => Five,
        "6" => Six,
        "7" => Seven,
        "8" => Eight,
        "9" => Nine,
        "10" => Ten,
        "J" => Jack,
        "Q" => Queen,
        "K" => King,
        "A" => Ace,
        _ => {
            println!("Sorry, I didn't understand your guess.");
            handle_guess(player_name)
        }
    }
}

fn display_final_score(active_player: Player, opponent: Player) {
    println!("{}", DIVIDER);
    println!("{} wins!", active_player.name);
    println!("{}", DIVIDER);
    active_player.print_books();
    opponent.print_books();
}
