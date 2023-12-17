
fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");
}

fn get_vals(line: &String) -> Vec<u64> {
    return line.split(':')
        .map(|s| s.trim())
        .collect::<Vec<_>>()[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();
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

fn part1(input: &str) -> u64 {
    let mut res = 1;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let times = get_vals(&lines[0]);
    let distances = get_vals(&lines[1]);
    for race in 0..times.len() {
        res *= get_ways_to_win(times[race], distances[race]);
    }
    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"Time:      7  15   30
Distance:  9  40  200";
        let result1 = part1(input1);
        assert_eq!(result1, 288);
    }
}
