use super::{file_reader::read_file, number_extractor::parse_numbers_from_string};

fn get_next_number_of_sequence(sequence: &Vec<i32>) -> i32 {
    let mut last_numbers: Vec<i32> = vec![];
    let mut sequence = sequence.clone();
    loop {
        last_numbers.push(*sequence.get(sequence.len() - 1).unwrap());
        let mut diffs: Vec<i32> = vec![];
        for i in 1..sequence.len() {
            let diff = sequence.get(i).unwrap() - sequence.get(i - 1).unwrap();
            diffs.push(diff);
        }
        if diffs.iter().all(|&diff| diff == 0) {
            break;
        }
        sequence = diffs
    }
    last_numbers.iter().fold(0, |acc, num| acc + num)
}

fn get_previous_number_of_sequence(sequence: &Vec<i32>) -> i32 {
    let mut first_numbers: Vec<i32> = vec![];
    let mut sequence = sequence.clone();
    loop {
        first_numbers.push(*sequence.get(0).unwrap());
        let mut diffs: Vec<i32> = vec![];
        for i in 1..sequence.len() {
            let diff = sequence.get(i).unwrap() - sequence.get(i - 1).unwrap();
            diffs.push(diff);
        }
        if diffs.iter().all(|&diff| diff == 0) {
            break;
        }
        sequence = diffs
    }
    first_numbers.iter().enumerate().fold(0, |acc, (i, &num)| {
        acc + match i % 2 == 0 {
            true => num,
            false => -num,
        }
    })
}

pub fn get_sum_of_next_numbers_from_file(filename: &str) -> i32 {
    read_file(filename)
        .lines()
        .map(|line| {
            let sequence = parse_numbers_from_string(line);
            let next_number = get_next_number_of_sequence(&sequence);
            next_number
        })
        .fold(0, |acc, curr| acc + curr)
}

pub fn get_sum_of_previous_numbers_from_file(filename: &str) -> i32 {
    read_file(filename)
        .lines()
        .map(|line| {
            let sequence = parse_numbers_from_string(line);
            let prev_number = get_previous_number_of_sequence(&sequence);
            prev_number
        })
        .fold(0, |acc, curr| acc + curr)
}
#[cfg(test)]
mod test {
    use super::{
        get_next_number_of_sequence, get_previous_number_of_sequence,
        get_sum_of_next_numbers_from_file, get_sum_of_previous_numbers_from_file,
    };
    use rstest::rstest;

    #[rstest]
    #[case(vec![0, 3, 6, 9, 12, 15], 18)]
    #[case(vec![1, 3, 6, 10, 15, 21], 28)]
    #[case(vec![10, 13, 16, 21, 30, 45], 68)]
    fn gets_next_number_of_sequence(#[case] sequence: Vec<i32>, #[case] expected_next_number: i32) {
        let next_number = get_next_number_of_sequence(&sequence);

        assert_eq!(next_number, expected_next_number);
    }

    #[test]
    fn sums_all_next_numbers_from_input_file() {
        let sum_of_next_numbers =
            get_sum_of_next_numbers_from_file("./src/advent_of_code/day_nine/test-input.txt");

        assert_eq!(sum_of_next_numbers, 114);
    }

    #[rstest]
    #[case(vec![0, 3, 6, 9, 12, 15], -3)]
    #[case(vec![1, 3, 6, 10, 15, 21], 0)]
    #[case(vec![10, 13, 16, 21, 30, 45], 5)]
    fn gets_previous_number_of_sequence(
        #[case] sequence: Vec<i32>,
        #[case] expected_previous_number: i32,
    ) {
        let previous_number = get_previous_number_of_sequence(&sequence);

        assert_eq!(previous_number, expected_previous_number);
    }

    #[test]
    fn sums_all_previous_numbers_from_input_file() {
        let sum_of_previous_numbers =
            get_sum_of_previous_numbers_from_file("./src/advent_of_code/day_nine/test-input.txt");

        assert_eq!(sum_of_previous_numbers, 2);
    }
}
