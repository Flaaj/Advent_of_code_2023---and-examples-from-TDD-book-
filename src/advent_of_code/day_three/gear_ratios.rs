use std::{
    char,
    collections::{HashMap, HashSet},
};

use super::file_reader::FileReader;

type Grid = Vec<Vec<char>>;

struct GridParser {}

impl GridParser {
    fn new() -> Self {
        Self {}
    }

    fn parse(&self, string: String) -> Grid {
        string
            .split("\n")
            .map(|line| line.chars().collect())
            .collect()
    }
}

type CharData = (char, usize, usize);

#[derive(Debug, PartialEq, Clone)]
struct NumberData {
    number: i32,
    adjacent_chars: Vec<CharData>,
}

struct GridNumberLocator {
    grid_parser: GridParser,
    directions: Vec<(i32, i32)>,
}

impl GridNumberLocator {
    fn new() -> Self {
        Self {
            grid_parser: GridParser::new(),
            directions: vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
            ],
        }
    }

    fn get_char_at(grid: &Grid, i: usize, j: usize) -> Option<CharData> {
        match grid.get(i) {
            None => None,
            Some(row) => match row.get(j) {
                None => None,
                Some(&c) => match c.is_digit(10) || c.to_string() == String::from(".") {
                    true => None,
                    false => Some((c, i, j)),
                },
            },
        }
    }

    fn consume_temporary_state(
        grid: &Grid,
        digits: &String,
        hashset: &HashSet<(usize, usize)>,
    ) -> Option<NumberData> {
        if digits.len() == 0 {
            return None;
        }

        let adjacent_chars: Vec<CharData> = hashset
            .iter()
            .map(|(i, j)| Self::get_char_at(grid, *i, *j))
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect();
        let number: i32 = digits.parse().unwrap();

        Some(NumberData {
            number,
            adjacent_chars,
        })
    }

    fn analyze_grid(&self, string: String) -> Vec<NumberData> {
        let grid = self.grid_parser.parse(string);

        let mut hashset: HashSet<(usize, usize)> = HashSet::new();
        let mut digits = String::from("");

        let mut number_data_vec: Vec<NumberData> = Vec::new();

        grid.iter().enumerate().for_each(|(i, row)| {
            row.iter()
                .enumerate()
                .for_each(|(j, &c)| match c.is_digit(10) {
                    true => {
                        self.directions.iter().for_each(|(x, y)| {
                            let x = i as i32 + x;
                            let y = j as i32 + y;
                            if x >= 0 && y >= 0 {
                                hashset.insert((x as usize, y as usize));
                            }
                        });
                        digits.push(c);
                    }
                    false => match Self::consume_temporary_state(&grid, &digits, &hashset) {
                        None => (),
                        Some(number_data) => {
                            number_data_vec.push(number_data);

                            hashset = HashSet::new();
                            digits = String::from("");
                        }
                    },
                });

            match Self::consume_temporary_state(&grid, &digits, &hashset) {
                None => (),
                Some(number_data) => {
                    number_data_vec.push(number_data);

                    hashset = HashSet::new();
                    digits = String::from("");
                }
            }
        });

        number_data_vec
    }
}

struct PartNumbersCalculator {}

impl PartNumbersCalculator {
    fn new() -> Self {
        Self {}
    }

    fn calculate(&self, numbers_data: &Vec<NumberData>) -> i32 {
        numbers_data.iter().fold(0, |acc, number_data| {
            acc + match number_data.adjacent_chars.len() > 0 {
                true => number_data.number,
                false => 0,
            }
        })
    }
}

#[derive(Debug, PartialEq)]
struct Gear {
    number_one: NumberData,
    number_two: NumberData,
}

struct GearFinder {}

impl GearFinder {
    fn new() -> Self {
        Self {}
    }

    fn find(&self, numbers_data_vec: &Vec<NumberData>) -> Vec<Gear> {
        let mut stars: HashMap<CharData, Vec<NumberData>> = HashMap::new();

        numbers_data_vec.iter().for_each(|number_data| {
            number_data.adjacent_chars.iter().for_each(|&char_data| {
                if char_data.0 == '*' {
                    stars
                        .entry(char_data)
                        .and_modify(|vec| vec.push(number_data.clone()))
                        .or_insert(vec![number_data.clone()]);
                }
            });
        });

        let mut gears: Vec<Gear> = vec![];

        stars.iter().for_each(|(_, numbers_data_vec)| {
            if numbers_data_vec.len() == 2 {
                gears.push(Gear {
                    number_one: numbers_data_vec.get(0).unwrap().to_owned(),
                    number_two: numbers_data_vec.get(1).unwrap().to_owned(),
                })
            }
        });

        gears
    }
}

struct GearRatioCalculator {}

