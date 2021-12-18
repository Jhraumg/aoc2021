#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small(&'static str),
    Large(&'static str),
}

use std::iter::once;
use std::ops::Index;

use itertools::Itertools;
use Cave::*;
impl Cave {
    pub fn parse(name: &'static str) -> Result<Cave, String> {
        match name.trim() {
            "start" => Ok(Start),
            "end" => Ok(End),
            "" => Err("empty value !".to_string()),
            lower if lower.chars().all(|c| c.is_lowercase()) => Ok(Small(lower)),
            upper if upper.chars().all(|c| c.is_uppercase()) => Ok(Large(upper)),
            unkown => Err(format!("'{}' is not a valid Cave name", unkown)),
        }
    }
}

struct CavesMap {
    caves: Vec<Cave>,

    /// associate 2 caves by their indexes, the lower first
    connections_by_index: Vec<(usize, usize)>,

    start_index: usize,
    end_index: usize,
}

impl<'m> Index<usize> for &'m CavesMap {
    type Output = Cave;

    fn index(&self, index: usize) -> &'m Self::Output {
        &self.caves[index]
    }
}

#[derive(Clone)]
struct Path {
    small_caves_index: Vec<usize>,
    current_index: usize,

    // this is really specific, but avcoids itering over small_caves_index
    small_cave_visited_twice: bool,
}

impl Path {
    pub fn new() -> Self {
        Self {
            small_caves_index: vec![],
            current_index: 0,
            small_cave_visited_twice: false,
        }
    }
}

impl CavesMap {
    fn parse_connection(line: &'static str) -> Result<(Cave, Cave), String> {
        if let Some((cave1, cave2)) = line
            .split('-')
            .filter_map(|name| Cave::parse(name).ok())
            .collect_tuple()
        {
            Ok((cave1, cave2))
        } else {
            Err(format!("cannot parse connection '{}'", line))
        }
    }

    pub fn parse(input: &'static str) -> Self {
        let (mut start, mut end) = (None, None);

        let connections: Vec<_> = input
            .lines()
            .filter_map(|l| Self::parse_connection(l).ok())
            .collect();

        let caves: Vec<_> = connections
            .iter()
            .flat_map(|(cave1, cave2)| once(*cave1).chain(once(*cave2)))
            .unique()
            .collect();

        for (i, cave) in caves.iter().enumerate() {
            if start.and(end).is_some() {
                break;
            }
            match cave {
                Start => {
                    start.replace(i);
                }
                End => {
                    end.replace(i);
                }
                _ => {}
            }
        }

        let get_index = |cave: &Cave| {
            caves
                .iter()
                .enumerate()
                .filter(|(_, c)| **c == *cave)
                .map(|(i, _)| i)
                .next()
                .unwrap()
        };

        let connections_by_index: Vec<_> = connections
            .iter()
            .map(|(cave1, cave2)| (get_index(cave1), get_index(cave2)))
            .collect();

        let start_index = start.unwrap();
        let end_index = end.unwrap();

        Self {
            caves,
            connections_by_index,
            start_index,
            end_index,
        }
    }

    fn get_connected_indexes(&self, cave_index: usize) -> Vec<usize> {
        self.connections_by_index
            .iter()
            .filter_map(|c| match c {
                // let's avoid infinite loop
                (i1, i2) if *i1 == *i2 => None,
                (src, other) if *src == cave_index => Some(*other),
                (other, src) if *src == cave_index => Some(*other),
                _ => None,
            })
            .collect()
    }

    fn count_pathes(&self, small_cave_selector: impl Fn(&Path, usize) -> bool) -> usize {
        // let's try to count without building actual path
        let mut pathes = vec![Path {
            current_index: self.start_index,
            ..Path::new()
        }];
        let mut pathes_count = 0;
        loop {
            let new_pathes: Vec<_> = pathes
                .into_iter()
                .flat_map(|p| {
                    // collecting here avoids keeping filter too long
                    let path_options: Vec<_> = self
                        .get_connected_indexes(p.current_index)
                        .into_iter()
                        .filter(|i| {
                            if let Small(_c) = self[*i] {
                                small_cave_selector(&p, *i)
                            } else {
                                true
                            }
                        })
                        .collect();

                    path_options
                        .into_iter()
                        .filter_map(move |o_idx| match self[o_idx] {
                            End => Some(Path {
                                current_index: self.end_index,
                                ..p.clone()
                            }),
                            // TODO : copy only when modified
                            Large(_) => Some(Path {
                                current_index: o_idx,
                                ..p.clone()
                            }),
                            Small(_) => {
                                let mut small_caves_index = p.small_caves_index.clone();
                                small_caves_index.push(o_idx);
                                Some(Path {
                                    small_caves_index,
                                    current_index: o_idx,
                                    small_cave_visited_twice: p.small_cave_visited_twice
                                        || p.small_caves_index.contains(&o_idx),
                                })
                            }
                            _ => None,
                        })
                })
                .collect();

            let new_pathes_len = new_pathes.len();
            pathes = new_pathes
                .into_iter()
                .filter(|p| !matches!(self[p.current_index], End))
                .collect();

            // filtered out pathes are new finished ones
            pathes_count += new_pathes_len - pathes.len();

            if pathes.is_empty() {
                break;
            }
        }
        pathes_count
    }
}

fn count_pathes(input: &'static str) -> usize {
    let map = CavesMap::parse(input);

    map.count_pathes(|p: &Path, i| !p.small_caves_index.contains(&i))
}

fn count_pathes_twice_visited(input: &'static str) -> usize {
    let map = CavesMap::parse(input);

    map.count_pathes(|p: &Path, i| !p.small_caves_index.contains(&i) || !p.small_cave_visited_twice)
}

pub fn display_pathes() {
    let input = include_str!("../ressources/day12_connections.txt");
    println!("number of pathes : {}", count_pathes(input));
    println!(
        "number of path while visiting twice small places : {}",
        count_pathes_twice_visited(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cave_can_be_parsed() {
        assert_eq!(Ok(Start), Cave::parse("start"));
        assert_eq!(Ok(End), Cave::parse("end"));
        assert_eq!(Ok(Small("smallcave")), Cave::parse("smallcave"));
        assert_eq!(Ok(Large("BIGCAVE")), Cave::parse("BIGCAVE"));
        assert!(Cave::parse("MediumCave").is_err());
        assert!(Cave::parse("not_a_cave").is_err());
        assert!(Cave::parse("   ").is_err());
    }

    #[test]
    fn check_aoc_example() {
        let simple_input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(10, count_pathes(simple_input));
        assert_eq!(36, count_pathes_twice_visited(simple_input));

        let larger_input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(19, count_pathes(larger_input));

        let largest_input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        assert_eq!(226, count_pathes(largest_input));
    }
}
