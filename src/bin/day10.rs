use adventofcode_rust::{print_result, read_chars};
use hashbrown::HashSet;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

const CONNECTED_FROM_NORTH: [char; 3] = ['|', '7', 'F'];
const CONNECTED_FROM_EAST: [char; 3] = ['-', 'J', '7'];
const _CONNECTED_FROM_WEST: [char; 3] = ['-', 'L', 'F']; // Not used yet, might need later
const CONNECTED_FROM_SOUTH: [char; 3] = ['|', 'L', 'J'];

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn get_position_in_bounds(&self, direction: Direction, max_pos: &Position) -> Option<Position> {
        match direction {
            Direction::North => {
                if self.row > 0 {
                    Some(self.get_position(direction))
                } else {
                    None
                }
            }
            Direction::South => {
                if self.row < max_pos.row {
                    Some(self.get_position(direction))
                } else {
                    None
                }
            }
            Direction::East => {
                if self.col < max_pos.col {
                    Some(self.get_position(direction))
                } else {
                    None
                }
            }
            Direction::West => {
                if self.col > 0 {
                    Some(self.get_position(direction))
                } else {
                    None
                }
            }
        }
    }
    fn get_position(&self, direction: Direction) -> Position {
        match direction {
            Direction::North => Position { row: self.row - 1, ..*self },
            Direction::South => Position { row: self.row + 1, ..*self },
            Direction::East => Position { col: self.col + 1, ..*self },
            Direction::West => Position { col: self.col - 1, ..*self },
        }
    }
    fn get_positions(&self, pipe: &char) -> [Position; 2] {
        match *pipe {
            '|' => [self.get_position(Direction::North), self.get_position(Direction::South)],
            '-' => [self.get_position(Direction::West), self.get_position(Direction::East)],
            'L' => [self.get_position(Direction::North), self.get_position(Direction::East)],
            'J' => [self.get_position(Direction::North), self.get_position(Direction::West)],
            '7' => [self.get_position(Direction::West), self.get_position(Direction::South)],
            'F' => [self.get_position(Direction::South), self.get_position(Direction::East)],
            _ => panic!("called get_positions of non-pipe character: {pipe}"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct VisitedBorder {
    pos: Position,
    direction: Direction,
}

impl VisitedBorder {
    fn get_positions(&self, pipe: &char) -> [VisitedBorder; 2] {
        match *pipe {
            '|' | '-' => {
                let new_pos = self.pos.get_positions(pipe);
                [VisitedBorder { pos: new_pos[0], ..*self }, VisitedBorder { pos: new_pos[1], ..*self }]
            }
            'L' => [
                VisitedBorder {
                    pos: self.pos.get_position(Direction::North),
                    direction: if self.direction == Direction::West || self.direction == Direction::South {
                        Direction::West
                    } else {
                        Direction::East
                    },
                },
                VisitedBorder {
                    pos: self.pos.get_position(Direction::East),
                    direction: if self.direction == Direction::West || self.direction == Direction::South {
                        Direction::South
                    } else {
                        Direction::North
                    },
                },
            ],
            'J' => [
                VisitedBorder {
                    pos: self.pos.get_position(Direction::North),
                    direction: if self.direction == Direction::South || self.direction == Direction::East {
                        Direction::East
                    } else {
                        Direction::West
                    },
                },
                VisitedBorder {
                    pos: self.pos.get_position(Direction::West),
                    direction: if self.direction == Direction::East || self.direction == Direction::South {
                        Direction::South
                    } else {
                        Direction::North
                    },
                },
            ],
            '7' => [
                VisitedBorder {
                    pos: self.pos.get_position(Direction::South),
                    direction: if self.direction == Direction::North || self.direction == Direction::East {
                        Direction::East
                    } else {
                        Direction::West
                    },
                },
                VisitedBorder {
                    pos: self.pos.get_position(Direction::West),
                    direction: if self.direction == Direction::North || self.direction == Direction::East {
                        Direction::North
                    } else {
                        Direction::South
                    },
                },
            ],
            'F' => [
                VisitedBorder {
                    pos: self.pos.get_position(Direction::South),
                    direction: if self.direction == Direction::North || self.direction == Direction::West {
                        Direction::West
                    } else {
                        Direction::East
                    },
                },
                VisitedBorder {
                    pos: self.pos.get_position(Direction::East),
                    direction: if self.direction == Direction::North || self.direction == Direction::West {
                        Direction::North
                    } else {
                        Direction::South
                    },
                },
            ],
            _ => panic!("called get_positions of non-pipe character: {pipe}"),
        }
    }
}

fn find_start_position(grid: &Vec<Vec<char>>) -> Option<Position> {
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, char) in row.iter().enumerate() {
            if char == &'S' {
                return Some(Position { row: row_index, col: column_index });
            }
        }
    }
    None
}

fn find_start_pipe(grid: &Vec<Vec<char>>, start: Position) -> char {
    fn get_pipe_at(grid: &Vec<Vec<char>>, pos: Position) -> char {
        *grid.get(pos.row).unwrap_or(&vec![]).get(pos.col).unwrap_or(&'.')
    }
    let north = if start.row > 0 { get_pipe_at(grid, start.get_position(Direction::North)) } else { '.' };
    let east = get_pipe_at(grid, start.get_position(Direction::East));
    let south = get_pipe_at(grid, start.get_position(Direction::South));
    if CONNECTED_FROM_NORTH.contains(&north) {
        if CONNECTED_FROM_EAST.contains(&east) {
            'L'
        } else if CONNECTED_FROM_SOUTH.contains(&south) {
            '|'
        } else {
            'J'
        }
    } else if CONNECTED_FROM_EAST.contains(&east) {
        if CONNECTED_FROM_SOUTH.contains(&south) {
            'F'
        } else {
            '-'
        }
    } else {
        '7'
    }
}

fn get_new_position(grid: &Vec<Vec<char>>, pos: Position, marked_positions: &HashSet<Position>) -> Option<Position> {
    let positions = pos.get_positions(&grid[pos.row][pos.col]);
    if !marked_positions.contains(&positions[0]) {
        Some(positions[0])
    } else if !marked_positions.contains(&positions[1]) {
        Some(positions[1])
    } else {
        None
    }
}

fn find_marked_positions(grid: &Vec<Vec<char>>, start: Position, start_pipe: &char) -> HashSet<Position> {
    let mut marked_positions = HashSet::new();
    marked_positions.insert(start);
    let mut next_positions = start.get_positions(start_pipe);
    loop {
        for i in 0..=1 {
            marked_positions.insert(next_positions[i]);
            match get_new_position(grid, next_positions[i], &marked_positions) {
                Some(pos) => next_positions[i] = pos,
                None => return marked_positions,
            }
        }
    }
}

fn challenge1(grid: &Vec<Vec<char>>) -> usize {
    let start = find_start_position(grid).expect("Could not find start position");
    let start_pipe = find_start_pipe(grid, start);
    let positions = find_marked_positions(grid, start, &start_pipe).len();
    if positions % 2 == 1 {
        (positions - 1) / 2
    } else {
        positions / 2
    }
}

#[derive(Debug)]
struct Cluster {
    is_outside: bool,
    positions: HashSet<Position>,
    border: HashSet<VisitedBorder>,
}

impl Cluster {
    fn new() -> Cluster {
        Cluster { is_outside: false, positions: HashSet::new(), border: HashSet::new() }
    }

    fn contains(&self, pos: &Position) -> bool {
        self.positions.contains(pos)
    }

    fn fill_cluster(&mut self, grid: &Vec<Vec<char>>, path: &HashSet<Position>, pos: Position, max_pos: &Position) {
        self.positions.insert(pos);

        for direction in DIRECTIONS {
            match pos.get_position_in_bounds(direction, max_pos) {
                Some(new_pos) => self._fill_cluster(path, new_pos, max_pos, direction.opposite()),
                None => self.is_outside = true,
            }
        }
        println!("Cluster contains {} items and is outside: {}", self.positions.len(), self.is_outside);
        println!("try sqeezing now");
        // walk along border
        let mut cur_borders: Vec<VisitedBorder> =
            vec![*self.border.iter().next().expect("a cluster must touch a border")];
        let mut walked_border: HashSet<Position> = HashSet::new();
        while !cur_borders.is_empty() {
            println!("cur borders: {cur_borders:?}, cluster size: {}", self.positions.len());
            walked_border.extend(cur_borders.iter().map(|border| border.pos));
            let mut next_borders: Vec<VisitedBorder> = vec![];
            for border in cur_borders {
                let new_borders = border.get_positions(&grid[border.pos.row][border.pos.col]);
                for new_border in new_borders {
                    if walked_border.contains(&new_border.pos) {
                        continue;
                    }
                    let potential_cluster_part = new_border.pos.get_position_in_bounds(new_border.direction, max_pos);
                    if potential_cluster_part.is_some() {
                        self._fill_cluster(
                            path,
                            potential_cluster_part.unwrap(),
                            max_pos,
                            new_border.direction.opposite(),
                        );
                    }
                    next_borders.push(new_border);
                }
            }
            cur_borders = next_borders;
        }
        println!("Cluster contains {} items and is outside: {}", self.positions.len(), self.is_outside);
    }

    fn _fill_cluster(&mut self, path: &HashSet<Position>, pos: Position, max_pos: &Position, from: Direction) {
        if self.contains(&pos) {
            return;
        }
        if path.contains(&pos) {
            self.border.insert(VisitedBorder { pos: pos, direction: from });
            return;
        }

        self.positions.insert(pos);

        for direction in DIRECTIONS {
            match pos.get_position_in_bounds(direction, max_pos) {
                Some(new_pos) => self._fill_cluster(path, new_pos, max_pos, direction.opposite()),
                None => self.is_outside = true,
            }
        }
    }
}

fn get_clusters(grid: &Vec<Vec<char>>, path: &HashSet<Position>, max_pos: &Position) -> Vec<Cluster> {
    let mut clusters: Vec<Cluster> = vec![];
    for row_id in 0..=max_pos.row {
        for col_id in 0..=max_pos.col {
            let pos = Position { row: row_id, col: col_id };
            if path.contains(&pos) || clusters.iter().any(|cluster| cluster.contains(&pos)) {
                continue;
            }
            let mut new_cluster = Cluster::new();
            new_cluster.fill_cluster(grid, &path, pos, &max_pos);
            clusters.push(new_cluster);
        }
    }
    clusters
}

fn challenge2(grid: &Vec<Vec<char>>) -> usize {
    let start = find_start_position(grid).expect("Could not find start position");
    let start_pipe = find_start_pipe(grid, start);
    let mut adjusted_grid = grid.clone();
    adjusted_grid[start.row][start.col] = start_pipe;
    let path = find_marked_positions(grid, start, &start_pipe);
    let max_pos = Position { row: grid.len() - 1, col: grid[0].len() - 1 };
    let clusters = get_clusters(&adjusted_grid, &path, &max_pos);
    clusters.iter().filter(|cluster| !cluster.is_outside).map(|cluster| cluster.positions.len()).sum()
    // 592 is too high
}

fn main() {
    let input: Vec<Vec<char>> = read_chars("data/2023/day10.txt");
    print_result(10, 1, challenge1, &input);
    print_result(10, 2, challenge2, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode_rust::to_chars;

    const CHALLENGE1_EXAMPLE1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";
    const CHALLENGE1_EXAMPLE2: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const CHALLENGE1_EXAMPLE3: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    const CHALLENGE1_EXAMPLE4: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1(&to_chars(CHALLENGE1_EXAMPLE1)), 4);
        assert_eq!(challenge1(&to_chars(CHALLENGE1_EXAMPLE2)), 4);
        assert_eq!(challenge1(&to_chars(CHALLENGE1_EXAMPLE3)), 8);
        assert_eq!(challenge1(&to_chars(CHALLENGE1_EXAMPLE4)), 8);
    }

    const CHALLENGE2_EXAMPLE1: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const CHALLENGE2_EXAMPLE2: &str = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
    const CHALLENGE2_EXAMPLE3: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    const CHALLENGE2_EXAMPLE4: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_challenge2_without_squeezing() {
        assert_eq!(challenge2(&to_chars(CHALLENGE2_EXAMPLE1)), 4);
    }

    #[test]
    fn test_challenge2_with_squeezing() {
        assert_eq!(challenge2(&to_chars(CHALLENGE2_EXAMPLE2)), 4);
        assert_eq!(challenge2(&to_chars(CHALLENGE2_EXAMPLE3)), 8);
        assert_eq!(challenge2(&to_chars(CHALLENGE2_EXAMPLE4)), 10);
    }
}
