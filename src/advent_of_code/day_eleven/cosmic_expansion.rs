type GalaxyCoords = (u64, u64);
type GalaxyMap = Vec<Vec<char>>;

#[derive(Debug, PartialEq)]
pub struct Universe {
    galaxies: Vec<GalaxyCoords>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

impl Universe {
    fn is_row_empty(galaxy_map: &GalaxyMap, row_number: usize) -> Option<bool> {
        let row = galaxy_map.get(row_number)?;
        Some(row.iter().all(|&c| c == '.'))
    }

    fn get_empty_rows(galaxy_map: &GalaxyMap) -> Vec<usize> {
        let mut empty_rows = vec![];
        for i in 1..galaxy_map.len() {
            if let Some(true) = Self::is_row_empty(galaxy_map, i) {
                empty_rows.push(i);
            }
        }
        empty_rows
    }

    fn is_column_empty(galaxy_map: &GalaxyMap, column_number: usize) -> Option<bool> {
        for row in galaxy_map {
            let &col = row.get(column_number)?;
            if col != '.' {
                return Some(false);
            }
        }
        Some(true)
    }

    fn get_empty_columns(galaxy_map: &GalaxyMap) -> Vec<usize> {
        let mut empty_columns = vec![];
        for i in 1..galaxy_map.get(0).unwrap().len() {
            if let Some(true) = Self::is_column_empty(galaxy_map, i) {
                empty_columns.push(i);
            }
        }
        empty_columns
    }

    fn get_galaxies(galaxy_map: &GalaxyMap) -> Vec<GalaxyCoords> {
        let mut galaxies: Vec<GalaxyCoords> = vec![];
        for (y, row) in galaxy_map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if col == &'#' {
                    galaxies.push((y as u64, x as u64))
                }
            }
        }
        galaxies
    }

    pub fn expand(&mut self, times: u64) {
        self.galaxies = self
            .galaxies
            .iter()
            .map(|&(y, x)| {
                let first_empty_row_before = self
                    .empty_rows
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|&(_, &row)| y > row as u64);
                let empty_rows_before = match first_empty_row_before {
                    None => 0,
                    Some((ry, _)) => 1 + ry as u64,
                };
                let first_empty_column_before = self
                    .empty_columns
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|&(_, &column)| x > column as u64);
                let empty_columns_before = match first_empty_column_before {
                    None => 0,
                    Some((cx, _)) => 1 + cx as u64,
                };
                (
                    y + times * empty_rows_before,
                    x + times * empty_columns_before,
                )
            })
            .collect()
    }

    pub fn get_sum_of_shortest_distances_between_galaxies(&self) -> u64 {
        let mut sum = 0u64;
        for a in 0..self.galaxies.len() {
            for b in a..self.galaxies.len() {
                let galaxy_a = self.galaxies.get(a).unwrap();
                let galaxy_b = self.galaxies.get(b).unwrap();
                sum += get_shortest_distance(galaxy_a, galaxy_b);
            }
        }
        sum
    }
}

impl From<String> for Universe {
    fn from(value: String) -> Self {
        let galaxy_map: GalaxyMap = value.lines().map(|line| line.chars().collect()).collect();
        Self {
            galaxies: Self::get_galaxies(&galaxy_map),
            empty_columns: Self::get_empty_columns(&galaxy_map),
            empty_rows: Self::get_empty_rows(&galaxy_map),
        }
    }
}

fn get_shortest_distance(galaxy_a: &GalaxyCoords, galaxy_b: &GalaxyCoords) -> u64 {
    let (ay, ax) = galaxy_a;
    let (by, bx) = galaxy_b;
    ay.abs_diff(*by) + ax.abs_diff(*bx)
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_eleven::file_reader::read_file;

    use super::{get_shortest_distance, GalaxyCoords, Universe};

    #[test]
    fn creates_universe_from_string() {
        let string = String::from(".#..\n....\n.#..\n#...");

        let universe = Universe::from(string);

        let expexted_galaxies: Vec<GalaxyCoords> = vec![(0, 1), (2, 1), (3, 0)];
        assert_eq!(universe.galaxies.len(), expexted_galaxies.len());
        for expexted_galaxy in expexted_galaxies {
            assert!(universe.galaxies.contains(&expexted_galaxy));
        }
        assert!(universe.empty_rows.len() == 1);
        assert!(universe.empty_rows.contains(&1));
        assert!(universe.empty_columns.len() == 2);
        assert!(universe.empty_columns.contains(&2));
        assert!(universe.empty_columns.contains(&3));
    }

    #[test]
    fn calculates_shortest_distance() {
        assert_eq!(get_shortest_distance(&(0, 1), &(3, 1)), 3);
        assert_eq!(get_shortest_distance(&(0, 1), &(4, 0)), 5);
        assert_eq!(get_shortest_distance(&(0, 1), &(4, 4)), 7);
    }

    #[test]
    fn expands_universe_by_n_times() {
        let string = read_file("./src/advent_of_code/day_eleven/test-input.txt");
        let mut universe = Universe::from(string);

        universe.expand(2);

        assert!(universe.galaxies.contains(&(0, 5)));
        assert!(universe.galaxies.contains(&(2, 0)));
        assert!(universe.galaxies.contains(&(13, 0)));
        assert!(universe.galaxies.contains(&(8, 15)));
    }

    #[test]
    fn calculates_sum_of_shortest_distances_between_all_galaxies() {
        let string = read_file("./src/advent_of_code/day_eleven/test-input.txt");
        let mut universe = Universe::from(string);

        universe.expand(1);
        let sum = universe.get_sum_of_shortest_distances_between_galaxies();

        assert_eq!(sum, 374);
    }
}
