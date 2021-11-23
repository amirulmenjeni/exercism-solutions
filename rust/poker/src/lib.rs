use std::{collections::{HashMap}, cmp::Ordering};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
enum PokerHand {
    StraightFlush   = 8,
    FourOfAKind     = 7,
    FullHouse       = 6,
    Flush           = 5,
    Straight        = 4,
    ThreeOfAKind    = 3,
    TwoPair         = 2,
    OnePair         = 1,
    HighCard        = 0,
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
enum Rank {
    LowAce  = 1,
    Two     = 2,
    Three   = 3,
    Four    = 4,
    Five    = 5,
    Six     = 6,
    Seven   = 7,
    Eight   = 8,
    Nine    = 9,
    Ten     = 10,
    J       = 11,
    Q       = 12,
    K       = 13,
    HighAce = 14,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
enum Suit {
    Club    = 0,
    Diamond = 1,
    Heart   = 2,
    Spade   = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    str: &'a str,
    cards: Vec<Card>,
    poker_hand: PokerHand,

    /// The ordered (highest to lowest) group of ranks in this hand.
    /// For example, if a Hand consists of cards of ranks (2,3,J,5,J),
    /// then this should be (J, 5, 3 2). How we interpret this depends on the
    /// type of our `poker_hand`.
    rank_groups: Vec<Rank>,
}

impl Card {

    fn from(s: &str) -> Self {
        let (rank, suit) = (&s[0..s.len() - 1], &s[s.len() - 1..]);

        let rank = match rank {
            "2"  => Rank::Two,
            "3"  => Rank::Three,
            "4"  => Rank::Four,
            "5"  => Rank::Five,
            "6"  => Rank::Six,
            "7"  => Rank::Seven,
            "8"  => Rank::Eight,
            "9"  => Rank::Nine,
            "10" => Rank::Ten,
            "J"  => Rank::J,
            "Q"  => Rank::Q,
            "K"  => Rank::K,
            "A"  => Rank::HighAce,
            _    => unreachable!(),
        };

        let suit = match suit {
            "C" => Suit::Club,
            "D" => Suit::Diamond,
            "H" => Suit::Heart,
            "S" => Suit::Spade,
            _    => unreachable!(),
        };

        Card {
            rank,
            suit,
        }
    }
}

impl<'a> Hand<'a> {

    /// Returns the sorted ranks of the given `&[Card]`.
    /// This function also handles the case of broadway straight and baby straight.
    fn sorted_ranks(cards: &[Card]) -> Vec<Rank> {

        let mut v = cards
            .iter()
            .filter(|&x | x.rank.ne(&Rank::HighAce))
            .map(|x| x.rank)
            .collect::<Vec<Rank>>();

        v.sort();

        // Handle broadway straight and baby straight.
        if let Some(_ace) = cards.iter().find(|&x| x.rank.eq(&Rank::HighAce)) {
            let min_non_ace_rank = v.iter().reduce(|a, b| if a < b { a } else { b }).unwrap();
            println!("min non ace rank: {:?}", min_non_ace_rank);
            match min_non_ace_rank {
                Rank::Two => {
                    v.splice(..0, vec![Rank::LowAce].drain(..));
                    v
                } 
                Rank::Ten => {
                    v.splice(v.len()..v.len(), vec![Rank::HighAce].drain(..));
                    v
                }
                _ => v
            }
        } else {
            v
        }
    }

    fn is_sequential(cards: &[Card]) -> bool {

        let v = Hand::sorted_ranks(cards);

        v.iter()
         .map(|x| (*x as i32) - (*v.first().unwrap() as i32))
         .eq(0..(v.len() as i32))
    }

    fn is_all_same_suit(cards: &[Card]) -> bool {
        cards
            .iter()
            .all(|x| cards.first().unwrap().suit == (*x).suit)
    }

    fn rank_distribution(cards: &[Card]) -> HashMap<Rank, i32> {

        let mut hm = HashMap::new();

        cards
            .iter()
            .map(|x| x.rank)
            .for_each(|x| *hm.entry(x).or_insert(0) += 1);

        hm
    }

