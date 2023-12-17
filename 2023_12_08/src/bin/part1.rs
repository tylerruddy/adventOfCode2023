use std::collections::HashMap;

fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");

}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let directions: Vec<char> = lines[0].chars().collect();
    let mut ht: HashMap<String, (String, String)> = Default::default();
    for level in &lines[2..] {
        let key = &level.split('=').filter(|x| !x.is_empty()).map(|x| x.trim()).collect::<Vec<_>>()[0];
        let vals = &level.split('=').filter(|x| !x.is_empty()).map(|x| x.trim()).collect::<Vec<_>>()[1];
        let left: String = vals.split(',')
            .filter(|x| !x.is_empty())
            .map(String::from)
            .collect::<Vec<String>>()[0]
            .chars()
            .filter(|x| *x != ')' && *x != '(' && *x != ' ')
            .collect::<String>();
        let right = &vals.split(',')
            .filter(|x| !x.is_empty())
            .map(String::from)
            .collect::<Vec<String>>()[1]
            .chars()
            .filter(|x| *x != ')' && *x != '(' && *x != ' ')
            .collect::<String>();
        ht.insert((key).to_string(), (left.to_string(), right.to_string()));
    }
    let mut pos = "AAA";
    let mut inst_num = 0;
    while pos != "ZZZ" {
        if inst_num == directions.len() {
            inst_num = 0;
        }
        match directions[inst_num] {
            'L' => pos = &ht.get(pos).unwrap().0,
            _ => pos = &ht.get(pos).unwrap().1,
        }
        res += 1;
        inst_num += 1;
    }
    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let input2 =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result1 = part1(input1);
        let result2 = part1(input2);
        assert_eq!(result1, 2);
        assert_eq!(result2, 6);
    }
}
