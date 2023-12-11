use std::collections::HashSet;

fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();

    for line in lines.iter() {
        let game_cards: Vec<_> = line.split(':')
            .map(|x| x.trim())
            .collect::<Vec<_>>();
        // println!("{:?}", game_cards);
        let nums: Vec<_> = game_cards[1].split('|')
            .map(|x| x.trim())
            .collect();
        // println!("{:?}", nums);
        let winning_nums: HashSet<i32> = nums[0].split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        // println!("{:?}", winning_nums);
        let my_nums: HashSet<i32> = nums[1].split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        // println!("{:?}", my_nums);
        let mut matches = 0;
        for num in my_nums.iter() {
            if winning_nums.contains(num) {
                matches += 1;
            }
        }
        if matches > 0 {
            res += 2_u32.pow(matches - 1);
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
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(input);
        assert_eq!(result, 13);
    }
}
