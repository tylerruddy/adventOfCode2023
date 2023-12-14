extern crate queues;
use queues::*;
use std::cmp;

fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");
}

// TODO: make sure the connections are correct
fn check_step(x: i32, y: i32, steps: u32, m: usize, n: usize, q: &mut Queue<(i32, i32, u32)>) {
    if x < 0 || y < 0 || x as usize >= m || y as usize >= n {
        return;
    }
    let _ = q.add((x, y, steps + 1));
}

fn try_up(x: i32, y: i32, steps: u32, m: usize, n: usize, q: &mut Queue<(i32, i32, u32)>, lines: &Vec<String>) {
    if x <= 0 {
        return;
    }
    let ch = lines[x as usize - 1].chars().nth(y as usize).unwrap();
    if ch == '|' || ch == '7' || ch == 'F' {
        check_step(x - 1, y, steps, m, n, q);
    }
}

fn try_down(x: i32, y: i32, steps: u32, m: usize, n: usize, q: &mut Queue<(i32, i32, u32)>, lines: &Vec<String>) {
    if x as usize + 1 >= m {
        return;
    }
    let ch = lines[x as usize + 1].chars().nth(y as usize).unwrap();
    if ch == '|' || ch == 'J' || ch == 'L' {
        check_step(x + 1, y, steps, m, n, q);
    }
}

fn try_left(x: i32, y: i32, steps: u32, m: usize, n: usize, q: &mut Queue<(i32, i32, u32)>, lines: &Vec<String>) {
    if y as usize <= 0 {
        return;
    }
    let ch = lines[x as usize].chars().nth(y as usize - 1).unwrap();
    if ch == '-' || ch == 'L' || ch == 'F' {
        check_step(x, y - 1, steps, m, n, q);
    }
}

fn try_right(x: i32, y: i32, steps: u32, m: usize, n: usize, q: &mut Queue<(i32, i32, u32)>, lines: &Vec<String>) {
    if y as usize + 1 >= n {
        return;
    }
    let ch = lines[x as usize].chars().nth(y as usize + 1).unwrap();
    if ch == '-' || ch == '7' || ch == 'J' {
        check_step(x, y + 1, steps, m, n, q);
    }
}


fn traverse(start: (i32, i32), lines: &Vec<String>) -> u32 {
    let m = lines.len();
    let n = lines[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
    let mut q: Queue<(i32, i32, u32)> = queue![];
    let _ = q.add((start.0, start.1, 0));
    let mut res = 0;
    while q.size() > 0 {
        let (x, y, steps) = q.remove().unwrap();
        let ch = lines[x as usize].chars().nth( y as usize ).unwrap();
        if visited[x as usize][y as usize] || ch == '.' {
            continue;
        }
        visited[x as usize][y as usize] = true;
        res = cmp::max(res, steps);
        if ch == '|' {
            try_down(x, y, steps, m, n, &mut q, lines);
            try_up(x, y, steps, m, n, &mut q, lines);
        }
        else if ch == '-' {
            try_left(x, y, steps, m, n, &mut q, lines);
            try_right(x, y, steps, m, n, &mut q, lines);
        }
        else if ch == 'L' {
            try_up(x, y, steps, m, n, &mut q, lines);
            try_right(x, y, steps, m, n, &mut q, lines);
        }
        else if ch == 'J' {
            try_up(x, y, steps, m, n, &mut q, lines);
            try_left(x, y, steps, m, n, &mut q, lines);
        }
        else if ch == '7' {
            try_down(x, y, steps, m, n, &mut q, lines);
            try_left(x, y, steps, m, n, &mut q, lines);
        }
        else if ch == 'F' {
            try_down(x, y, steps, m, n, &mut q, lines);
            try_right(x, y, steps, m, n, &mut q, lines);
        }
        else if ch == 'S' {
            try_up(x, y, steps, m, n, &mut q, lines);
            try_down(x, y, steps, m, n, &mut q, lines);
            try_left(x, y, steps, m, n, &mut q, lines);
            try_right(x, y, steps, m, n, &mut q, lines);
        }
    }
    return res;
}

fn part1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let mut start: (i32, i32) = (0, 0);
    for (row,  line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (row as i32, col as i32);
            }
        }
    }

    return traverse(start, &lines);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
".....
.S-7.
.|.|.
.L-J.
.....";
        let input2 = 
"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let result1 = part1(input1);
        let result2 = part1(input2);
        assert_eq!(result1, 4);
        assert_eq!(result2, 8);
    }
}
