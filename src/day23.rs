use crate::day23::Amphipod::{Amber, Bronze, Copper, Desert};
use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
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
struct Disposition<const ROOM_LENGTH: usize> {
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; ROOM_LENGTH]; 4],
}

impl<const ROOM_LENGTH: usize> Disposition<ROOM_LENGTH> {
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
            && self.rooms[room_idx][1..ROOM_LENGTH].iter().all(|a| {
                a.map(|a| room_idx == Self::get_suitable_room(a))
                    .unwrap_or(true)
            })
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
    fn get_first_available_place(&self, room_idx: usize) -> Option<usize> {
        for (i, place) in self.rooms[room_idx].iter().enumerate().rev() {
            match place {
                Some(p) if Self::get_suitable_room(*p) != room_idx => {
                    return None;
                }
                None => return Some(i),
                _ => {}
            }
        }
        None
    }
}

impl<const ROOM_LENGTH: usize> Display for Disposition<ROOM_LENGTH> {
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

        for j in 0..ROOM_LENGTH {
            f.write_str("  #")?;
            for i in 0..4 {
                if let Some(apod) = self.rooms[i][j] {
                    apod.fmt(f)?;
                } else {
                    f.write_str(".")?;
                }
                f.write_str("#")?;
            }
            f.write_str("  \n")?;
        }
        f.write_str("  #########  ")
    }
}

#[derive(Clone)]
struct Situation<const ROOM_LENGTH: usize> {
    score: usize,
    disposition: Disposition<ROOM_LENGTH>,

    #[cfg(debug_assertions)]
    previous_disposition: Vec<Disposition<ROOM_LENGTH>>,
}

impl<const ROOM_LENGTH: usize> Display for Situation<ROOM_LENGTH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[cfg(debug_assertions)]
        {
            let descriptions: Vec<String> = self
                .previous_disposition
                .iter()
                .map(|d| format!("{}", d))
                .collect();

            let flat_description = (0..descriptions[0].lines().count())
                .map(|i| {
                    // FIXME respliting each time is higly inneffective, but we"re not supposed to display these
                    descriptions
                        .iter()
                        .map(|descr| descr.lines().nth(i).unwrap_or(""))
                        .join("  ")
                })
                .join("\n");

            f.write_str(&flat_description)?;
        }

        f.write_fmt(format_args!("\n{}\n", self.score))?;
        self.disposition.fmt(f)
    }
}

impl<const ROOM_LENGTH: usize> Situation<ROOM_LENGTH> {
    // source is used in debug mode to log all successive steps
    fn new(score: usize, disposition: Disposition<ROOM_LENGTH>, source: &Self) -> Self {
        #[cfg(debug_assertions)]
        let mut previous_disposition = source.previous_disposition.clone();
        #[cfg(debug_assertions)]
        previous_disposition.push(source.disposition);

        Self {
            score,
            disposition,
            #[cfg(debug_assertions)]
            previous_disposition,
        }
    }
    fn can_move_out_of_room(&self, apod_loc: (usize, usize)) -> bool {
        // current room is finished
        if self.disposition.is_room_finished(apod_loc.0) {
            return false;
        }

        // cannot pass through top Amphipods
        if self.disposition.rooms[apod_loc.0][0..apod_loc.1]
            .iter()
            .any(Option::is_some)
        {
            return false;
        }
        // Amphipod is already in its room, which also contains only its siblings
        if self.disposition.is_room_available(apod_loc.0) {
            return false;
        }
        true
    }

