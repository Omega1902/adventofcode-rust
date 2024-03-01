use crate::util::{print_full_result, read_chars};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pos(u64, u64);

impl Pos {
    fn distance(&self, other: &Pos) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

fn enlarge_grid(grid: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let rows: Vec<usize> =
        grid.iter().enumerate().filter(|(_, row)| row.iter().all(|item| item == &'.')).map(|(i, _)| i).collect();
    let cols: Vec<usize> =
        (0..grid[0].len()).filter(|i| grid.iter().map(|row| row[*i]).all(|item| item == '.')).collect();
    (rows, cols)
}

fn get_star_positions(grid: &Vec<Vec<char>>, enlarge: usize) -> Vec<Pos> {
    fn add_spacing(id: usize, enlarged: &Vec<usize>, enlarge: usize) -> u64 {
        let mut spacing = enlarged.iter().filter(|i| i < &&id).count();
        if spacing > 0 {
            spacing *= enlarge - 1;
        }
        (id + spacing) as u64
    }
    let (enlarged_rows, enlarged_cols) = enlarge_grid(grid);
    let mut result = vec![];
    for (row_id, row) in grid.iter().enumerate() {
        for (col_id, item) in row.iter().enumerate() {
            if item == &'#' {
                result.push(Pos(
                    add_spacing(row_id, &enlarged_rows, enlarge),
                    add_spacing(col_id, &enlarged_cols, enlarge),
                ))
            }
        }
    }
    result
}

fn get_stars_distance(grid: &Vec<Vec<char>>, enlarge: usize) -> u64 {
    let stars = get_star_positions(&grid, enlarge);
    stars.into_iter().combinations(2).map(|cur_stars| cur_stars[0].distance(&cur_stars[1])).sum()
}

fn challenge1(grid: &Vec<Vec<char>>) -> u64 {
    get_stars_distance(grid, 2)
}

fn challenge2(grid: &Vec<Vec<char>>) -> u64 {
    get_stars_distance(grid, 1_000_000)
}

pub fn main() {
    let filename = "data/2023/day11.txt";
    print_full_result(11, filename, read_chars, challenge1, challenge2);
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::to_chars;

    const EXAMPLE_INPUT1: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_enlarge_grid() {
        assert_eq!(enlarge_grid(&to_chars(EXAMPLE_INPUT1)), (vec![3, 7], vec![2, 5, 8]));
    }

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_chars(EXAMPLE_INPUT1)), 374);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(get_stars_distance(&to_chars(EXAMPLE_INPUT1), 10), 1030);
        assert_eq!(get_stars_distance(&to_chars(EXAMPLE_INPUT1), 100), 8410);
    }
}
