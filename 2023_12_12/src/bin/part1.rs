fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");

}

fn recurse(puzzle: &mut Vec<char>, nums: &Vec<u32>, pos: u32, n_pos: u32, count: u32) -> u32 {
    if pos as usize == puzzle.len() {
        return ( (n_pos as usize >= nums.len()) || (((n_pos + 1) as usize) == nums.len() && count == nums[n_pos as usize]) ) as u32;
    }
    if puzzle[pos as usize] == '#' {
        if n_pos as usize == nums.len() || count + 1 > nums[n_pos as usize] {
            return 0;
        }
        return recurse(puzzle, nums, pos + 1, n_pos, count + 1);
    }
    if puzzle[pos as usize] == '.' {
        if count > 0 && ((n_pos as usize) == nums.len() || count != nums[n_pos as usize]) {
            return 0;
        }
        if count > 0 {
            return recurse(puzzle, nums, pos + 1, n_pos + 1, 0);
        }
        return recurse(puzzle, nums, pos + 1, n_pos, 0);
    }
    if count > 0 { 
        if (n_pos as usize) == nums.len() {
            return 0;
        }
        if count == nums[n_pos as usize] {
            return recurse(puzzle, nums, pos + 1, n_pos + 1, 0);
        }
        return recurse(puzzle, nums, pos + 1, n_pos, count + 1);
    }
    if n_pos as usize == nums.len() {
        return recurse(puzzle, nums, pos + 1, n_pos, 0);
    }
    return recurse(puzzle, nums, pos + 1, n_pos, 0) + recurse(puzzle, nums, pos + 1, n_pos, 1);
}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    for s in lines.iter() {
        let sp: Vec<_> = s.split(' ').collect();
        let nums: Vec<u32> = sp[1].split(',').map(|x| x.parse::<u32>().unwrap()).collect();
        let mut puzzle: Vec<char> = sp[0].chars().collect();
        let temp = recurse(&mut puzzle, &nums, 0, 0, 0);
        // println!("{temp}");
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
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result1 = part1(input1);
        assert_eq!(result1, 21);
    }
}
