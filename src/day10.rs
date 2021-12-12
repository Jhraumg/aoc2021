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

type SyntaxResult = Result<&'static str, char>;

fn get_valid_subsequence(seq: &'static str) -> SyntaxResult {
    if seq.is_empty() {
        return Ok("");
    }
    let start = seq.chars().next().unwrap();
    if seq.len() == 1 {
        eprintln!("sequence {} is incomplete", seq);
        return Err(start);
    }
    let mut read_l = 1;
    match start {
        '<' | '[' | '{' | '(' => {
            while read_l < seq.len() {
                let next_char = seq[read_l..].chars().next().unwrap();
                // maybe we're done
                if next_char == get_closing_char(start) {
                    return Ok(&seq[..read_l + 1]);
                }

                // maybe there is another subsequence
                if get_starting_chars().contains(&seq[read_l..].chars().next().unwrap()) {
                    let subs = get_valid_subsequence(&seq[read_l..])?;
                    read_l += subs.len();

                    continue;
                }
                // ore there is a pb
                let bad_closing_char = seq.chars().nth(read_l).unwrap();
                eprintln!(
                    "{} {} {} mismatch",
                    start,
                    &seq[..read_l + 1],
                    bad_closing_char
                );
                return Err(bad_closing_char);
            }
            //eprintln!("unfinished sequence");
            Err(start)
        }

        _ => {
            panic!("BAD START char {}", start);
            Err(start)
        }
    }
}

fn check_line(line: &'static str) -> SyntaxResult {
    let mut read_chars = 0;
    while read_chars < line.len() {
        let sub = get_valid_subsequence(&line[read_chars..])?;
        read_chars += sub.len();
    }
    Ok(line)
}

fn illegal_score(input: &'static str) -> usize {
    input
        .lines()
        .filter_map(|line| check_line(line).err())
        .map(|c| match dbg!(c) {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum()
}

pub fn print_syntax_check() {
    let input = include_str!("../ressources/day10_navigation_syntax.txt");

    println!("illegal score : {}", illegal_score(input));
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
    }
}
