use std::collections::HashMap;
use card::*;
use deck::*;

pub struct Player {
    pub name: String,
    pub books: HashMap<Rank, Vec<Card>>,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: String::from(name),
            books: HashMap::new(),
            hand: vec![],
        }
    }

    pub fn print_hand(&self) {
        println!("{}, your hand is:", self.name);
        for card in self.hand.as_slice() {
            println!("{:?} of {:?}", card.rank, card.suit);
        }
    }

    pub fn print_books(&self) {
        println!("{} has {} completed books:", self.name, self.books.len());
        for book in self.books.keys() {
            println!("Book of {:?}s", book);
        }
    }

    pub fn draw_starting_hand(&mut self, deck: &mut Deck) {
        let dealt_cards = deck.multi_deal(9);
        if dealt_cards.is_some() {
            self.hand.append(dealt_cards.unwrap().as_mut())
        }
        self.sort_hand();
    }


    pub fn ask_for(&mut self, rank: Rank, deck: &mut Deck, opponent: &mut Player) {
        println!("{}, got any {:?}s?", opponent.name, rank);
        match opponent.take(rank, deck) {
            Some(mut cards) => {
                println!("{} had {} {:?}s.", opponent.name, cards.len(), rank);
                self.hand.append(cards.as_mut());
            }
            None => {
                println!("Nope! Go fish.");
                self.go_fish(deck);
            }
        }
        self.sort_hand();
    }

    pub fn take(&mut self, rank: Rank, deck: &mut Deck) -> Option<Vec<Card>> {
        let mut matches = vec![];
        let mut new_hand = vec![];
        for card in self.hand.clone() {
            if card.rank == rank {
                matches.push(card);
            } else {
                new_hand.push(card);
            }
        }
        // if we've emptied this player's hand, they get to draw a card immediately.
        if new_hand.len() == 0 {
            println!("Emptied {}'s hand. Taking a card from the deck.", self.name);
            self.go_fish(deck);
        }
        self.hand = new_hand;
        if matches.len() == 0 {
            None
        } else {
            Some(matches)
        }
    }

    pub fn go_fish(&mut self, deck: &mut Deck) {
        match deck.deal() {
            Some(new_card) => self.hand.push(new_card),
            None => println!("Deck is empty, {} drew nothing.", self.name),
        };
        self.sort_hand();
    }

    pub fn score_books(&mut self, deck: &mut Deck) -> Option<Vec<Rank>> {
        let mut scored_books = Vec::new();
        let mut rank_counts = HashMap::new();
        for card in self.hand.as_mut_slice() {
            let rank_count = rank_counts.entry(card.rank).or_insert(0);
            *rank_count += 1;
        }
        for (rank, count) in rank_counts.iter() {
            if *count == 4 {
                println!("{} scored the book of {:?}s!", self.name, rank);
                let book = self.take(*rank, deck).unwrap();
                self.books.insert(*rank, book);
                scored_books.push(*rank);
            }
        }
        if scored_books.len() == 0 {
            None
        } else {
            Some(scored_books)
        }
    }
    fn sort_hand(&mut self) {
        self.hand.sort_by_key(|card| card.rank);
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use card::*;
    use card::Rank::*;
    use card::Suit::*;
    use deck::*;
    use player::*;

    #[test]
    fn player_can_draw_a_starting_hand() {
        let mut deck = Deck::new();
        let mut player = Player::new("Karl".to_string());
        player.draw_starting_hand(&mut deck);
        assert_eq!(player.hand.len(), 9);
        assert_eq!(player.hand[8].rank, Ace);
        assert_eq!(player.hand[8].suit, Spades);
    }

    #[test]
    fn player_can_give_cards_of_a_given_rank_from_hand() {
        let mut deck = Deck::new();
        let mut player = Player::new("Karl".to_string());
        player.draw_starting_hand(&mut deck);
        let result = player.take(Ace, &mut deck).unwrap();
        assert_eq!(result[0].rank, Ace);
        assert_eq!(result[0].suit, Spades);
        assert_eq!(player.hand.len(), 8);
    }

    #[test]
    fn player_can_ask_opponent_for_rank() {
        let mut deck = Deck::new();
        let mut player = Player::new("Sagan".to_string());
        let mut opponent = Player::new("Marx".to_string());
        player.draw_starting_hand(&mut deck);
        opponent.draw_starting_hand(&mut deck);
        assert_eq!(opponent.hand[3].rank, Five);
        assert_eq!(opponent.hand[3].suit, Spades);
        player.ask_for(Five, &mut deck, &mut opponent);
        assert_eq!(player.hand[0].rank, Five);
        assert_eq!(player.hand[0].suit, Spades);
    }

    #[test]
    fn player_can_see_their_hand() {
        let mut deck = Deck::new();
        let mut player = Player::new("Carl".to_string());
        player.draw_starting_hand(&mut deck);
        player.print_hand();
    }

    #[test]
    fn player_can_see_their_books() {
        let book = vec![Card {
                            rank: Five,
                            suit: Diamonds,
                        },
                        Card {
                            rank: Five,
                            suit: Hearts,
                        },
                        Card {
                            rank: Five,
                            suit: Clubs,
                        },
                        Card {
                            rank: Five,
                            suit: Spades,
                        }];
        let mut books = HashMap::new();
        books.insert(Five, book);
        let player = Player {
            name: "Carl".to_string(),
            hand: vec![],
            books: books,
        };
        player.print_books();
    }

    #[test]
    fn player_can_score_a_book() {
        let mut deck = Deck::new();
        let mut player = Player {
            name: "Carl".to_string(),
            books: HashMap::new(),
            hand: vec![Card {
                           rank: Five,
                           suit: Diamonds,
                       },
                       Card {
                           rank: Five,
                           suit: Hearts,
                       },
                       Card {
                           rank: Five,
                           suit: Clubs,
                       },
                       Card {
                           rank: Five,
                           suit: Spades,
                       }],
        };
        player.score_books(&mut deck);
        assert!(player.books.contains_key(&Five));
        assert_eq!(0, player.hand.len());
    }
}
