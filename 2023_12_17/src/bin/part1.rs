
fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");
}

fn get_board(input: &str) -> Vec<Vec<u32>> {
    let lines: Vec<_> = input.lines()
        .map(String::from)
        .collect();
    let board: Vec<Vec<u32>> = lines.iter()
        .map(|x| x.chars()
            .map(|ch| ch.to_digit(10).unwrap() as u32)
            .collect())
        .collect();
    return board;
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}


fn dfs(board: &Vec<Vec<u32>>, visited: &mut Vec<Vec<Vec<Vec<i64>>>>, dir: Direction, num_straight: u8, row: i32, col: i32, curr: u64, res: &mut u64) {
    if row < 0 || col < 0 || (col as usize) >= board[0].len() || (row as usize) >= board.len() || num_straight > 3 || curr + (board[row as usize][col as usize] as u64)>= *res {
        return; 
    }

    let curr = curr + board[row as usize][col as usize] as u64;
    if row as usize == (board.len() - 1) && col as usize == (board[0].len() - 1) {
        *res = curr;
        return;
    }
    if visited[num_straight as usize][dir.clone() as usize][row as usize][col as usize] > -1 {
        if (visited[num_straight as usize][dir.clone() as usize][row as usize][col as usize] as u64) <= curr {
            return;
        }
    }
    visited[num_straight as usize][dir.clone() as usize][row as usize][col as usize] = curr as i64;
    if Direction::Left == dir.clone() {
        dfs(board, visited, Direction::Up, 1, row - 1, col, curr, res);
        dfs(board, visited, Direction::Down, 1, row + 1, col, curr, res);
        dfs(board, visited, Direction::Left, num_straight + 1, row, col - 1, curr, res);
    }
    if Direction::Right == dir.clone() { 
        dfs(board, visited, Direction::Right, num_straight + 1, row, col + 1, curr, res);
        dfs(board, visited, Direction::Down, 1, row + 1, col, curr, res);
        dfs(board, visited, Direction::Up, 1, row - 1, col, curr, res);
    }
    if Direction::Up == dir.clone() {
        dfs(board, visited, Direction::Right, 1, row, col + 1, curr, res);
        dfs(board, visited, Direction::Left, 1, row, col - 1, curr, res);
        dfs(board, visited, Direction::Up, num_straight + 1, row - 1, col, curr, res);
    }
    if Direction::Down == dir.clone() {
        dfs(board, visited, Direction::Right, 1, row, col + 1, curr, res);
        dfs(board, visited, Direction::Down, num_straight + 1, row + 1, col, curr, res);
        dfs(board, visited, Direction::Left, 1, row, col - 1, curr, res);
    }
}

fn part1(input: &str) -> u64 {
    let board = get_board(input);
    let mut res = 9999999999;
    let mut visited: Vec<Vec<Vec<Vec<i64>>>> = vec![vec![vec![vec![-1; board[0].len()]; board.len()]; 5]; 5];
    dfs(&board, &mut visited, Direction::Right, 0, 0, 0, 0, &mut res);
    dfs(&board, &mut visited, Direction::Down, 0, 0, 0, 0, &mut res);
    return res - board[0][0] as u64;

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let result1 = part1(input1);
        assert_eq!(result1, 102);
    }
}
