use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut left: Vec<usize> = vec![];
        let mut right: Vec<usize> = vec![];

        reader
            .lines()
            .flatten()
            .into_iter()
            .for_each(|line| {
                let (a, b) = line.split_whitespace().map(|n| n.parse().unwrap()).collect_tuple().unwrap();
                left.push(a);
                right.push(b);
            });

        left.sort();
        right.sort();

        let answer = left.iter().zip(right.iter()).map(|(&a, &b)| {
            if a > b {
                a - b
            } else {
                b - a
            }
        }).sum();

        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut times: HashMap<usize, usize> = HashMap::new();
        let mut frequency: HashMap<usize, usize> = HashMap::new();

        reader
            .lines()
            .flatten()
            .into_iter()
            .map(|line| {
                line.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect_tuple::<(usize, usize)>().unwrap()
            })
            .for_each(|(a, b)| {
                times.insert(a, times.get(&a).map_or_else(|| 1, |count| count + 1));
                frequency.insert(b, frequency.get(&b).map_or_else(|| 1, |count| count + 1));
            });
        
        Ok(times.keys().map(|cifra| cifra * frequency.get(cifra).map_or(0, |a| *a) * times.get(cifra).map_or(0, |a| *a)).sum())
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
