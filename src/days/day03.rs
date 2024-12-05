use crate::{Solution, SolutionPair};
use chrono::format::parse;
use itertools::Itertools;
use std::fs::read_to_string;
use winnow::combinator::{alt, delimited, opt, preceded, repeat, repeat_till, separated_pair};
use winnow::prelude::*;
use winnow::seq;
use winnow::stream::Stream;
use winnow::token::{any, literal, take_until, take_while};
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").expect("Day 3 input file should be present");
    let sol1 = parse_and_sum(&input);
    let sol2 = parse_and_sum_with_ignore(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_and_sum(input: &str) -> u32 {
    let mut input = input;

    repeat(0.., parse_mul)
        .fold(
            || 0,
            |acc: u32, item: Option<u32>| acc.saturating_add(item.unwrap_or(0)),
        )
        .parse_next(&mut input)
        .unwrap_or(0)
}

fn parse_mul(input: &mut &str) -> PResult<Option<u32>> {
    repeat_till(
        0..,
        any,
        delimited("mul(", separated_pair(parse_number, ',', parse_number), ")")
            .map(|(x, y): (u32, u32)| x.saturating_mul(y)),
    )
    .map(|(_, val): (Vec<char>, u32)| Some(val))
    .parse_next(input)
}

fn parse_number(input: &mut &str) -> PResult<u32> {
    take_while(1..=3, |c: char| c.is_ascii_digit())
        .try_map(|digits: &str| digits.parse::<u32>())
        .parse_next(input)
}

// need to ignore all "mul(x, y)" between "don't()" and "do()"
// ie, consume and skip don't()...do() [even if mul is in it]

// 1. Find the next mul(x, y) or don't(), whichever comes first
// 3. If it's mul(x,y), repeat from 1
// 4. If it's don't(), try to find everything up to and including do(), skip all of that, then go back to 1
fn parse_and_sum_with_ignore(input: &str) -> u32 {
    let mut input = input;

    repeat(0.., parse_skip_dont_to_do)
        .fold(
            || 0,
            |acc: u32, item: Option<u32>| acc.saturating_add(item.unwrap_or(0)),
        )
        .parse_next(&mut input)
        .unwrap_or(0)
}

fn parse_skip_dont_to_do(input: &mut &str) -> PResult<Option<u32>> {
    alt((
        // don't do block
        seq!(
            "don't()",
            repeat_till(0.., any, "do()").map(|(_, _): (Vec<char>, &str)| {})
        )
        .map(|_| 0),
        // mul()
        delimited("mul(", separated_pair(parse_number, ',', parse_number), ")")
            .map(|(x, y): (u32, u32)| x.saturating_mul(y)),
        // anything else,
        any.map(|_| 0),
    ))
    .map(Some)
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#
    }

    fn test_input_2() -> &'static str {
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
    }

    fn test_input_3() -> &'static str {
        r#"xmul(2,4)&mul(3,7)!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
    }

    #[test]
    fn test_parse_and_sum() {
        assert_eq!(parse_and_sum(test_input()), 161);
    }

    #[test]
    fn test_parse_and_sum_with_ignore() {
        assert_eq!(parse_and_sum_with_ignore(test_input_2()), 48);
        assert_eq!(parse_and_sum_with_ignore(test_input_3()), 69);
    }
}
