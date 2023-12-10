fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");
}

fn is_touching(lines: &Vec<String>, row: i32, col: i32, m: i32, n: i32) -> bool {
    return row < m && col < n && row >= 0 && col >= 0 && lines[row as usize].chars().nth(col as usize).unwrap() != '.' && 
        !lines[row as usize].chars().nth(col as usize).unwrap().is_digit(10);
}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();

    for (row, line) in lines.iter().enumerate() {
        let mut valid: bool = false;
        let mut num_str: String = "".to_owned();
        for (col, ch) in line.chars().enumerate() {
            if !ch.is_digit(10) {
                if valid {
                    res += num_str.parse::<u32>().unwrap();
                    // println!("{}", num_str.parse::<u32>().unwrap());
                }
                valid = false;
                num_str.clear();
            }
            else {
                num_str.push(ch);
                let row = row as i32;
                let col = col as i32;
                valid = valid | 
                    is_touching(&lines, row - 1, col - 1, lines.len() as i32, line.len() as i32) |
                    is_touching(&lines, row - 1, col, lines.len() as i32, line.len() as i32) |
                    is_touching(&lines, row - 1, col + 1, lines.len() as i32, line.len() as i32) |
                    is_touching(&lines, row + 1, col - 1, lines.len() as i32, line.len() as i32) |
                    is_touching(&lines, row + 1, col, lines.len() as i32, line.len() as i32) |
                    is_touching(&lines, row + 1, col + 1, lines.len() as i32, line.len() as i32) |
                    is_touching(&lines, row, col + 1, lines.len() as i32, line.len() as i32) |
                    is_touching(&lines, row, col - 1, lines.len() as i32, line.len() as i32);
            }
        }
        if valid {
            res += num_str.parse::<u32>().unwrap();
            // println!("{}", num_str.parse::<u32>().unwrap());
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part1(input);
        assert_eq!(result, 4361);
    }
}
