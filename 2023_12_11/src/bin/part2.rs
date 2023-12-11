extern crate queues;
use queues::*;

fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input, 1000000);

    println!("{result}");
}

fn check_step(x: i64, y: i64, steps: u64, m: usize, n: usize, q: &mut Queue<(i64, i64, u64)>, step_size: u64) {
    if x < 0 || y < 0 || x as usize >= m || y as usize >= n {
        return;
    }
    let _ = q.add((x, y, steps + step_size));
}

fn shortest_path(a: (i64, i64), b: (i64, i64), m: usize, n: usize, extra_rows: &Vec<bool>, extra_cols: &Vec<bool>, step_size: u64) -> u64 {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
    let mut q: Queue<(i64, i64, u64)> = queue![];
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
        check_step(x + 1, y, steps, m, n, &mut q, if x as usize + 1 < m  && extra_rows[x as usize+1] { step_size } else { 1 });
        //check_step(x - 1, y, steps, m, n, &mut q); // all will be below or directly to the right
        check_step(x, y + 1, steps, m, n, &mut q, if y as usize + 1 < n  && extra_cols[y as usize+1] { step_size } else { 1 });
        check_step(x, y - 1, steps, m, n, &mut q, if y > 0 && extra_cols[y as usize-1] { step_size } else { 1 });
    }
    return 1000000;
}

fn part2(input: &str, step_size: u64) -> u64 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let mut extra_rows: Vec<bool> = vec![false; lines.len()];
    let mut extra_cols: Vec<bool> = vec![false; lines[0].len()];
    let mut galaxies: Vec<(i64, i64)> = Default::default();

    for (row,  line) in lines.iter().enumerate() {
        let mut found = false;
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                found = true;
                galaxies.push((row as i64, col as i64));
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

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let temp = shortest_path(galaxies[i], galaxies[j], lines.len(), lines[0].len(), &extra_rows, &extra_cols, step_size);
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
        let result1 = part2(input, 10);
        assert_eq!(result1, 1030);
        let result2 = part2(input, 100);
        assert_eq!(result2, 8410);
    }
}
