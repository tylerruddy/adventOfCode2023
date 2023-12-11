
fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    println!("{result}");
}

fn part1(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let mut v: Vec<u64> = Default::default();
    let mut temp: Vec<u64> = Default::default();

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            let s: Vec<_> = line.split(':').collect();
            v = s[1].split(' ')
                .filter(|x| !x.is_empty() && x.chars().nth(0).unwrap().is_digit(10))
                .map(|x| x.trim()
                    .parse::<u64>()
                    .unwrap())
                .collect();
            temp = v.clone();
            continue;
        }
        if line.is_empty() {
            v = temp.clone();
            // println!("{:?}", v);
            continue;
        }
        if !line.chars().nth(0).unwrap().is_digit(10) {
            // println!("{line}");
            continue;
        }
        let map = line.split(' ')
            .map(|x| x.trim()
                .parse::<u64>()
                .unwrap())
            .collect::<Vec<_>>();
        let dest_start = map[0];
        let source_start = map[1];
        let range = map[2];
        for i in 0..v.len() {
            let num = v[i];
            if num >= source_start && num < source_start + range {
                let offset = num - source_start;
                temp[i] = dest_start + offset;
            }
        }
    }
    v = temp.clone();
    // println!("{:?}", v);

    return *v.iter().min().unwrap();
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
        let result = part1(input);
        assert_eq!(result, 35);
    }
}
