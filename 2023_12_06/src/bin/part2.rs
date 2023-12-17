
fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("{result}");
}

fn get_vals(line: &String) -> u64 {
    return line.split(':')
        .map(|s| s.trim())
        .collect::<Vec<_>>()[1]
        .chars()
        .filter(|x| *x != ' ')
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
}

fn get_ways_to_win(time: u64, distance: u64) -> u64 {
    let mut res = 0;
    for i in 1..time {
        let my_dist = i * (time - i);
        if my_dist > distance {
            res += 1;
        }
    }
    return res;
}

fn part2(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let time = get_vals(&lines[0]);
    let distance = get_vals(&lines[1]);
    return get_ways_to_win(time, distance);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"Time:      7  15   30
Distance:  9  40  200";
        let result1 = part2(input1);
        assert_eq!(result1, 71503);
    }
}
