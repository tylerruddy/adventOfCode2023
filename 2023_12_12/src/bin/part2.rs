use std::thread;
use std::collections::HashMap;

fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("{result}");

}

fn recurse(puzzle: &mut Vec<char>, nums: &Vec<u64>, pos: u64, n_pos: u64, count: u64, ht: &mut HashMap::<(u64, u64, u64), u64>) -> u64 {
    let key = (pos, n_pos, count);
    if ht.contains_key(&key) {
        return *ht.get(&key).unwrap();
    }
    let mut res = 0;
    if pos as usize == puzzle.len() {
        res = ( (n_pos as usize >= nums.len()) || (((n_pos + 1) as usize) == nums.len() && count == nums[n_pos as usize]) ) as u64;
    }
    else if puzzle[pos as usize] == '#' {
        if n_pos as usize == nums.len() || count + 1 > nums[n_pos as usize] {
            res = 0;
        }
        else {
            res = recurse(puzzle, nums, pos + 1, n_pos, count + 1, ht);
        }
    }
    else if puzzle[pos as usize] == '.' {
        if count > 0 && ((n_pos as usize) == nums.len() || count != nums[n_pos as usize]) {
            res = 0;
        }
        else if count > 0 {
            res = recurse(puzzle, nums, pos + 1, n_pos + 1, 0, ht);
        }
        else {
            res = recurse(puzzle, nums, pos + 1, n_pos, 0, ht);
        }
    }
    else if count > 0 { 
        if (n_pos as usize) == nums.len() {
            res = 0;
        }
        else if count == nums[n_pos as usize] {
            res = recurse(puzzle, nums, pos + 1, n_pos + 1, 0, ht);
        }
        else {
            res = recurse(puzzle, nums, pos + 1, n_pos, count + 1, ht);
        }
    }
    else if n_pos as usize == nums.len() {
        res = recurse(puzzle, nums, pos + 1, n_pos, 0, ht);
    }
    else {
        res = recurse(puzzle, nums, pos + 1, n_pos, 0, ht) + recurse(puzzle, nums, pos + 1, n_pos, 1, ht);
    }
    let val = res;
    ht.insert(key, val);
    return res;
}

fn func(lines: &Vec<String>, start: usize) -> u64 {
    let mut res = 0;
    for i in (start..lines.len()).step_by(4) {
        let s = &lines[i as usize];
        let sp: Vec<_> = s.split(' ').collect();
        let prev_nums: Vec<u64> = sp[1].split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        let prev_puzzle: Vec<char> = sp[0].chars().collect();
        let mut nums: Vec<u64> = Default::default();
        let mut puzzle: Vec<char> = Default::default();
        for _ in 0..5 {
            for num in prev_nums.iter() {
                nums.push(*num);
            }
            for ch in prev_puzzle.iter() {
                puzzle.push(*ch);
            }
            puzzle.push('?');
        }
        puzzle.pop();
        // println!("{:?} {:?}", puzzle, nums);
        // println!("{}:{}", i, lines.len());
        let mut ht: HashMap::<(u64, u64, u64), u64> = HashMap::new();
        let temp = recurse(&mut puzzle, &nums, 0, 0, 0, &mut ht);
        res += temp;
        println!("{}:{}", i, temp);
    }
    return res;
}

fn part2(input: &str) -> u64 {
    let lines1: Vec<_> = input.lines().map(String::from).collect();
    let lines2: Vec<_> = input.lines().map(String::from).collect();
    let lines3: Vec<_> = input.lines().map(String::from).collect();
    let lines4: Vec<_> = input.lines().map(String::from).collect();
    let t1 = thread::spawn(move || {
        return func(&lines2, 1);
    });
    let t2 = thread::spawn(move || {
        return func(&lines3, 2);
    });
    let t3 = thread::spawn(move || {
        return func(&lines4, 3);
    });
    let mut res = func(&lines1, 0);
    res += t1.join().unwrap();
    res += t2.join().unwrap();
    res += t3.join().unwrap();
    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result1 = part2(input1);
        assert_eq!(result1, 525152);
    }
}
