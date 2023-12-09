
fn part1(input: &str) -> i32 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    for line in lines.iter() {
        let _nums = line.chars().enumerate().filter(|(_, ch)| *ch >= '0' && *ch <= '9').map(|(_, c)| c).collect::<Vec<_>>();
        // println!("{:?}", _nums);
        let mut string_num = String::from(_nums[0]);
        string_num.push_str(&(_nums.last().unwrap().to_string()));
        res += string_num.parse::<i32>().unwrap();
    }
    return res;
}

fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::part1;
        let input = "1abc2
                    pqr3stu8vwx
                    a1b2c3d4e5f
                    treb7uchet";
        let result = part1(input);
        assert_eq!(result, 142);
    }
}
