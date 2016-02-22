extern crate rand;
use rand::Rng;

/// Deck of cards.
#[derive(Clone, Debug)]
pub struct Deck<C> {
    deck: Vec<C>,
    cur: usize,
}

impl<C> Deck<C> {
    pub fn new() -> Deck<C> {
        Deck {
            deck: Vec::new(),
            cur: 0,
        }
    }

    pub fn from_cards(cards: Vec<C>) -> Deck<C> {
        Deck {
            deck: cards,
            cur: 0,
        }
    }

    pub fn discarded(&self) -> usize {
        self.cur
    }

    pub fn left(&self) -> usize {
        self.deck.len() - self.cur
    }

    pub fn discard<F: FnMut(&C) -> bool>(&mut self, mut fun: F) -> usize {
        let mut count = 0;
        let mut n = self.deck.len();
        while n > self.cur {
            if (fun)(&self.deck[n - 1]) {
                count += 1;
                self.deck.swap(n, self.cur);
                self.cur += 1;
            } else {
                n -= 1;
            }
        }
        count
    }

    pub fn push(&mut self, card: C) {
        let last = self.deck.len();
        self.deck.push(card);
        self.deck.swap(self.cur, last);
    }

    pub fn pop(&mut self) -> Option<C> {
        if self.deck.is_empty() {
            None
        } else {
            let last = self.deck.len() - 1;
            self.deck.swap(self.cur, last);
            self.deck.pop()
        }
    }

    pub fn draw(&mut self) -> Option<&C> {
        let cur = self.cur;
        if cur < self.deck.len() {
            self.cur += 1;
            Some(&self.deck[cur])
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&C> {
        if self.deck.is_empty() {
            None
        } else {
            Some(&self.deck[self.cur])
        }
    }

    pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        rng.shuffle(&mut self.deck[..]);
        self.cur = 0;
    }

    pub fn draw_and_suffle<R: Rng>(&mut self, rng: &mut R) -> Option<&C> {
        let cur = self.cur;
        if self.deck.is_empty() {
            None
        } else if cur < self.deck.len() {
            self.cur += 1;
            Some(&self.deck[cur])
        } else {
            rng.shuffle(&mut self.deck[..]);
            self.cur = 1;
            Some(&self.deck[0])
        }
    }

    pub fn split(mut self) -> (Deck<C>, Deck<C>) {
        let other_deck = self.deck.split_off(self.cur);
        (self, Deck::from_cards(other_deck))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{SeedableRng, XorShiftRng};

    #[test]
    fn drawing_whole_deck_once_works() {
        let mut rng = XorShiftRng::from_seed([1, 1, 2, 3]);
        for _ in 0..1000 {
            let mut deck = Deck::from_cards(vec![1, 2, 3]);
            deck.shuffle(&mut rng);
            assert_eq!(0, deck.discarded());
            assert_eq!(3, deck.left());
            let mut cards = vec![];
            for n in 0..3 {
                cards.push(deck.draw().unwrap().clone());
                assert_eq!(n + 1, deck.discarded());
                assert_eq!(2 - n, deck.left());
            }
            for n in 1..4 {
                cards.contains(&n);
            }
            assert_eq!(None, deck.draw());
        }
    }

    #[test]
    fn discarding_works() {
        let mut deck = Deck::from_cards(vec![1, 2, 3, 4, 5, 6]);
        deck.discard(|n| *n < 3);
        assert_eq!(2, deck.discarded());
        assert_eq!(4, deck.left());
        let mut cards = vec![];
        while let Some(c) = deck.draw() {
            cards.push(c.clone());
        }
        for n in 3..7 {
            cards.contains(&n);
        }
    }
}