    /// Get the 2-tuple value from the slice of `Cards` that represents the
    /// hierarchy of comparisons that can be made between a pair of Hands.
    fn comparable_hand(cards: &[Card]) -> (PokerHand, Vec<Rank>) {

        let sorted_ranks = |cards: &[Card]| {
            let mut v = Hand::sorted_ranks(cards);
            v.reverse();
            v
        };

        let rev_order = |ranks: &[Rank]| -> Vec<Rank> {
            let mut tmp: Vec<&Rank> = ranks.iter().collect();
            tmp.sort();
            tmp.into_iter().map(|x| *x).rev().collect()
        };

        if Hand::is_sequential(cards) && Hand::is_all_same_suit(cards) {
            (
                PokerHand::StraightFlush,
                sorted_ranks(cards),
            )
        } else if !Hand::is_sequential(cards) && Hand::is_all_same_suit(cards) {
            (
                PokerHand::Flush,
                sorted_ranks(cards),
            )
        } else if Hand::is_sequential(cards) && !Hand::is_all_same_suit(cards) {
            (
                PokerHand::Straight,
                sorted_ranks(cards),
            )
        } else {

            // Handle ace-to-five low rules

            let rank_dist = Hand::rank_distribution(cards);
            let mut it = rank_dist.iter();

            match (it.next(), it.next(), it.next(), it.next(), it.next()) {

                // Four of a kind: Match ranks distribution such that there are
                // four equal ranks, and another kicker.
                (
                    Some((&r1, x)),
                    Some((&r2, y)),
                    None,
                    None,
                    None,
                )
                if (x == &4) || (y == &4) => (
                    PokerHand::FourOfAKind,
                    if x == &4 { vec![r1, r2] } else { vec![r2, r1] },
                ),

                // Full House: Match ranks distribution such that there are 3
                // equal ranks, and another 2 equal ranks.
                (
                    Some((&r1, x)),
                    Some((&r2, y)),
                    None,
                    None,
                    None,
                )
                if (x == &3) || (y == &3) => (
                    PokerHand::FullHouse,
                    if x == &3 { vec![r1, r2] } else { vec![r2, r1] },
                ),

                // Three of a Kind: Match ranks distributions such that there
                // are 3 equal ranks and other two kickers.
                (
                    Some((&r1, x)),
                    Some((&r2, y)),
                    Some((&r3, z)),
                    None,
                    None,
                )
                if (x == &3) || (y == &3) || (z == &3) => (
                    PokerHand::ThreeOfAKind,
                    if x == &3 {
                        let mut it = rev_order(&vec![r2, r3][..]).into_iter();
                        vec![r1, it.next().unwrap(), it.next().unwrap()]
                    } else if y == &3 {
                        let mut it = rev_order(&vec![r1, r3][..]).into_iter();
                        vec![r2, it.next().unwrap(), it.next().unwrap()]
                    } else {
                        let mut it = rev_order(&vec![r1, r2][..]).into_iter();
                        vec![r3, it.next().unwrap(), it.next().unwrap()]
                    }
                ),

                // Two Pair: Match ranks distribution such that there are two
                // pairs of equal ranks, and one kicker.
                (
                    Some((&r1, x)),
                    Some((&r2, y)),
                    Some((&r3, z)),
                    None,
                    None,
                )
                if (z + x + y == 5) && (x == &1 || y == &1 || z == &1)  => (
                     PokerHand::TwoPair,
                    if x == &1 {
                        let mut it = rev_order(&vec![r2, r3][..]).into_iter();
                        vec![it.next().unwrap(), it.next().unwrap(), r1]
                    } else if y == &1 {
                        let mut it = rev_order(&vec![r1, r3][..]).into_iter();
                        vec![it.next().unwrap(), it.next().unwrap(), r2]
                    } else {
                        let mut it = rev_order(&vec![r1, r2][..]).into_iter();
                        vec![it.next().unwrap(), it.next().unwrap(), r3]
                    }
                ),

                // One Pair: Match ranks distribution such that there are one
                // pair of equal ranks, and three other kickers.
                (
                    Some((&r1, x)),
                    Some((&r2, y)),
                    Some((&r3, z)),
                    Some((&r4, w)),
                    None,
                ) if (x == &2) || (y == &2) || (z == &2) || (w == &2) => (
                    PokerHand::OnePair,
                    if x == &2 {
                        let mut it = rev_order(&vec![r2, r3, r4][..]).into_iter();
                        vec![r1, it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
                    } else if y == &2 {
                        let mut it = rev_order(&vec![r1, r3, r4][..]).into_iter();
                        vec![r2, it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
                    } else if z == &2 {
                        let mut it = rev_order(&vec![r1, r2, r4][..]).into_iter();
                        vec![r3, it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
                    } else {
                        let mut it = rev_order(&vec![r1, r2, r3][..]).into_iter();
                        vec![r4, it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
                    }
                ),

                // High Card
                (
                    Some((&r1, _)),
                    Some((&r2, _)),
                    Some((&r3, _)),
                    Some((&r4, _)),
                    Some((&r5, _)),
                ) => (
                    PokerHand::HighCard,
                    rev_order(&vec![r1, r2, r3, r4, r5][..])
                ),

                // Should be unreachable
                _ => unreachable!()
            }
        }
    }

    fn from(s: &'a str) -> Self {
        let cards = s
            .split(" ")
            .into_iter()
            .map(|x| Card::from(x)).collect::<Vec<Card>>();

        let (poker_hand, rank_groups) = Hand::comparable_hand(&cards[..]);

        Hand { str: s, cards, poker_hand, rank_groups }
    }
}

impl<'a> PartialOrd for Hand<'a> {

    /// Hand is first compared by the virtue of their Poker Hand ranking.
    /// If both hands has equal Poker Hand, they're compared using each next highest
    /// ranked card, down to the last card.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        if self.str.eq(other.str) {
            return Some(Ordering::Equal);
        }

        if !self.poker_hand.eq(&other.poker_hand) {
            self.poker_hand.partial_cmp(&other.poker_hand)
        } else {
            let lhs = self 
                .rank_groups
                .iter()
                .map(|x| *x)
                .collect::<Vec<Rank>>();
            let rhs = other
                .rank_groups
                .iter()
                .map(|x| *x)
                .collect::<Vec<Rank>>();

            let (lhs, rhs) = lhs
                .iter()
                .zip(&rhs)
                .skip_while(|(&a, &b)| a.partial_cmp(&b).eq(&Some(Ordering::Equal)))
                .next()
                .or(Some((lhs.last().unwrap(), rhs.last().unwrap())))
                .unwrap();
            lhs.partial_cmp(&rhs)
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {

    let mut vec_hands = hands
        .iter()
        .map(|x| Hand::from(*x))
        .collect::<Vec<Hand>>();

    vec_hands.sort_by(|a, b| {
        b.partial_cmp(&a).unwrap()
    });

    vec_hands
        .iter()
        .filter(|x|
            x.partial_cmp(&vec_hands.first().unwrap()).ne(&Some(Ordering::Less)))
        .map(|x| x.str)
        .collect()
}