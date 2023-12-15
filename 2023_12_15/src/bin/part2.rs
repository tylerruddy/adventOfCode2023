fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");

}

fn get_hash(s: &str) -> usize {
    let mut res = 0;
    for ch in s.chars() {
        let ch: i8 = ch as i8;
        res += ch as i64;
        res *= 17;
        res %= 256;
    }
    return res as usize;
}

// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.

fn part1(input: &str) -> i64 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let mut boxes: Vec<Vec<(String, u8)>> = vec![vec![]; 256];
    for s in lines.iter() {
        let steps: Vec<_> = s.split(',').filter(|x| !x.is_empty()).map(|x| x.trim()).collect();
        for step in steps {
            if step.chars().nth(step.len() - 1).unwrap() == '-' {
                let w = step.split('-').filter(|x| !x.is_empty()).map(|x| x.trim()).collect::<Vec<_>>()[0].to_string();
                let box_num = get_hash(&w);
                let pos_in_box = boxes[box_num].iter().position(|x| x.0 == w);
                if pos_in_box.is_some() {
                    boxes[box_num].remove(pos_in_box.unwrap());
                }
            }
            else {
                let w = step.split('=').filter(|x| !x.is_empty()).map(|x| x.trim()).collect::<Vec<_>>()[0].to_string();
                let num = step.split('=').filter(|x| !x.is_empty()).map(|x| x.trim()).collect::<Vec<_>>()[1].to_string().parse::<u8>().unwrap();
                let box_num = get_hash(&w);
                let pos_in_box = boxes[box_num].iter().position(|x| x.0 == w);
                if pos_in_box.is_some() && pos_in_box.unwrap() < boxes[box_num].len() {
                    boxes[box_num][pos_in_box.unwrap()].1 = num;
                }
                else {
                    boxes[box_num].push((w, num));
                }
            }
        }
    }
    // To confirm that all of the lenses are installed correctly, add up the focusing power of all of the lenses. The focusing power of a single lens is the result of multiplying together:
    // One plus the box number of the lens in question.
    // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    // The focal length of the lens.
    for (box_num, b) in boxes.iter().enumerate() {
        if b.len() == 0 {
            continue;
        }
        for (slot_num, p) in b.iter().enumerate() {
            res += (box_num + 1) * (slot_num + 1) * (p.1 as usize);
        }
    }
    return res as i64;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        // let input1 = "HASH";
        let input2 = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        // let result1 = part1(input1);
        let result2 = part1(input2);
        // assert_eq!(result1, 52);
        assert_eq!(result2, 145);
    }
}
