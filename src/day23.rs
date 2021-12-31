use crate::day23::Amphipod::{Amber, Bronze, Copper, Desert};
use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap};
use std::fmt::{Display, Formatter};
use std::hash::{Hash};
use std::str::FromStr;

// Note : could directly model amphipods by their move cost
// => would save a few match
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}
impl Amphipod {
    fn get_single_move_cost(&self) -> usize {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }
}
impl FromStr for Amphipod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "A" => Ok(Amber),
            "B" => Ok(Bronze),
            "C" => Ok(Copper),
            "D" => Ok(Desert),
            _ => Err(anyhow!("cannot made Amphipod from '{}'", s)),
        }
    }
}
impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Amber => "A",
            Bronze => "B",
            Copper => "C",
            Desert => "D",
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Disposition {
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; 2]; 4],
}

impl Disposition {
    fn can_store_amphipod(i: usize) -> bool {
        ![2, 4, 6, 8].contains(&i) && i < 11
    }
    fn get_suitable_room(a: Amphipod) -> usize {
        match a {
            Amber => 0,
            Bronze => 1,
            Copper => 2,
            Desert => 3,
        }
    }
    fn is_room_available(&self, room_idx: usize) -> bool {
        room_idx < 4
            && self.rooms[room_idx][0].is_none()
            && self.rooms[room_idx][1]
                .map(|a| room_idx == Self::get_suitable_room(a))
                .unwrap_or(true)
    }

    fn get_hallway_index_from_room(room_idx: usize) -> usize {
        (room_idx + 1) * 2
    }

    fn is_finished(&self) -> bool {
        (0..4).all(|i| self.is_room_finished(i))
    }

    fn is_room_finished(&self, room_idx: usize) -> bool {
        if room_idx >= 4 {
            panic!("room index is 0..4");
        }
        self.rooms[room_idx].iter().all(|apod| {
            apod.map(|apod| room_idx == Self::get_suitable_room(apod))
                .unwrap_or(false)
        })
    }
}

impl Display for Disposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("#############\n")?;
        f.write_str("#")?;
        for pod in &self.hallway {
            if let Some(apod) = pod {
                apod.fmt(f)?;
            } else {
                f.write_str(".")?;
            }
        }
        f.write_str("#\n")?;

        for j in 0..2 {
            f.write_str("  #")?;
            for i in 0..4 {
                if let Some(apod) = self.rooms[i][j] {
                    apod.fmt(f)?;
                } else {
                    f.write_str(".")?;
                }
                f.write_str("#")?;
            }
            f.write_str("\n")?;
        }
        f.write_str("")
    }
}

#[derive(Clone)]
struct Situation {
    score: usize,
    disposition: Disposition,

    /// debug only !
    previous_disposition: Vec<Disposition>,
}

impl Situation {
    /// provides a new Solution where an Amphipod have been moved from
    /// its start room to its target room (as deep as possible)
    fn room_to_room(&self, apod_loc: (usize, usize)) -> Option<Self> {
        // current room
        if self.disposition.is_room_finished(apod_loc.0) {
            return None;
        }

        if apod_loc.1 == 1 {
            if let Some(apod) = self.disposition.rooms[apod_loc.0][1] {
                if apod_loc.0 == Disposition::get_suitable_room(apod) {
                    // move is not necessary
                    return None;
                }
            }
            if self.disposition.rooms[apod_loc.0][0].is_some() {
                return None;
            }
        }

        if let Some(apod) = self.disposition.rooms[apod_loc.0][apod_loc.1] {
            let room_idx = Disposition::get_suitable_room(apod);
            let (hall_start, hall_end) = (
                Disposition::get_hallway_index_from_room(apod_loc.0),
                Disposition::get_hallway_index_from_room(room_idx),
            );
            let hallway_travel = min(hall_start, hall_end)..max(hall_start, hall_end) + 1;
            if self.disposition.is_room_available(room_idx)
                && self.disposition.hallway[hallway_travel.clone()]
                    .iter()
                    .all(Option::is_none)
            {
                let target_room_pos: usize = if self.disposition.rooms[room_idx][1].is_none() {
                    1
                } else {
                    0
                }; // TODO change is_room_available to get_room_best_place(Apod)->Option<usize>
                let score = self.score
                    + apod.get_single_move_cost()
                        * (apod_loc.1 + hallway_travel.len() + 1 + target_room_pos);
                let hallway = self.disposition.hallway;
                let mut rooms = self.disposition.rooms;
                rooms[apod_loc.0][apod_loc.1] = None;
                rooms[room_idx][target_room_pos] = Some(apod);

                let mut previous_disposition = self.previous_disposition.clone();
                previous_disposition.push(self.disposition);
                return Some(Self {
                    score,
                    disposition: Disposition { hallway, rooms },
                    previous_disposition,
                });
            }
        }
        None
    }

