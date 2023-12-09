
fn part2(input: &str) -> i32 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let look_ups: Vec<(&str, &str)> = [ 
        ("1", "one"), 
        ("2", "two"), 
        ("3", "three"), 
        ("4", "four"), 
        ("5", "five"), 
        ("6", "six"), 
        ("7", "seven"), 
        ("8", "eight"), 
        ("9", "nine")
    ].to_vec();
    for line in lines.iter() {
        let mut left = "1";
        let mut right = line.chars().next().unwrap().to_string();
        let mut l_ptr = 1000;
        let mut r_ptr = 0;
        for p in look_ups.iter() {
            let a: Vec<_> = line.match_indices(p.0).collect();
            let b: Vec<_> = line.match_indices(p.1).collect();
            if a.len() > 0 && a.first().unwrap().0 < l_ptr {
                l_ptr = a.first().unwrap().0;
                left = p.0;
            }
            if b.len() > 0 && b.first().unwrap().0 < l_ptr {
                l_ptr = b.first().unwrap().0;
                left = p.0;
            }
            if a.len() > 0 && a.last().unwrap().0 > r_ptr {
                r_ptr = a.last().unwrap().0;
                right = p.0.to_string();
            }
            if b.len() > 0 && b.last().unwrap().0 > r_ptr {
                r_ptr = b.last().unwrap().0;
                right = p.0.to_string();
            }
        }
        let mut string_num = String::from(left);
        string_num.push_str(&(String::from(right)));
        println!("{}, {}", string_num.parse::<i32>().unwrap(), line);
        res += string_num.parse::<i32>().unwrap();
    }
    return res;
}

fn main() {
    println!("part2");
    let input = include_str!("./input2.txt");
    let result = part2(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input = 
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        let result = part2(input);
        assert_eq!(result, 281);
    }
}
