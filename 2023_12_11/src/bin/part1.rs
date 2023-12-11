extern crate queues;
use queues::*;

fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");
}

fn check_step(x: i32, y: i32, steps: u32, m: usize, n: usize, q: &mut Queue<(i32, i32, u32)>) {
    if x < 0 || y < 0 || x as usize >= m || y as usize >= n {
        return;
    }
    let _ = q.add((x, y, steps + 1));
}

fn shortest_path(a: (i32, i32), b: (i32, i32), m: usize, n: usize) -> u32 {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
    let mut q: Queue<(i32, i32, u32)> = queue![];
    let _ = q.add((a.0, a.1, 0));
    while q.size() > 0 {
        let (x, y, steps) = q.remove().unwrap();
        if visited[x as usize][y as usize] {
            continue;
        }
        visited[x as usize][y as usize] = true;
        if x == b.0 && y == b.1 {
            return steps;
        }
        check_step(x + 1, y, steps, m, n, &mut q);
        //check_step(x - 1, y, steps, m, n, &mut q); // all will be below or directly to the right
        check_step(x, y + 1, steps, m, n, &mut q);
        check_step(x, y - 1, steps, m, n, &mut q);
    }
    return 1000000;
}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let mut extra_rows: Vec<bool> = vec![false; lines.len()];
    let mut extra_cols: Vec<bool> = vec![false; lines[0].len()];
    let mut galaxies: Vec<(i32, i32)> = Default::default();

    for (row,  line) in lines.iter().enumerate() {
        let mut found = false;
        for ch in line.chars() {
            if ch == '#' {
                found = true;
            }
        }
        if !found {
            extra_rows[row] = true;
        }
    }
    for col in 0..lines[0].len() {
        let mut found = false;
        for row in 0..lines.len() {
            if lines[row].chars().nth(col).unwrap() == '#' {
                found = true;
            }
        }
        if !found {
            extra_cols[col] = true;
        }
    }
    let mut row = 0;
    let mut col = 0;
    for (i, line) in lines.iter().enumerate() {
        col = 0;
        if extra_rows[i] {
            row += 2;
            continue;
        }
        for (j, ch) in line.chars().enumerate() {
            if extra_cols[j] {
                col += 2;
                continue;
            }
            if ch == '#' {
                galaxies.push((row, col));
            }
            col += 1;
        }
        row += 1;
    }

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let temp = shortest_path(galaxies[i], galaxies[j], row as usize, col as usize);
            println!("{}, {}", i + 1, j + 1);
            res += temp;
        }
    }

    // for p in galaxies.iter() {
    //     println!("{:?}", p);
    // }

    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = part1(input);
        assert_eq!(result, 374);
    }
}
