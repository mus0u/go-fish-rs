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
    Ace,
}

pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

pub struct Card {
    suit: Suit,
    rank: Rank,
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

#[cfg(test)]
mod test {
    use card::*;
    use card::Rank::*;
    use card::Suit::*;
    #[test]
    fn cards_of_equal_rank_are_equal() {
        let four_of_spades = Card {
            rank: Four,
            suit: Spades,
        };
        let four_of_diamonds = Card {
            rank: Four,
            suit: Diamonds,
        };
        assert!(four_of_spades == four_of_diamonds);
    }

    #[test]
    fn cards_can_be_compared_by_rank() {
        let three_of_hearts = Card {
            rank: Three,
            suit: Hearts,
        };
        let queen_of_clubs = Card {
            rank: Queen,
            suit: Clubs,
        };
        assert!(three_of_hearts < queen_of_clubs);
    }
}