    /// provides a new Solution where an Amphipod have been moved from
    /// its start room to its target room (as deep as possible)
    /// can be provided thanks to the 2 other moves, but this
    /// reduce the number of possibilities
    fn room_to_room(&self, apod_loc: (usize, usize)) -> Option<Self> {
        if !self.can_move_out_of_room(apod_loc) {
            return None;
        }

        if let Some(apod) = self.disposition.rooms[apod_loc.0][apod_loc.1] {
            let room_idx = Disposition::<ROOM_LENGTH>::get_suitable_room(apod);
            let (hall_start, hall_end) = (
                Disposition::<ROOM_LENGTH>::get_hallway_index_from_room(apod_loc.0),
                Disposition::<ROOM_LENGTH>::get_hallway_index_from_room(room_idx),
            );
            let hallway_travel = min(hall_start, hall_end)..max(hall_start, hall_end) + 1;

            if self.disposition.hallway[hallway_travel.clone()]
                .iter()
                .all(Option::is_none)
            {
                if let Some(target_room_pos) = self.disposition.get_first_available_place(room_idx)
                {
                    let score = self.score
                        + apod.get_single_move_cost()
                            * (apod_loc.1 + hallway_travel.len() + 1 + target_room_pos);
                    let hallway = self.disposition.hallway;
                    let mut rooms = self.disposition.rooms;
                    rooms[apod_loc.0][apod_loc.1] = None;
                    rooms[room_idx][target_room_pos] = Some(apod);

                    return Some(Self::new(score, Disposition { hallway, rooms }, self));
                }
            }
        }
        None
    }

    fn hallway_to_room(&self, apod_loc: usize) -> Option<Self> {
        if let Some(apod) = self.disposition.hallway[apod_loc] {
            let room_idx = Disposition::<ROOM_LENGTH>::get_suitable_room(apod);
            let (hall_start, hall_end) = (
                apod_loc,
                Disposition::<ROOM_LENGTH>::get_hallway_index_from_room(room_idx),
            );
            let hallway_travel = min(hall_start, hall_end)..max(hall_start, hall_end) + 1;

            // we're already on the hallway => that makes 1 place occupied
            if self.disposition.hallway[hallway_travel.clone()]
                .iter()
                .filter(|occupant| !occupant.is_none())
                .count()
                == 1
            {
                if let Some(target_room_pos) = self.disposition.get_first_available_place(room_idx)
                {
                    let score = self.score
                        + apod.get_single_move_cost() * (hallway_travel.len() + target_room_pos);
                    let mut hallway = self.disposition.hallway;
                    hallway[apod_loc] = None;
                    let mut rooms = self.disposition.rooms;
                    rooms[room_idx][target_room_pos] = Some(apod);
                    return Some(Self::new(score, Disposition { hallway, rooms }, self));
                }
            }
        }
        None
    }

    fn room_to_hallway(&self, apod_loc: (usize, usize), hall_target: usize) -> Option<Self> {
        if !self.can_move_out_of_room(apod_loc) {
            return None;
        }

        if !Disposition::<ROOM_LENGTH>::can_store_amphipod(hall_target) {
            return None;
        }

        if let Some(apod) = self.disposition.rooms[apod_loc.0][apod_loc.1] {
            let (hall_start, hall_end) = (
                Disposition::<ROOM_LENGTH>::get_hallway_index_from_room(apod_loc.0),
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

                return Some(Self::new(score, Disposition { hallway, rooms }, self));
            }
        }

        None
    }
}

impl<const ROOM_LENGTH: usize> FromStr for Disposition<ROOM_LENGTH> {
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

        if amphipods.len() != ROOM_LENGTH {
            return Err(anyhow!("amphipods rooms seem malformed"));
        }

        let hallway = [None; 11];
        let mut rooms = [[None; ROOM_LENGTH]; 4];
        for (i, l) in amphipods.into_iter().enumerate() {
            for (j, amphipod) in l.into_iter().enumerate() {
                rooms[j][i] = Some(amphipod);
            }
        }

        Ok(Self { hallway, rooms })
    }
}