    fn hallway_to_room(&self, apod_loc: usize) -> Option<Self> {
        if let Some(apod) = self.disposition.hallway[apod_loc] {
            let room_idx = Disposition::get_suitable_room(apod);
            let (hall_start, hall_end) =
                (apod_loc, Disposition::get_hallway_index_from_room(room_idx));
            let hallway_travel = min(hall_start, hall_end)..max(hall_start, hall_end) + 1;

            // we're already on the hallway
            if self.disposition.is_room_available(room_idx)
                && self.disposition.hallway[hallway_travel.clone()]
                    .iter()
                    .filter(|occupant| occupant.is_none())
                    .count()
                    == 1
            {
                let target_room_pos: usize = if self.disposition.rooms[room_idx][1].is_none() {
                    1
                } else {
                    0
                };
                let score = self.score
                    + apod.get_single_move_cost() * (hallway_travel.len() + target_room_pos);
                let mut hallway = self.disposition.hallway;
                hallway[apod_loc] = None;
                let mut rooms = self.disposition.rooms;
                rooms[room_idx][target_room_pos] = Some(apod);
                let mut previous_disposition = self.previous_disposition.clone();
                previous_disposition.push(self.disposition);

                return Some(Self {
                    score,
                    disposition: Disposition { hallway, rooms },
                    previous_disposition,
                });
            }
        }
        None
    }

    // TODO : directly provide a Vec<Self> ?
    fn room_to_hallway(&self, apod_loc: (usize, usize), hall_target: usize) -> Option<Self> {
        // current room
        if self.disposition.is_room_finished(apod_loc.0) {
            return None;
        }
        if apod_loc.1 == 1 {
            if let Some(apod) = self.disposition.rooms[apod_loc.0][1] {
                if apod_loc.0 == Disposition::get_suitable_room(apod) {
                    // move is not necessary
                    return None;
                }
            }
            if self.disposition.rooms[apod_loc.0][0].is_some() {
                // move is not possible
                return None;
            }
        }

        if !Disposition::can_store_amphipod(hall_target) {
            return None;
        }

        if let Some(apod) = self.disposition.rooms[apod_loc.0][apod_loc.1] {
            let (hall_start, hall_end) = (
                Disposition::get_hallway_index_from_room(apod_loc.0),
                hall_target,
            );
            let hallway_travel = min(hall_start, hall_end)..max(hall_start, hall_end) + 1;
            if self.disposition.hallway[hallway_travel.clone()]
                .iter()
                .all(Option::is_none)
            {
                let score =
                    self.score + apod.get_single_move_cost() * (hallway_travel.len() + apod_loc.1);
                let mut hallway = self.disposition.hallway;
                hallway[hall_target] = Some(apod);
                let mut rooms = self.disposition.rooms;
                rooms[apod_loc.0][apod_loc.1] = None;
                let mut previous_disposition = self.previous_disposition.clone();
                previous_disposition.push(self.disposition);

                return Some(Self {
                    score,
                    disposition: Disposition { hallway, rooms },
                    previous_disposition,
                });
            }
        }

        None
    }
}