impl GearRatioCalculator {
    fn new() -> Self {
        Self {}
    }

    fn calculate(&self, gears: &Vec<Gear>) -> i32 {
        gears.iter().fold(0, |acc, gear| {
            acc + gear.number_one.number * gear.number_two.number
        })
    }
}
pub struct GearRatios {
    numbers_data_vec: Vec<NumberData>,
    gears: Vec<Gear>,
    file_reader: FileReader,
    parts_number_calculator: PartNumbersCalculator,
    grid_number_locator: GridNumberLocator,
    gear_finder: GearFinder,
    gear_ratio_calculator: GearRatioCalculator,
}

impl GearRatios {
    pub fn new() -> Self {
        Self {
            numbers_data_vec: vec![],
            gears: vec![],
            file_reader: FileReader::new(),
            parts_number_calculator: PartNumbersCalculator::new(),
            grid_number_locator: GridNumberLocator::new(),
            gear_finder: GearFinder::new(),
            gear_ratio_calculator: GearRatioCalculator::new(),
        }
    }

    pub fn read_from_file(&mut self, filename: &str) {
        let string = self.file_reader.read(filename);
        let numbers_data_vec = self.grid_number_locator.analyze_grid(string);
        self.numbers_data_vec = numbers_data_vec;
        let gears = self.gear_finder.find(&self.numbers_data_vec);
        self.gears = gears;
    }

    pub fn calculate_sum_of_part_numbers(&self) -> i32 {
        self.parts_number_calculator
            .calculate(&self.numbers_data_vec)
    }

    pub fn calculate_of_gear_ratios(&self) -> i32 {
        self.gear_ratio_calculator.calculate(&self.gears)
    }
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_three::gear_ratios::{
        GearFinder, GearRatios, GridNumberLocator, GridParser, NumberData, PartNumbersCalculator,
    };

    #[test]
    fn parses_string_to_grid() {
        let string_from_file = String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
        let grid_parser = GridParser::new();

        let grid = grid_parser.parse(string_from_file);

        assert_eq!(grid.len(), 10);
        assert_eq!(grid.get(0).unwrap().len(), 10);
    }

    #[test]
    fn finds_number_and_its_adjacent_characters() {
        let grid_number_locator = GridNumberLocator::new();

        let numbers_data = grid_number_locator.analyze_grid("467....114\n11.*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string());

        let first_item = numbers_data.get(0).unwrap();
        let expected = NumberData {
            number: 467,
            adjacent_chars: vec![('*', 1, 3)],
        };
        assert_eq!(*first_item, expected);
    }

    #[test]
    fn number_at_the_begining_of_a_line_is_not_a_continuation_of_a_number_ending_on_previous_line()
    {
        let grid_number_locator = GridNumberLocator::new();

        let numbers_data = grid_number_locator.analyze_grid("467....114\n11.*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string());

        let first_item = numbers_data.get(1).unwrap();
        let expected = NumberData {
            number: 114,
            adjacent_chars: vec![],
        };
        assert_eq!(*first_item, expected);
    }

    #[test]
    fn extracts_random_number_from_somewhere_in_the_middle() {
        let grid_number_locator = GridNumberLocator::new();

        let numbers_data = grid_number_locator.analyze_grid("467....114\n11.*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string());

        let expected = NumberData {
            number: 664,
            adjacent_chars: vec![('$', 8, 3)],
        };
        assert!(numbers_data.contains(&expected));
    }

    #[test]
    fn calculates_sum_of_part_numbers() {
        let grid_number_locator = GridNumberLocator::new();
        let numbers_data = grid_number_locator.analyze_grid("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string());
        let part_numbers_calculator = PartNumbersCalculator::new();

        let sum = part_numbers_calculator.calculate(&numbers_data);

        assert_eq!(sum, 4361);
    }

    #[test]
    fn finds_all_gears_on_the_grid() {
        let grid_number_locator = GridNumberLocator::new();
        let numbers_data = grid_number_locator.analyze_grid("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string());
        let gear_finder = GearFinder::new();

        let gears = gear_finder.find(&numbers_data);

        assert_eq!(gears.len(), 2)
    }

    #[test]
    fn calculates_sum_of_part_numbers_from_file_input() {
        let mut gear_ratios = GearRatios::new();
        gear_ratios.read_from_file("./src/advent_of_code/day_three/test-input.txt");

        let sum = gear_ratios.calculate_sum_of_part_numbers();

        assert_eq!(sum, 4361);
    }

    #[test]
    fn calculates_sum_gear_ratios_from_file_input() {
        let mut gear_ratios = GearRatios::new();
        gear_ratios.read_from_file("./src/advent_of_code/day_three/test-input.txt");

        let sum = gear_ratios.calculate_of_gear_ratios();

        assert_eq!(sum, 467835);
    }
}