// TODO : just use dyn disptach inside Situation
fn get_least_energy_to_organize_amphipods<const ROOM_LENGTH: usize>(
    start: &Disposition<ROOM_LENGTH>,
) -> usize {
    let mut best_finished: Option<Situation<ROOM_LENGTH>> = None;
    let mut best_score: Option<usize> = None;

    let mut best_situations: HashMap<Disposition<ROOM_LENGTH>, Situation<ROOM_LENGTH>> =
        HashMap::new();

    let mut current_situations = vec![Situation {
        score: 0,
        disposition: *start,
        #[cfg(debug_assertions)]
        previous_disposition: vec![],
    }];
    while !current_situations.is_empty() {
        let mut next_situations: HashMap<Disposition<ROOM_LENGTH>, Situation<ROOM_LENGTH>> =
            HashMap::with_capacity(current_situations.len());

        for situation in &current_situations {
            let mut local_next_situations: Vec<Situation<ROOM_LENGTH>> =
                Vec::with_capacity(current_situations.len());

            // ROOM to ROOM
            local_next_situations.append(
                &mut (0..situation.disposition.rooms.len())
                    .flat_map(|i| {
                        (0..ROOM_LENGTH).filter_map(move |j| situation.room_to_room((i, j)))
                    })
                    .collect(),
            );

            // HALLWAY TO ROOM
            if local_next_situations.is_empty() {
                local_next_situations.append(
                    &mut (0..11)
                        .filter_map(move |i| situation.hallway_to_room(i))
                        .collect(),
                );
            }

            // ROOM_TO_HALLWAY
            if local_next_situations.is_empty() {
                local_next_situations.append(
                    &mut (0..situation.disposition.rooms.len())
                        .flat_map(|i| {
                            (0..ROOM_LENGTH).flat_map(move |j| {
                                (0..11).filter_map(move |z| situation.room_to_hallway((i, j), z))
                            })
                        })
                        .collect(),
                );
            }

            //best
            if let Some(local_best) = local_next_situations
                .iter()
                .filter(|s| s.disposition.is_finished())
                .min_by_key(|s| s.score)
            {
                if best_score.unwrap_or(usize::MAX) > local_best.score {
                    best_finished = Some(local_best.clone());
                    best_score = Some(local_best.score);
                }
            }

            // filter against known bests
            for s in local_next_situations {
                if !s.disposition.is_finished()
                    && best_score
                        .map(|best_score| best_score > s.score)
                        .unwrap_or(true)
                    && best_situations
                        .get(&s.disposition)
                        .map(|sit| sit.score > s.score)
                        .unwrap_or(true)
                {
                    best_situations.insert(s.disposition, s.clone());
                    // this way, next_situations are replaced on the fly if a better one is found
                    next_situations.insert(s.disposition, s.clone());
                }
            }
        }
        current_situations = next_situations.into_values().collect();
        // eprintln!("next evaluation : {} candidates", current_situations.len())
    }

    #[cfg(debug_assertions)]
    if let Some(best) = best_finished {
        eprintln!("{}\n", best);
    }

    best_score.unwrap_or(usize::MAX)
}

pub fn organize_amphipods() {
    let input = "\
#############
#...........#
###D#A#C#A###
  #D#C#B#B#
  #########";

    let start: Disposition<2> = input.parse().expect("failed to parse a Disposition");

    println!(
        "Least energy to organize amphipods: {}",
        get_least_energy_to_organize_amphipods(&start)
    );

    let unfold_input = "\
#############
#...........#
###D#A#C#A###
  #D#C#B#A#
  #D#B#A#C#
  #D#C#B#B#
  #########";

    let new_start: Disposition<4> = unfold_input.parse().expect("failed to parse a Disposition");

    println!(
        "Least energy to organize all amphipods: {}",
        get_least_energy_to_organize_amphipods(&new_start)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_place_are_vailable_in_order() {
        let mut disposition: Disposition<2> = Disposition {
            hallway: [None; 11],
            rooms: [[None; 2]; 4],
        };
        disposition.rooms[1][1] = Some(Bronze);
        disposition.rooms[2][1] = Some(Copper);
        disposition.rooms[2][0] = Some(Copper);
        disposition.rooms[3][1] = Some(Amber);

        assert_eq!(Some(1), disposition.get_first_available_place(0));
        assert_eq!(Some(0), disposition.get_first_available_place(1));
        assert_eq!(None, disposition.get_first_available_place(2));
        assert_eq!(None, disposition.get_first_available_place(3));
    }

    #[test]
    fn aoc_example_works() {
        let input = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########    ";
        let start: Disposition<2> = input.parse().expect("could not parse input as Situation");
        assert_eq!(12521, get_least_energy_to_organize_amphipods(&start));

        let input = "\
#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########  ";
        let start: Disposition<4> = input.parse().expect("could not parse input as Situation");
        assert_eq!(44169, get_least_energy_to_organize_amphipods(&start));
    }
}
