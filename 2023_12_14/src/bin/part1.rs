fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");

}

fn load_in_column(lines: &Vec<String>, col: usize) -> u64 {
    let mut res: u64 = 0;
    let mut next_score: u64 = lines.len() as u64;
    for (row, line) in lines.iter().enumerate() {
        if line.chars().nth(col).unwrap() == '#' {
            next_score = (lines.len() as u64) - (row as u64) - 1;
        }
        else if line.chars().nth(col).unwrap() == 'O' {
            res += next_score;
            next_score -= 1;
        }
    }
    return res;
}

fn part1(input: &str) -> u64 {
    let mut res: u64 = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    for col in 0..lines[0].len() {
        res += load_in_column(&lines, col);
    }
    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let result1 = part1(input1);
        assert_eq!(result1, 136);
    }
}
