use std::collections::{HashMap, HashSet};

fn input() -> String {
    std::fs::read_to_string("input/input_22.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents, 50);

    val.to_string()
}

fn a_with_input(input: &str, square_width: usize) -> usize {
    let (map, directions) = parse::parse_input(input, square_width);

    let mut y = 0;
    let mut x = map.rows[0].x_offset;
    while map.rows[y].tiles[x - map.rows[0].x_offset] != Tile::Empty {
        x += 1;
    }

    let mut facing = Facing::R;

    for dir in directions {
        match dir {
            Direction::RTurn => {
                facing = match facing {
                    Facing::R => Facing::D,
                    Facing::D => Facing::L,
                    Facing::L => Facing::U,
                    Facing::U => Facing::R,
                }
            }
            Direction::LTurn => {
                facing = match facing {
                    Facing::R => Facing::U,
                    Facing::U => Facing::L,
                    Facing::L => Facing::D,
                    Facing::D => Facing::R,
                }
            }
            Direction::Forward(amt) => {
                for _ in 0..amt {
                    let (next_x, next_y, next_facing) = match facing {
                        Facing::R => map.right_part_a(x, y),
                        Facing::L => map.left_part_a(x, y),
                        Facing::U => map.up_y_part_a(x, y),
                        Facing::D => map.down_y_part_a(x, y),
                    };

                    let row: &Row = &map.rows[next_y];
                    let tile = row.tiles[next_x - row.x_offset];

                    match tile {
                        Tile::Empty => {
                            x = next_x;
                            y = next_y;
                            facing = next_facing;
                        }
                        Tile::Filled => {
                            // stop moving, done
                            break;
                        }
                    }
                }
            }
        }
    }

    let mut out = 0;

    out += 1000 * (y + 1) + 4 * (x + 1);

    out += match facing {
        Facing::R => 0,
        Facing::D => 1,
        Facing::L => 2,
        Facing::U => 3,
    };

    out
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents, 50);

    val.to_string()
}

