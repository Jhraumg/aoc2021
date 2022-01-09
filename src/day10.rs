fn get_starting_chars() -> &'static [char; 4] {
    static STARTING_CHARS: [char; 4] = ['<', '[', '{', '('];
    &STARTING_CHARS
}

fn get_closing_char(start_char: char) -> char {
    match start_char {
        '<' => '>',
        '[' => ']',
        '{' => '}',
        '(' => ')',
        _ => panic!("{} is not a start char", start_char),
    }
}

fn get_closing_score_by_start_char(start: char) -> usize {
    match start {
        '<' => 4,
        '[' => 2,
        '{' => 3,
        '(' => 1,
        _ => 0,
    }
}

#[derive(Debug)]
enum ParsedSequence {
    Complete(&'static str),
    Incomplete { start: char, score: usize },
}

use itertools::Itertools;
use ParsedSequence::{Complete, Incomplete};
type SyntaxResult = Result<ParsedSequence, char>;

fn get_valid_subsequence(seq: &'static str) -> SyntaxResult {
    if seq.is_empty() {
        return Ok(Complete(""));
    }
    let start = seq.chars().next().unwrap();
    if seq.len() == 1 {
        // eprintln!("sequence {} is incomplete", seq);
        return Ok(Incomplete { start, score: 0 });
    }
    let mut read_l = 1;
    match start {
        '<' | '[' | '{' | '(' => {
            while read_l < seq.len() {
                let next_char = seq[read_l..].chars().next().unwrap();
                // maybe we're done
                if next_char == get_closing_char(start) {
                    return Ok(Complete(&seq[..read_l + 1]));
                }

                // maybe there is another subsequence
                if get_starting_chars().contains(&seq[read_l..].chars().next().unwrap()) {
                    match get_valid_subsequence(&seq[read_l..])? {
                        Complete(subs) => {
                            read_l += subs.len();
                        }
                        Incomplete {
                            start: sub_start,
                            score,
                        } => {
                            return Ok(Incomplete {
                                start,
                                score: score * 5 + get_closing_score_by_start_char(sub_start),
                            });
                        }
                    }

                    continue;
                }
                // ore there is a pb
                let bad_closing_char = seq.chars().nth(read_l).unwrap();
                // eprintln!(
                //     "{} {} {} mismatch",
                //     start,
                //     &seq[..read_l + 1],
                //     bad_closing_char
                // );
                return Err(bad_closing_char);
            }
            Ok(Incomplete { start, score: 0 })
        }

        _ => {
            panic!("BAD START char {}", start);
        }
    }
}

fn check_line(line: &'static str) -> SyntaxResult {
    let mut read_chars = 0;
    while read_chars < line.len() {
        match get_valid_subsequence(&line[read_chars..])? {
            Incomplete { start, score } => {
                let score = score * 5 + get_closing_score_by_start_char(start);
                let result = Incomplete { start: '§', score };
                // eprintln!("{} Incomplete : {}", line, score);
                return Ok(result);
            }
            Complete(sub) => {
                read_chars += sub.len();
            }
        }
    }
    Ok(Complete(line))
}

fn illegal_score(input: &'static str) -> usize {
    input
        .lines()
        .filter_map(|line| check_line(line).err())
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum()
}

fn middle_completion_score(input: &'static str) -> usize {
    let incomplete_scores: Vec<_> = input
        .lines()
        .filter_map(|line| check_line(line).ok())
        .filter_map(|seq| match seq {
            Incomplete { score, .. } => Some(score),
            _ => None,
        })
        .sorted()
        .collect();
    let middle = (incomplete_scores.len() - 1) / 2;
    incomplete_scores[middle]
}

pub fn print_syntax_check() {
    let input = include_str!("../resources/day10_navigation_syntax.txt");

    println!("illegal score : {}", illegal_score(input));
    println!(
        "middle incomplete score : {}",
        middle_completion_score(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example_works() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
        assert_eq!(26397, illegal_score(input));
        assert_eq!(288957, middle_completion_score(input))
    }
}
