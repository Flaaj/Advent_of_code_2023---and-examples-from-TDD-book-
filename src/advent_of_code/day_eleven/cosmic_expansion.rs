type Galaxy = (u32, u32);

#[derive(Debug, PartialEq)]
pub struct GalaxyMap {
    rows: Vec<Vec<char>>,
}

impl From<String> for GalaxyMap {
    fn from(value: String) -> Self {
        Self {
            rows: value.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

impl GalaxyMap {
    fn is_row_empty(&self, row_number: usize) -> Option<bool> {
        let row = self.rows.get(row_number)?;
        Some(row.iter().all(|&c| c == '.'))
    }

    fn is_column_empty(&self, column_number: usize) -> Option<bool> {
        for row in &self.rows {
            let &col = row.get(column_number)?;
            if col != '.' {
                return Some(false);
            }
        }
        Some(true)
    }

    fn add_row_at_index(&mut self, index: usize) {
        let first_row_len = self.rows.get(0).unwrap().len();
        let new_row: Vec<char> = (0..first_row_len).map(|_| '.').collect();
        self.rows.insert(index, new_row);
    }

    fn add_column_at_index(&mut self, index: usize) {
        for row in &mut self.rows {
            row.insert(index, '.');
        }
    }

    pub fn expand(&mut self) {
        let size_y = self.rows.len();
        for y in (0..size_y).rev() {
            match self.is_row_empty(y) {
                Some(true) => self.add_row_at_index(y),
                _ => (),
            }
        }

        let size_x = self.rows.get(0).unwrap().len();
        for x in (0..size_x).rev() {
            match self.is_column_empty(x) {
                Some(true) => self.add_column_at_index(x),
                _ => (),
            }
        }
    }

    fn get_galaxies(&self) -> Vec<Galaxy> {
        let mut galaxies: Vec<Galaxy> = vec![];
        for (y, row) in self.rows.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if col == &'#' {
                    galaxies.push((y as u32, x as u32))
                }
            }
        }
        galaxies
    }

    pub fn get_sum_of_shortest_distances_between_galaxies(&self) -> u32 {
        let mut sum = 0u32;
        let galaxies = self.get_galaxies();
        for a in 0..galaxies.len() {
            for b in a..galaxies.len() {
                let galaxy_a = galaxies.get(a).unwrap();
                let galaxy_b = galaxies.get(b).unwrap();
                sum += get_shortest_distance(galaxy_a, galaxy_b);
            }
        }
        sum
    }
}

fn get_shortest_distance(galaxy_a: &Galaxy, galaxy_b: &Galaxy) -> u32 {
    let (ay, ax) = galaxy_a;
    let (by, bx) = galaxy_b;
    ay.abs_diff(*by) + ax.abs_diff(*bx)
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_eleven::file_reader::read_file;

    use super::{get_shortest_distance, Galaxy, GalaxyMap};

    #[test]
    fn creates_galaxy_map_from_string() {
        let string = String::from(".#..\n....\n.#..\n#..#");

        let galaxy_map = GalaxyMap::from(string);

        assert_eq!(
            galaxy_map,
            GalaxyMap {
                rows: vec![
                    vec!['.', '#', '.', '.'],
                    vec!['.', '.', '.', '.'],
                    vec!['.', '#', '.', '.'],
                    vec!['#', '.', '.', '#']
                ]
            }
        )
    }

    #[test]
    fn determines_if_row_is_empty() {
        let galaxy_map = GalaxyMap {
            rows: vec![
                vec!['.', '#', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['.', '#', '.', '.'],
                vec!['#', '.', '.', '#'],
            ],
        };

        assert_eq!(galaxy_map.is_row_empty(0), Some(false));
        assert_eq!(galaxy_map.is_row_empty(1), Some(true));
        assert_eq!(galaxy_map.is_row_empty(4), None);
    }

    #[test]
    fn determines_if_column_is_empty() {
        let galaxy_map = GalaxyMap {
            rows: vec![
                vec!['.', '#', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['.', '#', '.', '.'],
                vec!['#', '.', '.', '#'],
            ],
        };

        assert_eq!(galaxy_map.is_column_empty(1), Some(false));
        assert_eq!(galaxy_map.is_column_empty(2), Some(true));
        assert_eq!(galaxy_map.is_column_empty(4), None);
    }

    #[test]
    fn adds_row() {
        let mut galaxy_map = GalaxyMap {
            rows: vec![
                vec!['.', '#', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['.', '#', '.', '.'],
                vec!['#', '.', '.', '#'],
            ],
        };

        galaxy_map.add_row_at_index(1);

        assert_eq!(
            galaxy_map,
            GalaxyMap {
                rows: vec![
                    vec!['.', '#', '.', '.'],
                    vec!['.', '.', '.', '.'],
                    vec!['.', '.', '.', '.'],
                    vec!['.', '#', '.', '.'],
                    vec!['#', '.', '.', '#'],
                ],
            }
        );
    }
    #[test]
    fn adds_column_at_index() {
        let mut galaxy_map = GalaxyMap {
            rows: vec![
                vec!['.', '#', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['.', '#', '.', '.'],
                vec!['#', '.', '.', '#'],
            ],
        };

        galaxy_map.add_column_at_index(2);

        assert_eq!(
            galaxy_map,
            GalaxyMap {
                rows: vec![
                    vec!['.', '#', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.'],
                    vec!['.', '#', '.', '.', '.'],
                    vec!['#', '.', '.', '.', '#'],
                ],
            }
        );
    }

    #[test]
    fn performs_galactic_expansion() {
        let mut galaxy_map = GalaxyMap {
            rows: vec![
                vec!['.', '#', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['.', '#', '.', '.'],
                vec!['#', '.', '.', '#'],
            ],
        };

        galaxy_map.expand();

        assert_eq!(
            galaxy_map,
            GalaxyMap {
                rows: vec![
                    vec!['.', '#', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.'],
                    vec!['.', '#', '.', '.', '.'],
                    vec!['#', '.', '.', '.', '#'],
                ],
            }
        )
    }

    #[test]
    fn calculates_shortest_distance() {
        assert_eq!(get_shortest_distance(&(0, 1), &(3, 1)), 3);
        assert_eq!(get_shortest_distance(&(0, 1), &(4, 0)), 5);
        assert_eq!(get_shortest_distance(&(0, 1), &(4, 4)), 7);
    }

    #[test]
    fn gets_all_galaxies() {
        let galaxy_map = GalaxyMap {
            rows: vec![
                vec!['.', '#', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.'],
                vec!['.', '#', '.', '.', '.'],
                vec!['#', '.', '.', '.', '#'],
            ],
        };
        let expexted_galaxies: Vec<Galaxy> = vec![(0, 1), (3, 1), (4, 0), (4, 4)];

        let galaxies = galaxy_map.get_galaxies();

        assert_eq!(galaxies.len(), expexted_galaxies.len());
        for expexted_galaxy in expexted_galaxies {
            assert!(galaxies.contains(&expexted_galaxy));
        }
    }

    #[test]
    fn calculates_sum_of_shortest_distances_between_all_galaxies() {
        let string = read_file("./src/advent_of_code/day_eleven/test-input.txt");
        let mut galaxy_map = GalaxyMap::from(string);

        galaxy_map.expand();
        let sum = galaxy_map.get_sum_of_shortest_distances_between_galaxies();

        assert_eq!(sum, 374);
    }
}
