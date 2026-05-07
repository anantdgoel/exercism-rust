// Given a list of poker hands, return a list of those hands which win.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let ranked_hands: Vec<_> = hands.iter().map(|&hand| (hand, hand_rank(hand))).collect();
    let Some(best_rank) = ranked_hands.iter().map(|(_, rank)| *rank).max() else {
        return Vec::new();
    };

    ranked_hands
        .into_iter()
        .filter_map(|(hand, rank)| (rank == best_rank).then_some(hand))
        .collect()
}

// Variants ordered worst -> best so derived Ord ranks them correctly.
// Each payload holds tiebreaker values in descending priority order;
// lexicographic comparison then resolves ties for free.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard([u8; 5]),
    OnePair(u8, [u8; 3]),
    TwoPair(u8, u8, u8),
    ThreeKind(u8, [u8; 2]),
    Straight(u8),
    Flush([u8; 5]),
    FullHouse(u8, u8),
    FourKind(u8, u8),
    StraightFlush(u8),
}

#[derive(Clone, Copy, Debug)]
struct Card {
    rank: u8,
    suit: char,
}

fn parse_card(card: &str) -> Card {
    let (rank, suit) = card.split_at(card.len() - 1);
    let rank = match rank {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        n => n.parse().unwrap(),
    };

    Card {
        rank,
        suit: suit.chars().next().unwrap(),
    }
}

fn hand_rank(hand: &str) -> HandRank {
    let mut cards: Vec<Card> = hand.split_whitespace().map(parse_card).collect();
    cards.sort_unstable_by_key(|card| std::cmp::Reverse(card.rank));
    let ranks = [
        cards[0].rank,
        cards[1].rank,
        cards[2].rank,
        cards[3].rank,
        cards[4].rank,
    ];

    let is_flush = cards.iter().all(|card| card.suit == cards[0].suit);
    let straight = straight_high(ranks);
    let groups = rank_groups(ranks);

    match (groups.as_slice(), is_flush, straight) {
        (_, true, Some(high)) => HandRank::StraightFlush(high),
        ([(4, q), (1, k)], _, _) => HandRank::FourKind(*q, *k),
        ([(3, t), (2, p)], _, _) => HandRank::FullHouse(*t, *p),
        (_, true, _) => HandRank::Flush(ranks),
        (_, _, Some(high)) => HandRank::Straight(high),
        ([(3, t), (1, a), (1, b)], _, _) => HandRank::ThreeKind(*t, [*a, *b]),
        ([(2, h), (2, l), (1, k)], _, _) => HandRank::TwoPair(*h, *l, *k),
        ([(2, p), (1, a), (1, b), (1, c)], _, _) => HandRank::OnePair(*p, [*a, *b, *c]),
        _ => HandRank::HighCard(ranks),
    }
}

fn straight_high(ranks: [u8; 5]) -> Option<u8> {
    if ranks == [14, 5, 4, 3, 2] {
        Some(5)
    } else if ranks.windows(2).all(|w| w[0] == w[1] + 1) {
        Some(ranks[0])
    } else {
        None
    }
}

fn rank_groups(ranks: [u8; 5]) -> Vec<(u8, u8)> {
    let mut groups: Vec<(u8, u8)> = Vec::new();

    // Ranks are already sorted, so repeated ranks are adjacent.
    for rank in ranks {
        match groups.last_mut() {
            Some((count, group_rank)) if *group_rank == rank => *count += 1,
            _ => groups.push((1, rank)),
        }
    }

    // Full house of 8s over 5s sorts to [(3, 8), (2, 5)].
    groups.sort_unstable_by(|a, b| b.cmp(a));
    groups
}
