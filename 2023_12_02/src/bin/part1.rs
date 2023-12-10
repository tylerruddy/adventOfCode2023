fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    let lines: Vec<_> = input.lines().map(String::from).collect();
    
    for line in lines.iter() {
        let s: Vec<_> = line.split(':').map(|x| x.trim()).collect();
        let game_num: u32 = s[0].split(' ').map(|x| x.trim()).collect::<Vec<_>>()[1].parse().unwrap();
        let amounts = s[1];
        // println!("{:?}+{:?}", game_num, amounts);
        let mut possible_game: bool = true;
        for subset in amounts.split(';').collect::<Vec<_>>() {
            for cubes in subset.split(',').map(|x| x.trim()).collect::<Vec<_>>() {
                let count_color: Vec<_> = cubes.split(' ').map(|x| x.trim()).collect();
                // println!("{:?}", count_color);
                if count_color[1] == "blue" {
                    if count_color[0].parse::<i32>().unwrap() > 14 {
                        possible_game = false;
                        break;
                    }
                }
                else if count_color[1] == "red" {
                    if count_color[0].parse::<i32>().unwrap() > 12 {
                        possible_game = false;
                        break;
                    }
                }
                else if count_color[1] == "green" {
                    if count_color[0].parse::<i32>().unwrap() > 13 {
                        possible_game = false;
                        break;
                    }
                }
            }
        }
        if possible_game {
            res += game_num;
            // println!("{game_num}: {:?}", amounts);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input);
        assert_eq!(result, 8);
    }
}
