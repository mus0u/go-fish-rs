use rand::{Rng, thread_rng};
use card::*;
use card::Rank::*;
use card::Suit::*;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let suits = [Diamonds, Clubs, Hearts, Spades];
        let ranks = [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace];
        let mut cards_vec: Vec<Card> = vec![];
        for (suit, rank) in iproduct!(suits.into_iter(), ranks.into_iter()) {
            cards_vec.push(Card {
                rank: *rank,
                suit: *suit,
            });
        }
        Deck { cards: cards_vec }
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn multi_deal(&mut self, number: u8) -> Option<Vec<Card>> {
        if number == 0 {
            return None
        }
        if let Some(first_deal) = self.deal() {
            let mut dealt_cards = vec![first_deal];
            for _ in 0..(number - 1) {
                if let Some(dealt_card) = self.deal() {
                    dealt_cards.push(dealt_card);
                } else {
                    break;
                }
            }
            Some(dealt_cards)
        } else {
            None
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(self.cards.as_mut_slice());
    }
}

#[cfg(test)]
mod test {
    use deck::*;
    use card::*;
    use card::Rank::*;
    use card::Suit::*;

    #[test]
    fn a_deck_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn dealing_from_an_unshuffled_deck() {
        let mut deck = Deck::new();
        let expected_card = Card {
            rank: Ace,
            suit: Spades,
        };
        assert_eq!(deck.deal().unwrap(), expected_card);
    }

    #[test]
    fn dealing_from_an_empty_deck() {
        let mut deck = Deck::new();
        for _ in 0..52 {
            deck.deal();
        }
        assert_eq!(deck.deal(), None);
    }

    #[test]
    fn multi_dealing_from_an_unshuffled_deck() {
        let mut deck = Deck::new();
        let dealt_cards = deck.multi_deal(3).unwrap();
        assert_eq!(dealt_cards[0].rank, Ace);
        assert_eq!(dealt_cards[0].suit, Spades);
        assert_eq!(dealt_cards[1].rank, King);
        assert_eq!(dealt_cards[1].suit, Spades);
        assert_eq!(dealt_cards[2].rank, Queen);
        assert_eq!(dealt_cards[2].suit, Spades);
    }

    #[test]
    fn multi_dealing_from_empty_deck_returns_none() {
        let mut deck = Deck::new();
        deck.multi_deal(52);
        assert_eq!(deck.multi_deal(10), None);
    }

    #[test]
    fn multi_dealing_from_short_deck_returns_only_remaining_cards() {
        let mut deck = Deck::new();
        deck.multi_deal(50);
        let dealt_cards = deck.multi_deal(10).unwrap();
        assert_eq!(dealt_cards.len(), 2);
    }

    #[test]
    fn shuffling_does_not_remove_cards_from_deck() {
        let mut deck = Deck::new();
        deck.shuffle();
        assert_eq!(deck.cards.len(), 52);
    }
}
