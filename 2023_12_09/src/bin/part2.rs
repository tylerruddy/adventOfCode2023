fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("{result}");

}

fn process_line(nums: &mut Vec<i64>) -> i64 {
    if nums.len() == 0 {
        return 0;
    }
    let mut next_v: Vec<i64> = vec![];
    let mut all_zero: bool = nums[0] == 0;
    for i in 1..nums.len() {
        if nums[i] != 0 {
            all_zero = false;
        }
        next_v.push(nums[i] - nums[i-1]);
    }
    let next = if all_zero { 0 } else { nums.first().unwrap() - process_line(&mut next_v) };
    return next;
}

fn part2(input: &str) -> i64 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    for s in lines.iter() {
        let mut temp_v: Vec<i64> = s.split(' ').filter(|x| !x.is_empty()).map(|x| x.trim().parse().unwrap()).collect();
        let temp = process_line(&mut temp_v);
        res += temp;
    }
    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result1 = part2(input1);
        assert_eq!(result1, 2);
    }
}
