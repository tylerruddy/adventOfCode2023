use std::collections::HashMap;

fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("{result}");

}

fn part2(input: &str) -> u64 {
    let mut res = 1;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let directions: Vec<char> = lines[0].chars().collect();
    let mut ht: HashMap<String, (String, String)> = Default::default();
    let mut pos: Vec<String> = vec![];
    for level in &lines[2..] {
        let key = &level.split('=').filter(|x| !x.is_empty()).map(|x| x.trim()).collect::<Vec<_>>()[0];
        if (**key).chars().last().unwrap() == 'A' {
            pos.push(key.to_string());
        }
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
    let mut inst_num: u64 = 0;
    let mut num_looped = 0;
    // let starts = pos.clone();
    let mut loops: Vec<HashMap<(String, u64), u64>> = vec![Default::default(); pos.len()];
    let mut steps_till_z: Vec<u64> = vec![0; pos.len()];
    let mut loop_start: Vec<u64> = vec![0; pos.len()];
    let mut loop_size: Vec<u64> = vec![0; pos.len()];
    while num_looped < loop_size.len() { // TODO: wrong stop condition
        // println!("{res}");
        if inst_num == (directions.len() as u64) {
            inst_num = 0;
        }
        for i in 0..pos.len() {
            if pos[i].is_empty() {
                continue;
            }
            let key = (pos[i].clone(), inst_num);
            // println!("{i}:{res},{:?}", key);
            if loops[i].contains_key(&key) {
                pos[i].clear();
                loop_start[i] = *loops[i].get_key_value(&key).unwrap().1;
                loop_size[i] = res - *loops[i].get_key_value(&key).unwrap().1;
                num_looped += 1;
                continue;
            }
            loops[i].insert(key, res);
            match directions[inst_num as usize] {
                'L' => pos[i] = ht.get(&pos[i]).unwrap().0.clone(),
                _ => pos[i] = ht.get(&pos[i]).unwrap().1.clone(),
            }
            if pos[i].chars().last().unwrap() == 'Z' {
                steps_till_z[i] = res;
            }
        }
        res += 1;
        inst_num += 1;
    }
    let output: u128 = 20659 * 20093 * 14999 * 17263 * 22357 * 16697; 
    println!("Steps till z {output}");
    // for v in steps_till_z.iter() {
    //     println!("{:?}", v);
    // }

    println!("{:?}", steps_till_z);
    println!("Starts: {:?}", loop_start);
    println!("Size: {:?}", loop_size);

    let mut all_same = false;
    while !all_same {
        // println!("{:?}", steps_till_z);
        all_same = true;

        let mut index = 0;
        let mut smallest = steps_till_z[0];
        for i in 1..steps_till_z.len() {
            if steps_till_z[i] < smallest {
                smallest = steps_till_z[i];
                index = i;
            }
        }

        steps_till_z[index] += loop_size[index];

        for i in 1..steps_till_z.len() {
            if steps_till_z[i] != steps_till_z[i-1] {
                all_same = false;
                break;
            }
        }
    }

    return steps_till_z[0];
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result1 = part2(input1);
        assert_eq!(result1, 6);
    }
}
