
fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("{result}");

}

fn is_row_symmetric(row: &String, col: usize) -> u32 {
    let mut left: i32 = col as i32;
    let mut right: i32 = (col + 1) as i32;
    let mut mismatches: u32 = 0;
    while left >= 0 && (right as usize) < row.len() {
        if row.chars().nth( left as usize ) != row.chars().nth( right as usize ) {
            if mismatches > 0 {
                return 2;
            }
            mismatches = 1;
        }
        left -= 1;
        right += 1;
    }
    return mismatches;
}

fn is_col_symmetric(puzzle: &Vec<String>, row: usize, col: usize) -> u32 {
    let mut top: i32 = row as i32;
    let mut bot: i32 = (row + 1) as i32;
    let mut mismatches: u32 = 0;
    while top >= 0 && (bot as usize) < puzzle.len() {
        if puzzle[top as usize].chars().nth(col).unwrap() != puzzle[bot as usize].chars().nth(col).unwrap() {
            if mismatches > 0 {
                return 2;
            }
            mismatches += 1;
        }
        top -= 1;
        bot += 1;
    }
    return mismatches;
}

fn process_puzzle(puzzle: &Vec<String>) -> u64 {
    let mut col_failed: Vec<u32> = vec![0; puzzle[0].len()-1];
    let mut row_failed: Vec<u32> = vec![0; puzzle.len()-1];
    for line in puzzle.iter() {
        for (col, _) in line.chars().enumerate() {
            if col >= col_failed.len() || col_failed[col] >= 2 {
                continue;
            }
            col_failed[col] += is_row_symmetric(line, col);
        }
    }
    let pos = col_failed.iter().position(|x| *x == 1);
    if pos.is_some() {
        return (pos.unwrap() + 1) as u64;
    }
    for col in 0..puzzle[0].len() {
        for row in 0..puzzle.len() {
            if row >= row_failed.len() || row_failed[row] >= 2 {
                continue;
            }
            row_failed[row] += is_col_symmetric(puzzle, row, col);
        }
    }
    let pos = row_failed.iter().position(|x| *x == 1);
    if pos.is_some() {
        return ((pos.unwrap() + 1) as u64) * 100;
    }
    return 0;
}

fn part2(input: &str) -> u64 {
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
        let result1 = part2(input1);
        assert_eq!(result1, 400);
    }
}
