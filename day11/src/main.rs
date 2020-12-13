use std::io;
use std::io::prelude::*;
use std::{fs::File, str::FromStr};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let board = input.parse::<Board>().expect("Cannot parse board");

    println!("part one: {:?}", part_one(board.clone()));
    println!("part two: {:?}", part_two(board));

    Ok(())
}

fn part_one(board: Board) -> usize {
    run_to_stabilizasion(board, 4, &adjacents).occupied_seat_count()
}

fn part_two(board: Board) -> usize {
    run_to_stabilizasion(board, 5, &first_see).occupied_seat_count()
}

fn run_to_stabilizasion(
    mut board: Board,
    tolerance: usize,
    neighbors_strategy: &dyn Fn(&Board, usize, usize) -> [(isize, isize); 8],
) -> Board {
    let mut next_state = board.clone();

    loop {
        next_state = round(&board, next_state, tolerance, neighbors_strategy);
        std::mem::swap(&mut next_state, &mut board);
        if board == next_state {
            break;
        }
    }

    board
}

#[derive(Clone, PartialEq)]
struct Board {
    data: Vec<Vec<u8>>,
    x_max: usize,
    y_max: usize,
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.lines().map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>();
        let y_max = data.len();
        let x_max = data[0].len();

        Ok(Board { data, x_max, y_max })
    }
}

impl Board {
    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if 0 <= x && x < (self.x_max as isize) && 0 <= y && y < (self.y_max as isize) {
            Some(self.data[y as usize][x as usize])
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, v: u8) {
        self.data[y][x] = v;
    }

    fn occupied_seat_count(&self) -> usize {
        self.data
            .iter()
            .map(|l| l.iter().filter(|&&position| position == b'#').count())
            .sum()
    }

    fn first_see(&self, mut x: isize, mut y: isize, dx: isize, dy: isize) -> (isize, isize) {
        loop {
            x += dx;
            y += dy;
            if let Some(position) = self.get(x, y) {
                if position == b'L' || position == b'#' {
                    return (x, y);
                }
            } else {
                return (x, y);
            }
        }
    }
}

fn adjacents(_board: &Board, x: usize, y: usize) -> [(isize, isize); 8] {
    let x = x as isize;
    let y = y as isize;

    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

fn first_see(board: &Board, x: usize, y: usize) -> [(isize, isize); 8] {
    let x = x as isize;
    let y = y as isize;

    [
        board.first_see(x, y, -1, -1),
        board.first_see(x, y, -1, 0),
        board.first_see(x, y, -1, 1),
        board.first_see(x, y, 0, -1),
        board.first_see(x, y, 0, 1),
        board.first_see(x, y, 1, -1),
        board.first_see(x, y, 1, 0),
        board.first_see(x, y, 1, 1),
    ]
}

fn round(
    current_state: &Board,
    mut next_state: Board,
    tolerance: usize,
    neighbors_strategy: &dyn Fn(&Board, usize, usize) -> [(isize, isize); 8],
) -> Board {
    for x in 0..current_state.x_max {
        for y in 0..current_state.y_max {
            if let Some(position) = current_state.get(x as isize, y as isize) {
                // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                if position == b'L'
                    && neighbors_strategy(&current_state, x, y)
                        .iter()
                        .filter_map(|(xn, yn)| current_state.get(*xn, *yn))
                        .all(|neighbor| neighbor == b'.' || neighbor == b'L')
                {
                    next_state.set(x, y, b'#');
                }
                // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty
                else if position == b'#'
                    && tolerance
                        <= neighbors_strategy(&current_state, x, y)
                            .iter()
                            .filter_map(|(xn, yn)| current_state.get(*xn, *yn))
                            .filter(|&neighbor| neighbor == b'#')
                            .count()
                {
                    next_state.set(x, y, b'L');
                }
                // Otherwise, the seat's state does not change.
                else {
                    next_state.set(x, y, position);
                }
            }
        }
    }

    next_state
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_tests() {
        let board = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"
            .parse::<Board>()
            .unwrap();
        assert_eq!(part_one(board), 37);
    }

    #[test]
    fn part_two_tests() {
        let board = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .parse::<Board>()
            .unwrap();
        assert_eq!(part_two(board), 26);
    }
}
