use std::{cmp::Ordering, collections::HashMap};

use super::file_reader::FileReader;

fn get_card_strength(card: char) -> u8 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0,
    }
}

fn get_hand_strength(hand: &String) -> u8 {
    let mut card_counts_map = HashMap::new();
    let mut jokers_count = 0;
    hand.chars().for_each(|card| {
        if card == 'J' {
            jokers_count += 1;
        } else {
            let current = *card_counts_map.get(&card).unwrap_or(&0);
            card_counts_map.insert(card, current + 1);
        }
    });

    if jokers_count == 5 {
        return 6;
    }

    let max = *card_counts_map.values().max().unwrap_or(&0);
    let mut has_added_jokers = false;
    let occurences: Vec<u8> = card_counts_map
        .values()
        .map(|occ| {
            if *occ == max && !has_added_jokers {
                has_added_jokers = true;
                *occ + jokers_count
            } else {
                *occ
            }
        })
        .collect();

    if occurences.contains(&&5) {
        6
    } else if occurences.contains(&&4) {
        5
    } else if occurences.contains(&&3) && occurences.contains(&&2) {
        4
    } else if occurences.contains(&&3) {
        3
    } else if occurences.contains(&&2) && occurences.len() == 3 {
        2
    } else if occurences.contains(&&2) {
        1
    } else {
        0
    }
}

fn compare_cards_strength_at_index(hand_one: &String, hand_two: &String, index: usize) -> Ordering {
    let card_one = hand_one.chars().nth(index).unwrap();
    let card_two = hand_two.chars().nth(index).unwrap();
    let card_one_strength = get_card_strength(card_one);
    let card_two_strength = get_card_strength(card_two);
    return card_one_strength.cmp(&card_two_strength);
}

#[derive(Eq, Debug)]
pub struct HandData {
    hand: String,
    bid: u64,
}

impl PartialEq for HandData {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Ord for HandData {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_strength = get_hand_strength(&self.hand);
        let other_strength = get_hand_strength(&other.hand);
        if self_strength > other_strength {
            return Ordering::Greater;
        }
        if self_strength < other_strength {
            return Ordering::Less;
        }
        for card_index in 0..5 {
            let ordering = compare_cards_strength_at_index(&self.hand, &other.hand, card_index);
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for HandData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn read_hands_from_file(filename: &str) -> Vec<HandData> {
    FileReader::read(filename)
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(" ").collect();
            let hand = split.get(0).unwrap().to_string();
            let bid = split.get(1).unwrap().parse::<u64>().unwrap();
            HandData { hand, bid }
        })
        .collect()
}

pub fn calculate_total_winnings(hands: &mut Vec<HandData>) -> u64 {
    hands.sort();
    hands.iter().enumerate().fold(0, |acc, (i, hand_data)| {
        // println!("{}, {}, {}", hand_data.hand, i, (i as u64 + 1) * hand_data.bid);
        acc + (i as u64 + 1) * hand_data.bid
    })
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use rstest::rstest;

    use crate::advent_of_code::day_seven::camel_cards_2::{
        calculate_total_winnings, get_card_strength, get_hand_strength, read_hands_from_file,
        HandData,
    };

    #[rstest]
    #[case('A', 13)]
    #[case('T', 10)]
    #[case('9', 9)]
    #[case('2', 2)]
    fn gets_strength_of_a_card(#[case] card: char, #[case] strength: u8) {
        assert_eq!(get_card_strength(card), strength);
    }

    #[rstest]
    #[case("32T3K", 1)]
    #[case("T55J5", 5)]
    #[case("KK677", 2)]
    #[case("KTJJT", 5)]
    #[case("QQQJA", 5)]
    #[case("QQQ22", 4)]
    #[case("AAAAA", 6)]
    #[case("AKJTQ", 1)]
    #[case("2222A", 5)]
    fn gets_strength_of_a_hand(#[case] hand: String, #[case] strength: u8) {
        assert_eq!(get_hand_strength(&hand), strength);
    }

    #[rstest]
    #[case("32T3K", "T55J5", Ordering::Less)]
    #[case("AAAAA", "QQQ22", Ordering::Greater)]
    #[case("AAAAA", "AAAAA", Ordering::Equal)]
    #[case("32222", "2AAAA", Ordering::Greater)]
    #[case("32222", "4AAAA", Ordering::Less)]
    #[case("AAAA3", "AAAA4", Ordering::Less)]
    #[case("AAAA4", "AAAA3", Ordering::Greater)]
    #[case("T55J5", "QQQJA", Ordering::Less)]
    #[case("KK677", "KTJJT", Ordering::Less)]
    #[case("22JJJ", "AAAAB", Ordering::Greater)]
    #[case("JJJJA", "JJJJJ", Ordering::Greater)]
    #[case("2222J", "2J222", Ordering::Greater)]
    #[case("JJJJJ", "AAAAQ", Ordering::Greater)]
    fn determines_which_hand_is_stronger(
        #[case] hand_one: String,
        #[case] hand_two: String,
        #[case] expected_ordering: Ordering,
    ) {
        let hand_data_one = HandData {
            hand: hand_one,
            bid: 0,
        };
        let hand_data_two = HandData {
            hand: hand_two,
            bid: 0,
        };

        assert_eq!(hand_data_one.cmp(&hand_data_two), expected_ordering);
    }

    #[test]
    fn parses_hands_from_input_file() {
        let hands = read_hands_from_file("./src/advent_of_code/day_seven/test-input.txt");

        assert_eq!(
            hands,
            vec![
                HandData {
                    hand: String::from("32T3K"),
                    bid: 765,
                },
                HandData {
                    hand: String::from("T55J5"),
                    bid: 684
                },
                HandData {
                    hand: String::from("KK677"),
                    bid: 28
                },
                HandData {
                    hand: String::from("KTJJT"),
                    bid: 220
                },
                HandData {
                    hand: String::from("QQQJA"),
                    bid: 483
                },
            ]
        )
    }

    #[test]
    fn calculates_total_winnings() {
        let hands = &mut read_hands_from_file("./src/advent_of_code/day_seven/test-input.txt");

        let total_winnings = calculate_total_winnings(hands);

        assert_eq!(total_winnings, 5905)
    }
}
