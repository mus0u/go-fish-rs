extern crate rand;
#[macro_use] extern crate itertools;

use std::io::{self, Write};

mod card;
mod deck;
mod player;
mod game;

use player::*;

fn main() {
    println!("{}", game::DIVIDER);
    println!("{header:^width$}", header="Go Fish!", width=80);
    println!("{}", game::DIVIDER);
    let player_one = Player::new(get_player_name("1"));
    let player_two = Player::new(get_player_name("2"));
    game::run(player_one, player_two);
}

fn get_player_name(player_number: &str) -> String {
    print!("Player {}, please enter your name: ", player_number);
    let mut player_name = String::new();
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut player_name);
    player_name.trim().to_string()
}
