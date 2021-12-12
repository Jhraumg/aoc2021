use itertools::Itertools;
use std::fmt;

#[derive(Debug)]
pub struct BingoResult {
    winner_idx: usize,
    winner_score: usize,
}

pub(crate) struct BingoBoard {
    // Each time a number is found, replace it with None
    grid: Vec<Vec<Option<usize>>>,
    round_nb: usize,
    score: Option<usize>,
}

impl fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BingoBoard")?;
        for line in self.grid.iter() {
            let line_str = line
                .iter()
                .map(|v| v.map_or("  ".to_string(), |v| v.to_string()))
                .fold("\n".to_string(), |mut acc, v| {
                    acc.push(' ');
                    acc.push_str(&v);
                    acc
                });
            write!(f, "\n    {}", line_str)?;
        }
        write!(
            f,
            "\n round {} : score {}",
            self.round_nb,
            self.score.map_or("".to_string(), |v| v.to_string())
        )?;

        write!(f, "\n.")
    }
}

impl BingoBoard {
    pub(crate) fn new(lines: &[&str]) -> BingoBoard {
        let grid = lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|val| val.parse::<usize>().ok())
                    .collect_vec()
            })
            .filter(|v| !v.is_empty())
            .collect_vec();
        let lenght = grid[0].len();
        if grid.iter().any(|l| l.len() != lenght) {
            panic!("grid from {:?} is not regular : {:?}", lines, grid);
        }

        BingoBoard {
            grid,
            round_nb: 0,
            score: None,
        }
    }

    pub(crate) fn compute_score(&mut self, last_played: usize) -> Option<usize> {
        let finished_line = self.grid.iter().find(|l| l.iter().all(|v| v.is_none()));

        if finished_line.is_some() {
            // dbg!(&finished_line);
        }

        let finished_column = finished_line.is_some() || {
            let line_length = self.grid[0].len();
            let columns = (0..line_length)
                .map(|c_idx| self.grid.iter().map(|l| l[c_idx]).collect_vec())
                .collect_vec();

            let finished = columns.iter().find(|c| c.iter().all(|v| v.is_none()));
            if finished.is_some() {
                // dbg!(&finished);
                // dbg!(&self);
            }

            finished.is_some()
        };

        if finished_line.is_some() || finished_column {
            // dbg!(finished_line, finished_column);
            let sum: usize = self
                .grid
                .iter()
                .flat_map(|v| v.iter())
                .map(|v| v.unwrap_or(0))
                .sum();
            Some(last_played * sum)
        } else {
            None
        }
    }

    // returns Some(score) when winning
    pub(crate) fn play(&mut self, value: usize) -> Option<usize> {
        if self.score.is_none() {
            self.round_nb += 1;
            for line in self.grid.iter_mut() {
                for val in line.iter_mut() {
                    if val.map(|v| v == value).unwrap_or(false) {
                        val.take();
                    }
                }
            }
            self.score = self.compute_score(value);
        }
        self.score
    }
}

struct BingoGame {
    drawns: Vec<usize>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    pub fn new(bingo: &str) -> BingoGame {
        let lines = bingo.lines().collect_vec();
        // dbg!(&lines[0]);
        let drawns = lines[0]
            .split(',')
            .filter_map(|val| val.parse::<usize>().ok())
            .collect_vec();

        // boards are separated by empty lines
        // ==> boards boundaries are non consecutive empty lines
        let mut last_non_empty_line = None;
        let mut last_empty_line = None;
        let mut boards: Vec<BingoBoard> = Vec::new();
        for (idx, line) in lines[1..].iter().enumerate() {
            // dbg!(idx, line);
            if line.is_empty() {
                last_empty_line = Some(idx);
                if let Some(start_idx) = last_non_empty_line {
                    boards.push(BingoBoard::new(&lines[start_idx..idx + 1])); // tricky :because of init offset
                    last_non_empty_line = None;
                }
            } else if last_non_empty_line.is_none() {
                last_non_empty_line = Some(idx + 1); // FIXME : tricky
            }
        }
        if last_non_empty_line.unwrap() > last_empty_line.unwrap() {
            boards.push(BingoBoard::new(&lines[last_non_empty_line.unwrap()..]));
        }
        BingoGame { drawns, boards }
    }
}

pub fn play_bingo(bingo: &str) -> (BingoResult, BingoResult) {
    let mut bg = BingoGame::new(bingo);
    // dbg!("drawns : {}, boards :{}", &bg.drawns ,bg.boards.len());
    let mut first_winner = None;
    let mut last_winner = None;

    for drawn in bg.drawns {
        let ranked_scores = bg
            .boards
            .iter_mut()
            .enumerate()
            .map(|(_idx, board)| {
                // println!("play( {}, {})", idx, drawn);
                board.play(drawn);
                (board.round_nb, board.score)
            })
            .collect_vec();
        let best = ranked_scores
            .iter()
            .enumerate()
            .filter_map(|(idx, (rank, score))| score.map(|s| (idx, *rank, s)))
            .max_by(|(_idx, rank1, _score), (_, rank2, _)| rank1.cmp(rank2));
        if let Some((winner_idx, _rank, winner_score)) = best {
            // dbg!(drawn, winner_idx);
            last_winner = Some(BingoResult {
                winner_idx,
                winner_score,
            });
            if first_winner.is_none() {
                first_winner = Some(BingoResult {
                    winner_idx,
                    winner_score,
                });
            }
        }
    }

    (first_winner.unwrap(), last_winner.unwrap())
}

pub fn display_bingo() {
    let bingo = include_str!("../ressources/day4_bingo.txt");
    let result = play_bingo(bingo);
    println!("result {:?}", result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example_works() {
        let bingo = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
        ";
        let (first, last) = play_bingo(bingo);
        assert_eq!(2, first.winner_idx, "bad first winner idx");
        assert_eq!(4512, first.winner_score, "bad firstscore");
        assert_eq!(1, last.winner_idx, "bad last winner idx");
        assert_eq!(1924, last.winner_score, "bad last score");
    }
}
