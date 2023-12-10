use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("{result}");
}

fn is_touching(lines: &Vec<String>, row: i32, col: i32, m: i32, n: i32, st: &mut HashSet<(i32, i32)>) -> bool {
    let touching: bool = row < m && col < n && row >= 0 && col >= 0 && 
        lines[row as usize].chars().nth(col as usize).unwrap() == '*';
    if touching {
        st.insert((row, col));
    }
    return touching;
}

fn part2(input: &str) -> u32 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let mut ht: HashMap<(i32, i32), Vec<u32>> = Default::default();

    for (row, line) in lines.iter().enumerate() {
        let mut valid: bool = false;
        let mut num_str: String = "".to_owned();
        let mut st: HashSet<(i32, i32)> = Default::default();
        for (col, ch) in line.chars().enumerate() {
            if !ch.is_digit(10) {
                if valid {
                    let num = num_str.parse::<u32>().unwrap();
                    for p in st.iter() {
                        if !ht.contains_key(&p) {
                            ht.insert(*p, Vec::new());
                        }
                        ht.get_mut(&p).expect("Key needs to exist").push(num);
                    }
                }
                valid = false;
                num_str.clear();
                st.clear();
            }
            else {
                num_str.push(ch);
                let row = row as i32;
                let col = col as i32;
                valid = valid | 
                    is_touching(&lines, row - 1, col - 1, lines.len() as i32, line.len() as i32, &mut st) |
                    is_touching(&lines, row - 1, col, lines.len() as i32, line.len() as i32, &mut st) |
                    is_touching(&lines, row - 1, col + 1, lines.len() as i32, line.len() as i32, &mut st) |
                    is_touching(&lines, row + 1, col - 1, lines.len() as i32, line.len() as i32, &mut st) |
                    is_touching(&lines, row + 1, col, lines.len() as i32, line.len() as i32, &mut st) |
                    is_touching(&lines, row + 1, col + 1, lines.len() as i32, line.len() as i32, &mut st) |
                    is_touching(&lines, row, col + 1, lines.len() as i32, line.len() as i32, &mut st) |
                    is_touching(&lines, row, col - 1, lines.len() as i32, line.len() as i32, &mut st);
            }
        }
        if valid {
            let num = num_str.parse::<u32>().unwrap();
            for p in st.iter() {
                if !ht.contains_key(&p) {
                    ht.insert(*p, Vec::new());
                }
                ht.get_mut(&p).expect("Key needs to exist").push(num);
            }
        }
    }

    for (_, vec) in ht.iter() {
        if vec.len() == 2 {
            res += vec[0] * vec[1];
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
        let result = part2(input);
        assert_eq!(result, 467835);
    }
}
