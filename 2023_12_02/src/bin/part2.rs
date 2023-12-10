use std::cmp;

fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("{result}");
}

fn part2(input: &str) -> u32 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    
    for line in lines.iter() {
        let s: Vec<_> = line.split(':').map(|x| x.trim()).collect();
        let amounts = s[1];
        // println!("{:?}+{:?}", game_num, amounts);
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for subset in amounts.split(';').collect::<Vec<_>>() {
            for cubes in subset.split(',').map(|x| x.trim()).collect::<Vec<_>>() {
                let count_color: Vec<_> = cubes.split(' ').map(|x| x.trim()).collect();
                if count_color[1] == "blue" {
                    blue = cmp::max(blue, count_color[0].parse::<u32>().unwrap());
                }
                else if count_color[1] == "red" {
                    red = cmp::max(red, count_color[0].parse::<u32>().unwrap());
                }
                else if count_color[1] == "green" {
                    green = cmp::max(green, count_color[0].parse::<u32>().unwrap());
                }
            }
        }
        res += red * blue * green;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input = 
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2(input);
        assert_eq!(result, 2286);
    }
}
