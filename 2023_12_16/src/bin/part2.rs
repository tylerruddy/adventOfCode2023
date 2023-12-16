extern crate queues;
use queues::*;

fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    //7679 -> too low
    println!("{result}");
}

fn get_board(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<_> = input.lines()
        .map(String::from)
        .collect();
    let board: Vec<Vec<char>> = lines.iter()
        .map(|x| x.chars()
            .collect())
        .collect();
    return board;
}

#[repr(u8)]
#[derive(Clone)]
enum Direction {
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
}

fn try_to_add(_pos: &mut u8, dir: Direction, new_x: i32, new_y: i32, q: &mut Queue<(i32, i32, Direction)>) {
    let _ = q.add((new_x, new_y, dir));
}

fn traverse_board(board: &Vec<Vec<char>>, dp: &mut Vec<Vec<u8>>, start_row: i32, start_col: i32, start_dir: Direction) {
    let mut q: Queue<(i32, i32, Direction)> = queue![];
    let _ = q.add((start_row, start_col, start_dir));
    while q.size() > 0 {
        let (x, y, dir) = q.remove().unwrap();
        if x < 0 || y < 0 || (x as usize) >= dp.len() || (y as usize) >= dp[0].len() || 
            (dp[x as usize][y as usize] & (dir.clone() as u8)) != 0 {
            continue;
        }
        dp[x as usize][y as usize] |= dir.clone() as u8;
        match dir {
            Direction::Left => {
                if board[x as usize][y as usize] == '.' || board[x as usize][y as usize] == '-' {
                    let _ = q.add((x, y - 1, Direction::Left));
                }
                else if board[x as usize][y as usize] == '/' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Down, x + 1, y, &mut q);
                }
                else if board[x as usize][y as usize] == '\\' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Up, x - 1, y, &mut q);
                }
                else if board[x as usize][y as usize] == '|' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Up, x - 1, y, &mut q);
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Down, x + 1, y, &mut q);
                }
                continue;
            },
            Direction::Right => {
                if board[x as usize][y as usize] == '.' || board[x as usize][y as usize] == '-' {
                    let _ = q.add((x, y + 1, Direction::Right));
                }
                else if board[x as usize][y as usize] == '/' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Up, x - 1, y, &mut q);
                }
                else if board[x as usize][y as usize] == '\\' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Down, x + 1, y, &mut q);
                }
                else if board[x as usize][y as usize] == '|' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Up, x - 1, y, &mut q);
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Down, x + 1, y, &mut q);
                }
                continue;
            },
            Direction::Up => {
                if board[x as usize][y as usize] == '.' || board[x as usize][y as usize] == '|' {
                    let _ = q.add((x - 1, y, Direction::Up));
                }
                else if board[x as usize][y as usize] == '/' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Right, x, y + 1, &mut q);
                }
                else if board[x as usize][y as usize] == '\\' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Left, x, y - 1, &mut q);
                }
                else if board[x as usize][y as usize] == '-' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Left, x, y - 1, &mut q);
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Right, x, y + 1, &mut q);
                }
                continue;
            },
            Direction::Down => {
                if board[x as usize][y as usize] == '.' || board[x as usize][y as usize] == '|' {
                    let _ = q.add((x + 1, y, Direction::Down));
                }
                else if board[x as usize][y as usize] == '/' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Left, x, y - 1, &mut q);
                }
                else if board[x as usize][y as usize] == '\\' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Right, x, y + 1, &mut q);
                }
                else if board[x as usize][y as usize] == '-' {
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Left, x, y - 1, &mut q);
                    try_to_add(&mut dp[x as usize][y as usize], Direction::Right, x, y + 1, &mut q);
                }
                continue;
            },
        }
    }
}

fn count_energized(board: &Vec<Vec<u8>>) -> u64 {
    return board.iter().map(|v| v.iter()
                            .map(|x| (*x > 0) as u64)
                            .sum::<u64>())
        .sum();
}

fn part2(input: &str) -> u64 {
    let mut res: u64 = 0;
    let board = get_board(input);
    for col in 0..board[0].len() {
        {
            let mut dp: Vec<Vec<u8>> = vec![vec![0; board[0].len()]; board.len()];
            traverse_board(&board, &mut dp, 0, col as i32, Direction::Down);
            res = std::cmp::max(res, count_energized(&dp));
        }
        {
            let mut dp: Vec<Vec<u8>> = vec![vec![0; board[0].len()]; board.len()];
            traverse_board(&board, &mut dp, (board.len() - 1) as i32, col as i32, Direction::Up);
            res = std::cmp::max(res, count_energized(&dp));
        }
    }
    for row in 0..board.len() {
        {
            let mut dp: Vec<Vec<u8>> = vec![vec![0; board[0].len()]; board.len()];
            traverse_board(&board, &mut dp, row as i32, 0, Direction::Right);
            res = std::cmp::max(res, count_energized(&dp));
        }
        {
            let mut dp: Vec<Vec<u8>> = vec![vec![0; board[0].len()]; board.len()];
            traverse_board(&board, &mut dp, row as i32, (board[0].len() - 1) as i32, Direction::Left);
            res = std::cmp::max(res, count_energized(&dp));
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let result1 = part2(input1);
        assert_eq!(result1, 51);
    }
}
