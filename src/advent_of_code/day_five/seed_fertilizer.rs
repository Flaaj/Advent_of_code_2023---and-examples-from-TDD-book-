use super::{file_reader::FileReader, number_extractor::NumberExtractor};

type MapperRange = (u64, u64, u64);

#[derive(Debug, PartialEq)]
struct Mapper {
    ranges: Vec<MapperRange>,
}

impl Mapper {
    fn new(ranges: Vec<MapperRange>) -> Self {
        Self { ranges }
    }

    fn map(&self, source: u64) -> u64 {
        let mut destination: Option<u64> = None;

        for &(dest_start, source_start, length) in &self.ranges {
            if source >= source_start && source < source_start + length {
                let diff = source - source_start;
                destination = Some(dest_start + diff);
                break;
            }
        }

        match destination {
            None => source,
            Some(destination) => destination,
        }
    }
}

#[derive(Debug, PartialEq)]
struct MappersChain {
    seeds: Vec<u64>,
    seed_to_soil_mapper: Mapper,
    soil_to_fertilizer_mapper: Mapper,
    fertilizer_to_water_mapper: Mapper,
    water_to_light_mapper: Mapper,
    light_to_temperature_mapper: Mapper,
    temperature_to_humidity_mapper: Mapper,
    humidity_to_location_mapper: Mapper,
}

impl MappersChain {
    fn get_seed_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil_mapper.map(seed);
        let fertilizer = self.soil_to_fertilizer_mapper.map(soil);
        let water = self.fertilizer_to_water_mapper.map(fertilizer);
        let light = self.water_to_light_mapper.map(water);
        let temperature = self.light_to_temperature_mapper.map(light);
        let humidity = self.temperature_to_humidity_mapper.map(temperature);
        let location = self.humidity_to_location_mapper.map(humidity);
        location
    }

    fn get_locations_of_seeds(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|&seed| self.get_seed_location(seed))
            .collect()
    }

    fn get_seed_ranges(&self) -> Vec<u64> {
        let mut seeds: Vec<u64> = vec![];
        let ranges_count = &self.seeds.len() / 2;

        for i in 0..ranges_count {
            let range_start = *self.seeds.get(2 * i).unwrap();
            let range_length = *self.seeds.get(2 * i + 1).unwrap();
            let mut range = (range_start..(range_start + range_length)).collect::<Vec<u64>>();
            seeds.append(&mut range);
        }

        seeds
    }

    fn get_locations_of_seed_ranges(&self) -> u64 {
        let seed_ranges = self.get_seed_ranges();
        let mut min = 10_000_000_00;
        for (l, &seed) in seed_ranges.iter().enumerate() {
            min = min.min(self.get_seed_location(seed));
        }
        min
        // seed_ranges
        //     .iter()
        //     .map(|&seed| self.get_seed_location(seed))
        //     .collect()
    }
}

struct MapperParser {
    number_extractor: NumberExtractor,
}

impl MapperParser {
    fn new() -> Self {
        Self {
            number_extractor: NumberExtractor::new(),
        }
    }

    fn parse(&self, string: String) -> MappersChain {
        let mut temp: Vec<MapperRange> = vec![];

        let mut seeds: Vec<u64> = vec![];
        let mut seed_to_soil_ranges: Vec<MapperRange> = vec![];
        let mut soil_to_fertilizer_ranges: Vec<MapperRange> = vec![];
        let mut fertilizer_to_water_ranges: Vec<MapperRange> = vec![];
        let mut water_to_light_ranges: Vec<MapperRange> = vec![];
        let mut light_to_temperature_ranges: Vec<MapperRange> = vec![];
        let mut temperature_to_humidity_ranges: Vec<MapperRange> = vec![];
        let mut humidity_to_location_ranges: Vec<MapperRange> = vec![];

        for line in string.lines() {
            let numbers = self.number_extractor.parse_numbers(line);
            if numbers.len() == 0 {
                if temp.len() > 0 {
                    if seed_to_soil_ranges.len() == 0 {
                        seed_to_soil_ranges = temp;
                    } else if soil_to_fertilizer_ranges.len() == 0 {
                        soil_to_fertilizer_ranges = temp;
                    } else if fertilizer_to_water_ranges.len() == 0 {
                        fertilizer_to_water_ranges = temp;
                    } else if water_to_light_ranges.len() == 0 {
                        water_to_light_ranges = temp;
                    } else if light_to_temperature_ranges.len() == 0 {
                        light_to_temperature_ranges = temp;
                    } else if temperature_to_humidity_ranges.len() == 0 {
                        temperature_to_humidity_ranges = temp;
                    } else if humidity_to_location_ranges.len() == 0 {
                        humidity_to_location_ranges = temp;
                    }
                    temp = vec![];
                }
            } else if seeds.len() == 0 {
                seeds = numbers;
            } else {
                let dest = *numbers.get(0).unwrap();
                let source = *numbers.get(1).unwrap();
                let length = *numbers.get(2).unwrap();
                temp.push((dest, source, length));
            }
        }

        if temp.len() > 0 {
            if seed_to_soil_ranges.len() == 0 {
                seed_to_soil_ranges = temp;
            } else if soil_to_fertilizer_ranges.len() == 0 {
                soil_to_fertilizer_ranges = temp;
            } else if fertilizer_to_water_ranges.len() == 0 {
                fertilizer_to_water_ranges = temp;
            } else if water_to_light_ranges.len() == 0 {
                water_to_light_ranges = temp;
            } else if light_to_temperature_ranges.len() == 0 {
                light_to_temperature_ranges = temp;
            } else if temperature_to_humidity_ranges.len() == 0 {
                temperature_to_humidity_ranges = temp;
            } else if humidity_to_location_ranges.len() == 0 {
                humidity_to_location_ranges = temp;
            }
        }

        MappersChain {
            seeds,
            fertilizer_to_water_mapper: Mapper::new(fertilizer_to_water_ranges),
            humidity_to_location_mapper: Mapper::new(humidity_to_location_ranges),
            light_to_temperature_mapper: Mapper::new(light_to_temperature_ranges),
            seed_to_soil_mapper: Mapper::new(seed_to_soil_ranges),
            soil_to_fertilizer_mapper: Mapper::new(soil_to_fertilizer_ranges),
            temperature_to_humidity_mapper: Mapper::new(temperature_to_humidity_ranges),
            water_to_light_mapper: Mapper::new(water_to_light_ranges),
        }
    }
}

