fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("{result}");

}

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum Type { Open = 0, Roll, Stuck }

fn load_on_north_wall(lines: &Vec<u128>) -> u64 {
    let mut res = 0;
    for (row, num) in lines.iter().enumerate() {
        let mut num_ones = 0;
        let mut temp = *num;
        while temp > 0 {
            temp &= temp - 1;
            num_ones += 1;
        }
        res += num_ones * ((lines.len() as u64) - (row as u64));
    }
    return res;
}

fn shift_direction_impl(lines: &mut Vec<Vec<Type>>, mut row: i32, mut col: i32, d_row: i32, d_col: i32) {
    let mut prev_rock: i32 = {
        if d_row == 1 || d_col == 1 { -1 }
        else if d_col == -1 { lines[0].len() as i32 }
        else { lines.len() as i32 }
    };
    while row >= 0 && (row as usize) < lines.len() && col >= 0 && (col as usize) < lines[0].len() {
        if lines[row as usize][col as usize] == Type::Stuck {
            if d_row != 0 {
                prev_rock = row;
            }
            else {
                prev_rock = col;
            }
        }
        else if lines[row as usize][col as usize] == Type::Roll {
            lines[row as usize][col as usize] = Type::Open;
            if d_row != 0 {
                prev_rock += d_row;
                lines[prev_rock as usize][col as usize] = Type::Roll;
            }
            else {
                prev_rock += d_col;
                lines[row as usize][prev_rock as usize] = Type::Roll;
            }
        }
        row += d_row;
        col += d_col;
    }
}

enum Direction {
    North,
    South,
    East,
    West
}

fn shift_direction(lines: &mut Vec<Vec<Type>>, dir: Direction) {
    match dir {
        Direction::North => {
            for i in 0..lines[0].len() {
                shift_direction_impl(lines, 0, i as i32, 1, 0);
            }
        }
        Direction::South => {
            for i in 0..lines[0].len() {
                shift_direction_impl(lines, (lines.len() - 1) as i32, i as i32, -1, 0);
            }
        }
        Direction::East => {
            for i in 0..lines.len() {
                shift_direction_impl(lines, i as i32, (lines[0].len() - 1) as i32, 0, -1);
            }
        }
        Direction::West => {
            for i in 0..lines.len() {
                shift_direction_impl(lines, i as i32, 0, 0, 1);
            }
        }
    }
}

fn to_type(ch: char) -> Type{ 
    if ch == '.' { return Type::Open; }
    else if ch == '#' { return Type::Stuck; }
    else { return Type::Roll; }
}

fn make_hash(b: &Vec<Vec<Type>>) -> Vec<u128> {
    let mut res: Vec<u128> = Default::default();
    for row in b.iter() {
        let mut hash_row: u128 = 0;
        for (col, val) in row.iter().enumerate() {
            hash_row = hash_row | (((*val == Type::Roll) as u128) << col);
        }
        res.push(hash_row);
    }
    return res;
}

fn are_equal(a: &Vec<u128>, b: &Vec<u128>) -> bool {
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

fn part2(input: &str) -> u64 {
    let mut res: u64 = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let mut board: Vec<Vec<Type>> = lines.iter().map(|x| x.chars().map(|x| to_type(x)).collect()).collect();
    let mut prevs: Vec<Vec<u128>> = Default::default();
    let mut restart_pos: i64 = -1;
    let mut curr_pos: i64 = -1;
    for _ in 0..1000000000 {
        if curr_pos != -1 {
            break;
        }
        shift_direction(&mut board, Direction::North);
        shift_direction(&mut board, Direction::West);
        shift_direction(&mut board, Direction::South);
        shift_direction(&mut board, Direction::East);
        let temp = make_hash(&board);
        let mut found = false;
        for (num, prev) in prevs.iter().enumerate() {
            if are_equal(&prev, &temp) {
                found = true;
                restart_pos = num as i64;
                curr_pos = num as i64;
                break;
            }
        }
        if !found {
            prevs.push(temp);
        }
    }
    let loops_size = prevs.len() - (restart_pos as usize);
    let mut num_loops = 1000000000;
    num_loops -= prevs.len();
    num_loops = num_loops % loops_size;
    println!("{restart_pos}, {loops_size}, {}", prevs.len());
    let prev_pos: usize = {
        if num_loops == 0 { 
            (prevs.len() - 1 ) as usize
        }
        else { 
            (num_loops + (restart_pos as usize) - 1) as usize
        }
    };
    res += load_on_north_wall(&prevs[prev_pos]);

    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let result1 = part2(input1);
        assert_eq!(result1, 64);
    }
}
