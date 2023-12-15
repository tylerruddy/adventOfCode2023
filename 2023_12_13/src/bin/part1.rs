
fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");

}

fn is_row_symmetric(row: &String, col: usize) -> bool {
    let mut left: i32 = col as i32;
    let mut right: i32 = (col + 1) as i32;
    while left >= 0 && (right as usize) < row.len() {
        if row.chars().nth( left as usize ) != row.chars().nth( right as usize ) {
            return false;
        }
        left -= 1;
        right += 1;
    }
    return true;
}

fn is_col_symmetric(puzzle: &Vec<String>, row: usize, col: usize) -> bool {
    let mut top: i32 = row as i32;
    let mut bot: i32 = (row + 1) as i32;
    while top >= 0 && (bot as usize) < puzzle.len() {
        if puzzle[top as usize].chars().nth(col).unwrap() != puzzle[bot as usize].chars().nth(col).unwrap() {
            return false;
        }
        top -= 1;
        bot += 1;
    }
    return true;
}

fn process_puzzle(puzzle: &Vec<String>) -> u64 {
    let mut col_failed: Vec<bool> = vec![false; puzzle[0].len()-1];
    let mut row_failed: Vec<bool> = vec![false; puzzle.len()-1];
    for line in puzzle.iter() {
        for (col, _) in line.chars().enumerate() {
            if col >= col_failed.len() ||col_failed[col] {
                continue;
            }
            if !is_row_symmetric(line, col) {
                col_failed[col] = true;
            }
        }
    }
    let pos = col_failed.iter().position(|x| !*x);
    if pos.is_some() {
        return (pos.unwrap() + 1) as u64;
    }
    for col in 0..puzzle[0].len() {
        for row in 0..puzzle.len() {
            if row >= row_failed.len() || row_failed[row] {
                continue;
            }
            if !is_col_symmetric(puzzle, row, col) {
                row_failed[row] = true;
            }
        }
    }
    let pos = row_failed.iter().position(|x| !*x);
    if pos.is_some() {
        return ((pos.unwrap() + 1) as u64) * 100;
    }
    return 0;
}

fn part1(input: &str) -> u64 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let mut puzzle: Vec<String> = Default::default();
    for s in lines.iter() {
        if s.len() == 0 {
            res += process_puzzle(&puzzle);
            puzzle.clear();
        }
        else {
            puzzle.push(s.to_string());
        }
    }
    res += process_puzzle(&puzzle);
    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let result1 = part1(input1);
        assert_eq!(result1, 405);
    }
}
