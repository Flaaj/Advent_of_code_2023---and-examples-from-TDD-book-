fn calculate_race_distance(race_time_ms: u64, powering_time_ms: u64) -> u64 {
    let time_moving_ms = race_time_ms - powering_time_ms;
    time_moving_ms * powering_time_ms
}

fn get_winning_races(race_time_ms: u64, current_record_mm: u64) -> u64 {
    let mut winning_races = 0;
    for powering_time_ms in 0..(race_time_ms + 1) {
        let distance_travelled = calculate_race_distance(race_time_ms, powering_time_ms);
        if distance_travelled > current_record_mm {
            winning_races += 1;
        }
    }
    winning_races
}

pub fn get_product_of_winning_races(races: &[(u64, u64)]) -> u64 {
    races.iter().fold(1, |acc, &(race_time_ms, current_record_mm)| {
        let winning_races = get_winning_races(race_time_ms, current_record_mm);
        acc * winning_races
    })
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_six::wait_for_it::{
        get_product_of_winning_races, get_winning_races,
    };

    use super::calculate_race_distance;
    use rstest::rstest;

    #[rstest]
    #[case(7, 0, 0)]
    #[case(7, 1, 6)]
    #[case(7, 2, 10)]
    #[case(7, 3, 12)]
    #[case(7, 4, 12)]
    #[case(7, 5, 10)]
    #[case(7, 6, 6)]
    #[case(7, 7, 0)]
    fn calculates_distance_travelled(
        #[case] race_time_ms: u64,
        #[case] powering_time_ms: u64,
        #[case] expected_distance_travelled_mm: u64,
    ) {
        let distance_travelled_mm = calculate_race_distance(race_time_ms, powering_time_ms);

        assert_eq!(distance_travelled_mm, expected_distance_travelled_mm);
    }

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn calculates_number_of_winning_races(
        #[case] race_time_ms: u64,
        #[case] current_record_mm: u64,
        #[case] expected_number_of_winning_races: u64,
    ) {
        let winning_races = get_winning_races(race_time_ms, current_record_mm);

        assert_eq!(winning_races, expected_number_of_winning_races)
    }

    #[test]
    fn calculates_product_of_each_races_number_of_winning_races() {
        let product = get_product_of_winning_races(&[(7, 9), (15, 40), (30, 200)]);

        assert_eq!(product, 288);
    }
}
