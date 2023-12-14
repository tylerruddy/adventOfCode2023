extern crate queues;
use queues::*;

fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

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
    // println!("Checking!");
    for (row, v) in visited.iter().enumerate() {
        for (col, b) in v.iter().enumerate() {
            if !b {
                if check_intersects(&visited, &lines, row as i32, col as i32, 0, 1) % 2 == 1 {
                    // println!("{row}, {col}");
                    res += 1;
                }
            }
        }
    }
    return res;
}

fn is_vert(lines: &Vec<String>, x: i32, y: i32) -> bool {
    return lines[x as usize].chars().nth(y as usize).unwrap() == '|' ||
            (lines[x as usize].chars().nth(y as usize).unwrap() == 'S' &&
             x > 0 && (lines[(x-1) as usize].chars().nth(y as usize).unwrap() == '|' || lines[(x-1) as usize].chars().nth(y as usize).unwrap() == 'F' || lines[(x-1) as usize].chars().nth(y as usize).unwrap() == '7') &&
             ((x + 1) as usize) < lines.len() && (lines[(x+1) as usize].chars().nth(y as usize).unwrap() == '|' || lines[(x+1) as usize].chars().nth(y as usize).unwrap() == 'L' || lines[(x+1) as usize].chars().nth(y as usize).unwrap() == 'J'));
}

fn is_j(lines: &Vec<String>, x: i32, y: i32) -> bool {
    return lines[x as usize].chars().nth(y as usize).unwrap() == 'J' ||
            (lines[x as usize].chars().nth(y as usize).unwrap() == 'S' &&
             x > 0 && (lines[(x-1) as usize].chars().nth(y as usize).unwrap() == '|' || lines[(x-1) as usize].chars().nth(y as usize).unwrap() == 'F' || lines[(x-1) as usize].chars().nth(y as usize).unwrap() == '7') &&
             y > 0 && (lines[x as usize].chars().nth((y-1) as usize).unwrap() == '-' || lines[x as usize].chars().nth((y-1) as usize).unwrap() == 'F' || lines[x as usize].chars().nth((y-1) as usize).unwrap() == 'L' ));
}

fn is_l(lines: &Vec<String>, x: i32, y: i32) -> bool {
    return lines[x as usize].chars().nth(y as usize).unwrap() == 'L' ||
            (lines[x as usize].chars().nth(y as usize).unwrap() == 'S' &&
             x > 0 && (lines[(x-1) as usize].chars().nth(y as usize).unwrap() == '|' || lines[(x-1) as usize].chars().nth(y as usize).unwrap() == 'F' || lines[(x-1) as usize].chars().nth(y as usize).unwrap() == '7') &&
             ((y+1) as usize) < lines[0].len() && (lines[x as usize].chars().nth((y+1) as usize).unwrap() == '-' || lines[x as usize].chars().nth((y+1) as usize).unwrap() == '7' || lines[x as usize].chars().nth((y+1) as usize).unwrap() == 'J'));
}

fn is_7(lines: &Vec<String>, x: i32, y: i32) -> bool {
    return lines[x as usize].chars().nth(y as usize).unwrap() == '7' ||
            (lines[x as usize].chars().nth(y as usize).unwrap() == 'S' &&
             ((x + 1) as usize) < lines.len() && (lines[(x+1) as usize].chars().nth(y as usize).unwrap() == '|' || lines[(x+1) as usize].chars().nth(y as usize).unwrap() == 'L' || lines[(x+1) as usize].chars().nth(y as usize).unwrap() == 'J') &&
             y > 0 && (lines[x as usize].chars().nth((y-1) as usize).unwrap() == '-' || lines[x as usize].chars().nth((y-1) as usize).unwrap() == 'F' || lines[x as usize].chars().nth((y-1) as usize).unwrap() == 'L'));
}

fn is_f(lines: &Vec<String>, x: i32, y: i32) -> bool {
    return lines[x as usize].chars().nth(y as usize).unwrap() == 'F' ||
            (lines[x as usize].chars().nth(y as usize).unwrap() == 'S' &&
             ((x + 1) as usize) < lines.len() && (lines[(x+1) as usize].chars().nth(y as usize).unwrap() == '|' || lines[(x+1) as usize].chars().nth(y as usize).unwrap() == 'L' || lines[(x+1) as usize].chars().nth(y as usize).unwrap() == 'J') &&
             ((y+1) as usize) < lines[0].len() && (lines[x as usize].chars().nth((y+1) as usize).unwrap() == '-' || lines[x as usize].chars().nth((y+1) as usize).unwrap() == '7' || lines[x as usize].chars().nth((y+1) as usize).unwrap() == 'J'));
}

fn check_intersects(visited: &Vec<Vec<bool>>, lines: &Vec<String>, mut x: i32, mut y: i32, dx: i32, dy: i32) -> u32
{
    x += dx;
    y += dy;
    let mut count = 0;
    let mut v : Vec<char> = Default::default();
    while x >= 0 && y >= 0 && (x as usize) < lines.len() && (y as usize) < lines[0].len() {
        if !visited[x as usize][y as usize] {
            x += dx;
            y += dy;
            continue;
        }
        if is_vert(lines, x, y) {
                count += 1;
        }
        else if is_j(lines, x, y) {
                if v.len() > 0 && *v.last().unwrap() == 'L' {
                    v.pop();
                }
                else if v.len() > 0 && *v.last().unwrap() == 'F' {
                    count += 1;
                    v.pop();
                }
                else {
                    v.push('J');
                }
        }
        else if is_l(lines, x, y) {
                    v.push('L');
        }
        else if is_7(lines, x, y) {
                if v.len() > 0 && *v.last().unwrap() == 'F' {
                    v.pop();
                }
                else if v.len() > 0 && *v.last().unwrap() == 'L' {
                    count += 1;
                    v.pop();
                }
                else {
                    v.push('7');
                }
        }
        else if is_f(lines, x, y) {
                    v.push('F');
        }
        x += dx;
        y += dy;
    }

    return count;
}

fn part2(input: &str) -> u32 {
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
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let input2 = 
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let input3 =
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let result1 = part2(input1);
        let result2 = part2(input2);
        let result3 = part2(input3);
        assert_eq!(result1, 4);
        assert_eq!(result2, 8);
        assert_eq!(result3, 10);
    }
}