pub struct LocationFinder {
    file_reader: FileReader,
    mapper_parser: MapperParser,
    mappers_chain: Option<MappersChain>,
}

impl LocationFinder {
    pub fn new() -> Self {
        Self {
            file_reader: FileReader::new(),
            mapper_parser: MapperParser::new(),
            mappers_chain: None,
        }
    }

    pub fn load_mappers_from_file(&mut self, filename: &str) {
        let string = self.file_reader.read(filename);
        let mappers_chain = self.mapper_parser.parse(string);
        self.mappers_chain = Some(mappers_chain);
    }

    pub fn find_lowest_location_number_part_one(&self) -> u64 {
        let locations = match &self.mappers_chain {
            None => vec![],
            Some(mappers_chain) => mappers_chain.get_locations_of_seeds(),
        };

        if locations.len() == 0 {
            return 0;
        }

        let initial = *locations.get(0).unwrap();
        locations.iter().fold(initial, |acc, &loc| acc.min(loc))
    }

    pub fn find_lowest_location_number_part_two(&self) -> u64 {
        self.mappers_chain.as_ref().unwrap().get_locations_of_seed_ranges()
    }
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_five::{
        file_reader::FileReader,
        seed_fertilizer::{LocationFinder, MapperParser, MappersChain},
    };

    use super::Mapper;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1)]
    #[case(49, 49)]
    #[case(98, 50)]
    #[case(99, 51)]
    #[case(50, 52)]
    #[case(60, 62)]
    #[case(97, 99)]
    #[case(100, 100)]
    fn maps_input_to_output(#[case] source: u64, #[case] expected_destination: u64) {
        let mapper = Mapper::new(vec![(50, 98, 2), (52, 50, 48)]);

        let destination = mapper.map(source);

        assert_eq!(destination, expected_destination);
    }

    #[test]
    fn reads_mappers_chain_from_input_file() {
        let mut file_reader = FileReader::new();
        let string = file_reader.read("./src/advent_of_code/day_five/test-input.txt");
        let mapper_parser = MapperParser::new();

        let mappers_chain = mapper_parser.parse(string);

        assert_eq!(
            mappers_chain,
            MappersChain {
                seeds: vec![79, 14, 55, 13],
                seed_to_soil_mapper: Mapper::new(vec![(50, 98, 2), (52, 50, 48)]),
                soil_to_fertilizer_mapper: Mapper::new(vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)]),
                fertilizer_to_water_mapper: Mapper::new(vec![
                    (49, 53, 8),
                    (0, 11, 42),
                    (42, 0, 7),
                    (57, 7, 4)
                ]),
                water_to_light_mapper: Mapper::new(vec![(88, 18, 7), (18, 25, 70)]),
                light_to_temperature_mapper: Mapper::new(vec![
                    (45, 77, 23),
                    (81, 45, 19),
                    (68, 64, 13)
                ]),
                temperature_to_humidity_mapper: Mapper::new(vec![(0, 69, 1), (1, 0, 69)]),
                humidity_to_location_mapper: Mapper::new(vec![(60, 56, 37), (56, 93, 4)]),
            }
        )
    }

    #[test]
    fn gets_location_of_a_seed() {
        let mut file_reader = FileReader::new();
        let string = file_reader.read("./src/advent_of_code/day_five/test-input.txt");
        let mapper_parser = MapperParser::new();
        let mappers_chain = mapper_parser.parse(string);

        let locations_of_seeds = mappers_chain.get_locations_of_seeds();

        assert_eq!(locations_of_seeds, vec![82, 43, 86, 35]);
    }

    #[test]
    fn gets_lowest_location_number_from_input_file_part_one() {
        let mut location_finder = LocationFinder::new();
        location_finder.load_mappers_from_file("./src/advent_of_code/day_five/test-input.txt");

        let lowers_location_number = location_finder.find_lowest_location_number_part_one();

        assert_eq!(lowers_location_number, 35);
    }

    #[test]
    fn gets_lowest_location_number_from_input_file_part_two() {
        let mut location_finder = LocationFinder::new();
        location_finder.load_mappers_from_file("./src/advent_of_code/day_five/test-input.txt");

        let lowers_location_number = location_finder.find_lowest_location_number_part_two();

        assert_eq!(lowers_location_number, 46);
    }
}
