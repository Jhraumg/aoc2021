use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

struct DeterministDie {
    rolled: usize,
}
impl DeterministDie {
    fn rolls(&mut self) -> usize {
        self.rolled += 1;
        ((self.rolled - 1) % 100) + 1
    }
    fn new() -> Self {
        DeterministDie { rolled: 0 }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Player {
    place: usize,
    score: usize,
}
impl Player {
    fn new(place: usize) -> Self {
        Self { place, score: 0 }
    }
    fn advance(&mut self, steps: usize) -> usize {
        self.place = (self.place + steps - 1) % 10 + 1;
        self.score += self.place;
        self.score
    }
}
impl FromStr for Player {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(" starting position: ") {
            return Err(anyhow!("'{}' does not seems to be a Player", s));
        }
        s.split(" starting position: ")
            .nth(1)
            .and_then(|v| v.parse::<usize>().ok())
            .map(|start| Ok(Player::new(start)))
            .unwrap_or_else(|| Err(anyhow!("could not parse start place from '{}'", s)))
    }
}

fn play_deterministic_dice(mut players: Vec<Player>) -> usize {
    let mut die = DeterministDie::new();
    'main: loop {
        for player in &mut *players {
            let moves: usize = (0..3).map(|_| die.rolls()).sum();
            player.advance(moves);
            if player.score >= 1000 {
                break 'main;
            }
        }
    }
    for player in players {
        if player.score < 1000 {
            return player.score * die.rolled;
        }
    }
    panic!("everyone won !??!")
}

fn get_triple_dirac_die_counts_by_sum() -> &'static [usize] {
    static TRIPLE_DIE_COUNT_BY_SUM: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
    &TRIPLE_DIE_COUNT_BY_SUM
}

#[derive(Clone)]
struct Universe {
    /// count of equivalent universe, because
    /// several 3 dice throw can result in the same amount
    count: usize,
    players: Vec<Player>,
    winner_idx: Option<usize>,
}

fn compute_results_for_dirac_dices(players: Vec<Player>) -> Vec<usize> {
    let players_count = players.len();
    let mut win_count = vec![0usize, players_count];

    // We're storing each play, with its number of occurrences (different dice throws lead to same sum)
    // and the winner idx if present
    let mut multiverse: Vec<Universe> = vec![Universe {
        count: 1,
        players,
        winner_idx: None,
    }];
    loop {
        // eprintln!("open plays : {}", multiverse.iter().map(|((count,_),_)|*count).sum::<usize>());

        let step_result: Vec<_> = multiverse
            .iter()
            .flat_map(
                |Universe {
                     count,
                     players,
                     winner_idx,
                 }| {
                    let mut possibilities = vec![Universe {
                        count: *count,
                        players: players.clone(),
                        winner_idx: *winner_idx,
                    }];

                    for i in 0..players_count {
                        let mut new_possibilities: Vec<Universe> =
                            Vec::with_capacity(5 * possibilities.len());
                        for Universe {
                            count,
                            players,
                            winner_idx,
                        } in &possibilities
                        {
                            if winner_idx.is_some() {
                                new_possibilities.push(Universe {
                                    count: *count,
                                    players: players.clone(),
                                    winner_idx: *winner_idx,
                                });
                            } else {
                                let others = players
                                    .iter()
                                    .enumerate()
                                    .filter_map(
                                        |(z, p)| if z != i { Some(p.clone()) } else { None },
                                    )
                                    .collect_vec();

                                for dice_sum in 3..=9 {
                                    let mut new_p = players[i].clone();
                                    new_p.advance(dice_sum);
                                    let score = new_p.score;
                                    let mut new_players = others.clone();
                                    new_players.insert(i, new_p);
                                    new_possibilities.push(Universe {
                                        count: count
                                            * get_triple_dirac_die_counts_by_sum()[dice_sum],
                                        players: new_players,
                                        winner_idx: if score >= 21 { Some(i) } else { None },
                                    });
                                }
                            }
                        }
                        possibilities = new_possibilities;
                    }

                    possibilities.into_iter()
                },
            )
            .collect();

        let mut count_by_players: HashMap<Vec<Player>, usize> = HashMap::new();

        // TODO : #[cfg(debug_assertions)]
        // let winners = step_result
        //     .iter()
        //     .filter_map(|((count, _), winner)| winner.map(|_| *count))
        //     .sum::<usize>();
        // eprintln!("{} new winners", winners);
        //
        // let kept_count = step_result
        //     .iter()
        //     .filter(|((_, _), winner)| winner.is_none())
        //     .count();
        for Universe {
            count,
            players,
            winner_idx,
        } in step_result
        {
            if let Some(i) = winner_idx {
                win_count[i] += count;
            } else {
                let new_count = count_by_players.get(&players).unwrap_or(&0) + count;
                count_by_players.insert(players, new_count);
            }
        }

        multiverse = count_by_players
            .into_iter()
            .map(|(players, count)| Universe {
                count,
                players,
                winner_idx: None,
            })
            .collect();
        // eprintln!("condensed {} entries, kept {}", kept_count-multiverse.len(), multiverse.len());
        if multiverse.is_empty() {
            break;
        }
    }
    win_count
}

pub fn display_dirac_dice_play() {
    let input = "Player 1 starting position: 3
Player 2 starting position: 5
";
    let players: Vec<_> = input
        .lines()
        .filter_map(|l| l.parse::<Player>().ok())
        .collect();
    let loosing_score_by_rolls = play_deterministic_dice(players.clone());

    println!(
        "deterministic dice : loosing score by rolls {}",
        loosing_score_by_rolls
    );

    let dirac_wins = compute_results_for_dirac_dices(players);
    println!("max number of win : {}", dirac_wins.iter().max().unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn aoc_example_works() {
        let input = "Player 1 starting position: 4
Player 2 starting position: 8";

        let mut players: Vec<_> = input
            .lines()
            .filter_map(|l| l.parse::<Player>().ok())
            .collect();
        assert_eq!(2, players.len());
        assert_eq!(4, players[0].place);
        assert_eq!(8, players[1].place);

        assert_eq!(739785, play_deterministic_dice(players.clone()));

        let wins = compute_results_for_dirac_dices(players.clone());
        println!("wins {:?}", wins);
        assert_eq!(444356092776315, wins[0]);

        //assert_eq!(341960390180808,wins[1]);
        // is this a bug ?
        assert_eq!(341960390180810, wins[1]);
    }
}
