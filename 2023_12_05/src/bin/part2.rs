
fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("{result}");
}

fn part2(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let mut v: Vec<bool> = vec![false; 100];
    let mut temp: Vec<bool> = vec![false; 100];

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            let s: Vec<_> = line.split(':').collect();
            let to_parse: Vec<_> = s[1].split(' ')
                .filter(|x| !x.is_empty() && x.chars().nth(0).unwrap().is_digit(10))
                .map(|x| x.trim()
                    .parse::<u64>()
                    .unwrap())
                .collect();
            for i in (1..to_parse.len()).step_by(2) {
                for seed in to_parse[i-1]..to_parse[i-1]+to_parse[i] {
                    let pos = seed as usize;
                    v[pos] = true;
                    temp[pos] = true;
                }
            }
            continue;
        }
        if line.is_empty() {
            v = temp.clone();
            continue;
        }
        if !line.chars().nth(0).unwrap().is_digit(10) {
            println!("{line}");
            continue;
        }
        let map = line.split(' ')
            .map(|x| x.trim()
                .parse::<u64>()
                .unwrap())
            .collect::<Vec<_>>();
        let dest_start = map[0] as usize;
        let source_start = map[1] as usize;
        let range = map[2] as usize;
        for i in 0..range {
            if v[source_start+i] {
                temp[source_start+i] = false;
                temp[dest_start+i] = true;
            }
        }
    }

    v = temp.clone();
    println!("{:?}", v);

    let mut res = 0;
    for (i, val) in v.iter().enumerate() {
        if *val {
            res = i as u64;
            break;
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input = 
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part2(input);
        assert_eq!(result, 46);
    }
}