fn b_with_input(input: &str, square_width: usize) -> usize {
    let (map, directions) = parse::parse_input(input, square_width);

    let mut y = 0;
    let mut x = map.rows[0].x_offset;
    while map.rows[y].tiles[x - map.rows[0].x_offset] != Tile::Empty {
        x += 1;
    }

    let mut facing = Facing::R;

    for dir in directions {
        match dir {
            Direction::RTurn => {
                facing = match facing {
                    Facing::R => Facing::D,
                    Facing::D => Facing::L,
                    Facing::L => Facing::U,
                    Facing::U => Facing::R,
                }
            }
            Direction::LTurn => {
                facing = match facing {
                    Facing::R => Facing::U,
                    Facing::U => Facing::L,
                    Facing::L => Facing::D,
                    Facing::D => Facing::R,
                }
            }
            Direction::Forward(amt) => {
                for _ in 0..amt {
                    let (next_x, next_y, next_facing) = match facing {
                        Facing::R => map.right_part_b(x, y),
                        Facing::L => map.left_part_b(x, y),
                        Facing::U => map.up_y_part_b(x, y),
                        Facing::D => map.down_y_part_b(x, y),
                    };

                    let row: &Row = &map.rows[next_y];
                    let tile = row.tiles[next_x - row.x_offset];

                    match tile {
                        Tile::Empty => {
                            x = next_x;
                            y = next_y;
                            facing = next_facing;
                        }
                        Tile::Filled => {
                            // stop moving, done
                            break;
                        }
                    }
                }
            }
        }
    }

    let mut out = 0;

    out += 1000 * (y + 1) + 4 * (x + 1);

    out += match facing {
        Facing::R => 0,
        Facing::D => 1,
        Facing::L => 2,
        Facing::U => 3,
    };

    out
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Map {
    rows: Vec<Row>,
    square_width: usize,
    // in square coordinates; so if square_width is 50 and your actual coordinate is 83, 12,
    // then you'll look at square coordinates (83 // 50, 12 // 50), which is (1, 0)
    edge_connections: HashMap<(usize, usize, FaceEdge), (usize, usize, FaceEdge)>,
}

fn make_edge_connections(
    squares_filled: HashSet<(usize, usize)>,
) -> HashMap<(usize, usize, FaceEdge), (usize, usize, FaceEdge)> {
    assert_eq!(squares_filled.len(), 6);

    let mut edge_connections = HashMap::new();

    let symmetric_add = |map: &mut HashMap<_, _>, a, b| {
        map.insert(a, b);
        map.insert(b, a);
    };

    for (x, y) in squares_filled.iter().copied() {
        if x > 0 && squares_filled.contains(&(x - 1, y)) {
            symmetric_add(
                &mut edge_connections,
                (x, y, FaceEdge::XMin),
                (x - 1, y, FaceEdge::XMax),
            );
        }

        if squares_filled.contains(&(x + 1, y)) {
            symmetric_add(
                &mut edge_connections,
                (x, y, FaceEdge::XMax),
                (x + 1, y, FaceEdge::XMin),
            );
        }

        if y > 0 && squares_filled.contains(&(x, y - 1)) {
            symmetric_add(
                &mut edge_connections,
                (x, y, FaceEdge::YMin),
                (x, y - 1, FaceEdge::YMax),
            );
        }

        if squares_filled.contains(&(x, y + 1)) {
            symmetric_add(
                &mut edge_connections,
                (x, y, FaceEdge::YMax),
                (x, y + 1, FaceEdge::YMin),
            );
        }
    }

    while edge_connections.len() < 24 {
        let old_count = edge_connections.len();

        for (x, y) in squares_filled.iter().copied() {
            // basically if we have a corner coming off this square, we can infer the rest
            for edge in [
                FaceEdge::XMin,
                FaceEdge::XMax,
                FaceEdge::YMin,
                FaceEdge::YMax,
            ] {
                let adj_edge = edge.right();
                match (
                    edge_connections.get(&(x, y, edge)).copied(),
                    edge_connections.get(&(x, y, adj_edge)).copied(),
                ) {
                    (Some((x_a, y_a, edge_a)), Some((x_b, y_b, edge_b))) => {
                        let adj_edge_a = edge_a.left();
                        let adj_edge_b = edge_b.right();

                        symmetric_add(
                            &mut edge_connections,
                            (x_a, y_a, adj_edge_a),
                            (x_b, y_b, adj_edge_b),
                        );
                    }
                    // if we don't have both, do nothing
                    _ => {}
                }
            }
        }

        let new_count = edge_connections.len();
        if new_count <= old_count {
            panic!("Edge map is not full, but our method did not find more edges");
        }
    }

    assert_eq!(edge_connections.len(), 24);

    edge_connections
}

impl Map {
    fn new(rows: Vec<Row>, square_width: usize) -> Map {
        let mut squares_filled = HashSet::new();
        for y in 0..rows.len() {
            let row: &Row = &rows[y];
            for x in 0..row.tiles.len() {
                squares_filled.insert(((x + row.x_offset) / square_width, y / square_width));
            }
        }
        let edge_connections = make_edge_connections(squares_filled);
        Map {
            rows,
            square_width,
            edge_connections,
        }
    }
}

// part A
impl Map {
    // wrapping
    // PRE: position is valid
    fn up_y_part_a(&self, x: usize, mut y: usize) -> (usize, usize, Facing) {
        for _attempt in 0..self.rows.len() {
            y = if y == 0 { self.rows.len() - 1 } else { y - 1 };
            if self.rows[y].contains_x(x) {
                break;
            }
        }
        (x, y, Facing::U)
    }

    // wrapping
    // PRE: position is valid
    fn down_y_part_a(&self, x: usize, mut y: usize) -> (usize, usize, Facing) {
        for _attempt in 0..self.rows.len() {
            y = if y >= self.rows.len() - 1 { 0 } else { y + 1 };
            if self.rows[y].contains_x(x) {
                break;
            }
        }
        (x, y, Facing::D)
    }

    // wrapping
    // PRE: position is valid
    fn right_part_a(&self, x: usize, y: usize) -> (usize, usize, Facing) {
        let row = &self.rows[y];
        if x >= row.x_max() - 1 {
            (row.x_offset, y, Facing::R)
        } else {
            (x + 1, y, Facing::R)
        }
    }

    // wrapping
    // PRE: position is valid
    fn left_part_a(&self, x: usize, y: usize) -> (usize, usize, Facing) {
        let row = &self.rows[y];
        if x == row.x_offset {
            (row.x_offset + row.tiles.len() - 1, y, Facing::L)
        } else {
            (x - 1, y, Facing::L)
        }
    }
}

// part B
impl Map {
    fn get_connection(
        &self,
        mut local_x: usize,
        mut local_y: usize,
        mut local_facing: Facing,
        mut local_edge: FaceEdge,
        new_square_x: usize,
        new_square_y: usize,
        new_edge: FaceEdge,
    ) -> (usize, usize, Facing) {
        let square_max = self.square_width - 1;

        while local_edge != new_edge {
            match local_edge {
                FaceEdge::XMin => {
                    assert_eq!(local_facing, Facing::R);
                    assert_eq!(local_x, 0);
                    local_x = square_max - local_y;
                    local_y = 0;
                    local_edge = FaceEdge::YMin;
                    local_facing = Facing::D;
                }
                FaceEdge::XMax => {
                    assert_eq!(local_facing, Facing::L);
                    assert_eq!(local_x, square_max);
                    local_x = square_max - local_y;
                    local_y = square_max;
                    local_edge = FaceEdge::YMax;
                    local_facing = Facing::U;
                }
                FaceEdge::YMin => {
                    assert_eq!(local_facing, Facing::D);
                    assert_eq!(local_y, 0);
                    local_y = local_x;
                    local_x = square_max;
                    local_edge = FaceEdge::XMax;
                    local_facing = Facing::L;
                }
                FaceEdge::YMax => {
                    assert_eq!(local_facing, Facing::U);
                    assert_eq!(local_y, square_max);
                    local_y = local_x;
                    local_x = 0;
                    local_edge = FaceEdge::XMin;
                    local_facing = Facing::R;
                }
            }
        }
        (
            local_x + (new_square_x * self.square_width),
            local_y + (new_square_y * self.square_width),
            local_facing,
        )
    }

    // wrapping
    // PRE: position is valid
    fn up_y_part_b(&self, x: usize, y: usize) -> (usize, usize, Facing) {
        if y > 0 && self.rows[y - 1].contains_x(x) {
            (x, y - 1, Facing::U)
        } else {
            let square_pos = (x / self.square_width, y / self.square_width, FaceEdge::YMin);
            let (new_square_x, new_square_y, new_edge) =
                self.edge_connections.get(&square_pos).copied().unwrap();
            let square_max = self.square_width - 1;

            // appear on the bottom (YMax) side of the square, then rotate coordinates
            // until we're set
            self.get_connection(
                x % self.square_width,
                square_max,
                Facing::U,
                FaceEdge::YMax,
                new_square_x,
                new_square_y,
                new_edge,
            )
        }
    }

    // wrapping
    // PRE: position is valid
    fn down_y_part_b(&self, x: usize, y: usize) -> (usize, usize, Facing) {
        if y + 1 < self.rows.len() && self.rows[y + 1].contains_x(x) {
            (x, y + 1, Facing::D)
        } else {
            let square_pos = (x / self.square_width, y / self.square_width, FaceEdge::YMax);
            let (new_square_x, new_square_y, new_edge) =
                self.edge_connections.get(&square_pos).copied().unwrap();

            // appear on the top (YMin) side of the square, then rotate coordinates until we're set
            self.get_connection(
                x % self.square_width,
                0,
                Facing::D,
                FaceEdge::YMin,
                new_square_x,
                new_square_y,
                new_edge,
            )
        }
    }

    // wrapping
    // PRE: position is valid
    fn right_part_b(&self, x: usize, y: usize) -> (usize, usize, Facing) {
        let row = &self.rows[y];
        if x >= row.x_max() - 1 {
            let square_pos = (x / self.square_width, y / self.square_width, FaceEdge::XMax);
            let (new_square_x, new_square_y, new_edge) =
                self.edge_connections.get(&square_pos).copied().unwrap();

            // appear on the left (XMin) side of the square, then rotate coordinates until we're set
            self.get_connection(
                0,
                y % self.square_width,
                Facing::R,
                FaceEdge::XMin,
                new_square_x,
                new_square_y,
                new_edge,
            )
        } else {
            (x + 1, y, Facing::R)
        }
    }

    // wrapping
    // PRE: position is valid
    fn left_part_b(&self, x: usize, y: usize) -> (usize, usize, Facing) {
        let row = &self.rows[y];
        if x == row.x_offset {
            let square_pos = (x / self.square_width, y / self.square_width, FaceEdge::XMin);
            let (new_square_x, new_square_y, new_edge) =
                self.edge_connections.get(&square_pos).copied().unwrap();

            // appear on the right (XMax) side of the square, then rotate coordinates until we're set
            self.get_connection(
                self.square_width - 1,
                y % self.square_width,
                Facing::L,
                FaceEdge::XMax,
                new_square_x,
                new_square_y,
                new_edge,
            )
        } else {
            (x - 1, y, Facing::L)
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Row {
    tiles: Vec<Tile>,
    x_offset: usize,
}

impl Row {
    // exclusive
    fn x_max(&self) -> usize {
        self.x_offset + self.tiles.len()
    }

    fn contains_x(&self, x: usize) -> bool {
        x >= self.x_offset && x < self.x_max()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    Filled,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Forward(usize),
    RTurn,
    LTurn,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Facing {
    R,
    U,
    L,
    D,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum FaceEdge {
    XMin,
    XMax,
    YMin,
    YMax,
}

impl FaceEdge {
    // If you were on the face, looking at this edge, which edge is to your left?
    #[inline(always)]
    fn left(self) -> Self {
        match self {
            FaceEdge::XMin => FaceEdge::YMax,
            FaceEdge::YMax => FaceEdge::XMax,
            FaceEdge::XMax => FaceEdge::YMin,
            FaceEdge::YMin => FaceEdge::XMin,
        }
    }

    #[inline(always)]
    fn right(self) -> Self {
        // easier than checking two tables manually
        // LLVM should inline this and turn it into a single lookup, don't @ me
        self.left().left().left()
    }
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, space0},
        combinator::{eof, map},
        multi::many1,
        IResult,
    };

    use super::{Direction, Map, Row, Tile};

    fn parse_num(input: &str) -> IResult<&str, usize> {
        let (input, val) = map(digit1, |digits: &str| digits.parse::<usize>().unwrap())(input)?;

        Ok((input, val))
    }

    fn parse_tile(input: &str) -> IResult<&str, Tile> {
        alt((
            map(tag("."), |_| Tile::Empty),
            map(tag("#"), |_| Tile::Filled),
        ))(input)
    }

    fn parse_row(input: &str) -> Row {
        fn helper(input: &str) -> IResult<&str, Row> {
            let (input, x_offset) = map(space0, |spaces: &str| spaces.len())(input)?;
            let (input, tiles) = many1(parse_tile)(input)?;
            let (_, _) = eof(input)?;
            Ok(("", Row { x_offset, tiles }))
        }

        helper(input).unwrap().1
    }

    fn parse_turn(input: &str) -> IResult<&str, Direction> {
        alt((
            map(tag("L"), |_| Direction::LTurn),
            map(tag("R"), |_| Direction::RTurn),
            map(parse_num, |num| Direction::Forward(num)),
        ))(input)
    }

    fn parse_directions(input: &str) -> Vec<Direction> {
        fn helper(input: &str) -> IResult<&str, Vec<Direction>> {
            let (input, dirs) = many1(parse_turn)(input)?;
            let (_, _) = eof(input)?;
            Ok(("", dirs))
        }

        helper(input).unwrap().1
    }

    pub(super) fn parse_input(input: &str, square_width: usize) -> (Map, Vec<Direction>) {
        let mut lines = input.lines().peekable();

        let rows: Vec<Row> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| parse_row(line))
            .collect();

        // the above consumes the empty line without passing it to the mapper

        let directions = parse_directions(lines.next().unwrap());

        assert_eq!(lines.next(), None);

        (Map::new(rows, square_width), directions)
    }

    #[cfg(test)]
    mod parse_test {
        use super::*;

        #[test]
        fn parse_row_test() {
            assert_eq!(
                parse_row("        #..."),
                Row {
                    x_offset: 8,
                    tiles: vec![Tile::Filled, Tile::Empty, Tile::Empty, Tile::Empty]
                }
            );
        }

        #[test]
        fn parse_dir_test() {
            assert_eq!(
                parse_directions("10R5L"),
                vec![
                    Direction::Forward(10),
                    Direction::RTurn,
                    Direction::Forward(5),
                    Direction::LTurn
                ]
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input, 4);
        assert_eq!(actual, 6032);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input, 4);
        assert_eq!(actual, 5031);
    }
}
