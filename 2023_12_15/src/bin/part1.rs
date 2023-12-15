fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");

}

fn get_hash(s: &str) -> i64 {
    let mut res = 0;
    for ch in s.chars() {
        let ch: i8 = ch as i8;
        res += ch as i64;
        res *= 17;
        res %= 256;
    }
    return res;
}

// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.

fn part1(input: &str) -> i64 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    for s in lines.iter() {
        let words: Vec<_> = s.split(',').filter(|x| !x.is_empty()).map(|x| x.trim()).collect();
        for word in words {
            let temp = get_hash(word);
            res += temp;
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = "HASH";
        let input2 = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result1 = part1(input1);
        let result2 = part1(input2);
        assert_eq!(result1, 52);
        assert_eq!(result2, 1320);
    }
}
