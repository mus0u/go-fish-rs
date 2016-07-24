use std::cmp::Ordering;

#[derive(PartialEq,Eq,PartialOrd,Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(PartialEq,Eq,PartialOrd,Ord)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades
}

#[derive(Eq,Ord)]
pub struct Card {
    suit: Suit,
    rank: Rank
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

#[test]
fn cards_of_equal_rank_are_equal() {
    let four_of_spades = Card{ rank: Rank::Four, suit: Suit::Spades };
    let four_of_diamonds = Card{ rank: Rank::Four, suit: Suit::Diamonds };
    assert!(four_of_spades == four_of_diamonds);
}

#[test]
fn cards_can_be_compared_by_rank() {
    let three_of_hearts = Card{ rank: Rank::Three, suit: Suit::Hearts };
    let queen_of_clubs = Card{ rank: Rank::Queen, suit: Suit::Clubs };
    assert!(three_of_hearts < queen_of_clubs);
}
