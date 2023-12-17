use std::collections::BTreeSet;

fn main() {
    println!("part2");
    let input = include_str!("./input1.txt");
    let result = part2(input);

    println!("{result}");
}

fn parse_starting_ranges(line: String) -> Vec<(u64, u64)> {
    let v: Vec<u64> = line.split(':')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim())
        .collect::<Vec<_>>()[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();
    let mut res: Vec<(u64, u64)> = vec![];
    for i in (1..v.len()).step_by(2) {
        res.push((v[i-1], v[i-1] + v[i]));
    }
    return res;
}

fn parse_range(line: &String) -> ((u64, u64), (u64, u64)) {
    let nums: Vec<u64> = line.split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();
    let dest_start = nums[0];
    let source_start = nums[1];
    let range = nums[2];
    return ((source_start, source_start + range), (dest_start, dest_start + range))
}

// Custom data type that stores ranges in sorted order and extracts ranges, then I make a new one
// everytime theres a map
// Extract will return a vec of ranges and will have removed any overlap
// Adding a range to this type will allow merges

fn extract_overlapping_ranges(dsr: &mut BTreeSet<(u64, u64)>, range: (u64, u64)) -> Vec<(u64, u64)> {
    let mut overlapping_ranges: Vec<(u64, u64)> = vec![];
    for r in dsr.iter() {
        // Overlap
        if ranges_overlap(*r, range) {
            overlapping_ranges.push(*r);
        }
    }
    let mut res: Vec<(u64, u64)> = vec![];
    for r in overlapping_ranges.iter() {
        dsr.remove(r);
        // Range leaves some to left and right
        if r.0 < range.0 && r.1 > range.1 {
            dsr.insert((r.0, range.0));
            dsr.insert((range.1, r.1));
            res.push((range.0, range.1))
        } // Range leaves some to the right
        else if r.1 > range.1 {
            dsr.insert((range.1, r.1));
            res.push((r.0, range.1))
        } // Range leaves some to the left
        else if r.0 < range.0 {
            dsr.insert((r.0, range.0));
            res.push((range.0, r.1))
        } // Range completely engulfed
        else { 
            res.push(*r);
        }
    }
    return res;
}

fn add_range(dsr: &mut BTreeSet<(u64, u64)>, range: (u64, u64)) {
    let mut overlapping_ranges: Vec<(u64, u64)> = vec![];
    for r in dsr.iter() {
        // Overlap
        if can_merge_ranges(*r, range) {
            overlapping_ranges.push(*r);
        }
    }
    if overlapping_ranges.is_empty() {
        dsr.insert(range);
        return;
    }
    for r in overlapping_ranges.iter() {
        dsr.remove(r);
    }
    let smallest = std::cmp::min(range.0, overlapping_ranges.iter().map(|(start, _)| *start).min().unwrap());
    let largest = std::cmp::max(range.1, overlapping_ranges.iter().map(|(_, end)| *end).max().unwrap());
    dsr.insert((smallest, largest));
}

fn ranges_overlap(a: (u64, u64), b: (u64, u64)) -> bool {
    return (a.0 >= b.0 && a.0 < b.1) || (b.0 >= a.0 && b.0 < a.1);
}

fn can_merge_ranges(a: (u64, u64), b: (u64, u64)) -> bool {
    return (a.0 >= b.0 && a.0 <= b.1) || (b.0 >= a.0 && b.0 <= a.1);
}

fn convert_range(start: (u64, u64), end: (u64, u64), range: (u64, u64)) -> (u64, u64) {
    let start_diff = range.0 - start.0;
    let end_diff = range.1 - start.0;
    return (end.0 + start_diff, end.0 + end_diff);
}

fn part2(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().map(String::from).collect();
    let mut curr_ranges: BTreeSet<(u64, u64)> = Default::default();
    let mut temp_ranges: BTreeSet<(u64, u64)> = Default::default();
    let ranges = parse_starting_ranges(lines.iter().next().unwrap().to_string());
    for range in ranges.iter() {
        add_range(&mut curr_ranges, *range);
    }
    let _ = lines.iter().next();

    for line in lines.iter() {
        if line.is_empty() {
            for r in temp_ranges.iter() {
                curr_ranges.insert(*r);
            }
            temp_ranges.clear();
            continue;
        }
        if !line.chars().nth(0).unwrap().is_digit(10) {
            continue;
        }
        let (start_range, dest_range) = parse_range(line);

        let overlapping_ranges = extract_overlapping_ranges(&mut curr_ranges, start_range);
        for range in overlapping_ranges.iter() {
            temp_ranges.insert(convert_range(start_range, dest_range, *range));
        }
    }
    for r in temp_ranges.iter() {
        curr_ranges.insert(*r);
    }

    return curr_ranges.iter()
        .map(|(start, _)| *start)
        .min()
        .unwrap();
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