impl FromStr for Disposition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amphipods: Vec<_> = s
            .lines()
            .filter(|l| l.contains(&['A', 'B', 'C', 'D'][..]))
            .map(|l| {
                l.split('#')
                    .filter_map(|c| c.parse::<Amphipod>().ok())
                    .collect_vec()
            })
            .collect();

        if amphipods.len() != 2 {
            return Err(anyhow!("amphipods rooms seem malformed"));
        }

        let hallway = [None; 11];
        let mut rooms = [[None; 2]; 4];
        for (i, l) in amphipods.into_iter().enumerate() {
            for (j, amphipod) in l.into_iter().enumerate() {
                rooms[j][i] = Some(amphipod);
            }
        }

        Ok(Self { hallway, rooms })
    }
}

fn get_least_energy_to_organize_amphipods(start: &Disposition) -> usize {
    let mut best_score: Option<usize> = None;
    let mut best_situation: Option<Situation> = None;

    // an HashSet would be enough !!
    let mut best_situations: HashMap<Disposition, Situation> = HashMap::new();

    let mut current_situations = vec![Situation {
        score: 0,
        disposition: *start,
        previous_disposition: vec![],
    }];
    while !current_situations.is_empty() {
        let mut next_situations: HashMap<Disposition, Situation> =
            HashMap::with_capacity(current_situations.len());
        for situation in &current_situations {
            let mut local_next_situations: Vec<Situation> =
                Vec::with_capacity(current_situations.len());
            local_next_situations.append(
                &mut (0..situation.disposition.rooms.len())
                    .flat_map(|i| (0..2).filter_map(move |j| situation.room_to_room((i, j))))
                    .collect(),
            );

            if local_next_situations.is_empty() {
                local_next_situations.append(
                    &mut situation
                        .disposition
                        .hallway
                        .iter()
                        .enumerate()
                        .filter_map(|(i, _)| situation.hallway_to_room(i))
                        .collect(),
                );
            }

            if local_next_situations.is_empty() {
                local_next_situations.append(
                    &mut (0..situation.disposition.rooms.len())
                        .flat_map(|i| {
                            (0..2).flat_map(move |j| {
                                situation
                                    .disposition
                                    .hallway
                                    .iter()
                                    .enumerate()
                                    .filter_map(move |(z, _)| situation.room_to_hallway((i, j), z))
                            })
                        })
                        .collect(),
                );
            }

            //best
            // filter against best
            if let Some(local_best) = local_next_situations
                .iter()
                .filter(|s| s.disposition.is_finished())
                .min_by_key(|s| s.score)
            {
                println!("local best score {}", local_best.score);

                if best_score.unwrap_or(usize::MAX) > local_best.score {
                    best_score = Some(local_best.score);
                    best_situation = Some(local_best.clone());
                }

                println!("best_score is now {:?}", best_score);
            }

            for s in local_next_situations {
                if best_score.map(|score| score > s.score).unwrap_or(true)
                    && best_situations
                        .get(&s.disposition)
                        .map(|sit| sit.score > s.score)
                        .unwrap_or(true)
                {
                    best_situations.insert(s.disposition, s.clone());
                    // this way, next_situaitions are replaced on the fly if a better one is found
                    next_situations.insert(s.disposition, s.clone());
                }
            }
        }
        // FIXME : build a map situatiion => next_situations
        current_situations = next_situations.into_values().collect();
        println!("next evaluation : {} candidates", current_situations.len())
    }

    let best = best_situation.unwrap();
    for dispo in best.previous_disposition {
        println!("{}\n\n", dispo)
    }
    println!("{}\n\n", best.disposition);
    best_score.unwrap_or(usize::MAX)
}

pub fn organize_amphipods() {
    let input = "\
#############
#...........#
###D#A#C#A###
  #D#C#B#B#
  #########";

    let start: Disposition = input.parse().expect("failed to parse a Disposition");

    println!(
        "Least energy to organize amphipods: {}",
        get_least_energy_to_organize_amphipods(&start)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example_works() {
        let input = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########    ";
        let situation: Disposition = input.parse().expect("could not parse input as Situation");
        assert_eq!(12521, get_least_energy_to_organize_amphipods(&situation));
    }

    #[test]
    fn my_value_works() {
        let input = "\
#############
#...........#
###D#A#C#A###
  #D#C#B#B#
  #########  ";
        let situation: Disposition = input.parse().expect("could not parse input as Situation");
        assert!(19169 > get_least_energy_to_organize_amphipods(&situation));
    }
}
