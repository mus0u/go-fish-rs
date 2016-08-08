use std::cmp::Ordering;
use std::collections::HashSet;
use card::Rank::*;

#[derive(Copy,Clone,PartialEq,Eq,Hash,PartialOrd,Ord,Debug)]
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
impl Rank {
    // SO UGLY AAAAAAAAAAAAA
    pub fn all_ranks() -> HashSet<Rank> {
        let mut result = HashSet::with_capacity(13);
        result.insert(Two);
        result.insert(Three);
        result.insert(Four);
        result.insert(Five);
        result.insert(Six);
        result.insert(Seven);
        result.insert(Eight);
        result.insert(Nine);
        result.insert(Ten);
        result.insert(Jack);
        result.insert(Queen);
        result.insert(King);
        result.insert(Ace);
        result
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

#[derive(Copy,Clone,Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
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
