use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::slice::Iter;
use itertools::Itertools;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
use Wire::{A,B,C,D,E,F,G};
impl Wire {

    // Should get proper error
    pub fn parse(v : &str)->Result<Self, String > {
        match v {
            "a" => Ok(A),
            "b" => Ok(B),
            "c" => Ok(C),
            "d" => Ok(D),
            "e" => Ok(E),
            "f" => Ok(F),
            "g" => Ok(G),
            unknown => Err(format!("'{}' cannot be parsed to Wire",unknown))
        }
    }

    pub fn iter() -> Iter<'static,Wire > {
        static VALUES : [Wire;7] = [A, B, C, D, E, F, G];
        VALUES.iter()
    }

}

impl PartialOrd<Self> for Wire {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Wire  {
    fn cmp(&self, other: &Self) -> Ordering {
        let index = Wire::iter().enumerate().find(|(i,w)|*w==self).unwrap().0;
        let other_index = Wire::iter().enumerate().find(|(i,w)|*w==other).unwrap().0;
        index.cmp(&other_index)
    }
}

// TODO : just use a [[bool;7];10] to store the mapping ?

fn get_wires(digit: usize) -> Vec<Wire> {
    match digit {
        0 => vec![A, B, C, E, F, G],
        1 => vec![C, F],
        2 => vec![A, C, D, E, G],
        3 => vec![A, C, D, F, G],
        4 => vec![B, C, D, F],
        5 => vec![A, B, D, F, G],
        6 => vec![A, B, D, E, F, G],
        7 => vec![A, C, F],
        8 => vec![A, B, C, D, E, F, G],
        9 => vec![A, B, C, D, F, G],
        _ => panic!("{} is not a valid digit", digit),
    }
}

fn segments_to_value(mut segments: Vec<Wire>) -> usize {
    segments.sort();
    for i in 0..10 {
        if get_wires(i) == segments {
            return i;
        }
    }
    panic!("{:?} is not a valid digit", segments);
}

fn get_possible_digits_from_number_of_wire(wires_count:usize )->Vec<usize> {
    let number_of_wires_by_digits : HashMap<usize,usize> = (0..10).map(|d|(d, get_wires(d).len())).collect();
    number_of_wires_by_digits
        .iter()
        .filter(|&(_digit,count)| *count==wires_count )
        .map(|(digit, _count)|*digit)
        .collect()
}

// idée générale :
// on construit une map wire => [potential wires] (tous au départ)
// à partir de chaque group
//   => nb wires => [digits]
//  pour chacune des entrées de ce groupe, on réduit la liste des candidats à partir des transcriptions des wires
// quand un wire a un seul match, on retire ce match dees autres wire (récursif)

struct  DisplaysSource {
    /// segments associated with (unordered) digits
    digits_segments : Vec<Vec<Wire>>,

    /// 4 digits represented by their output segments
    output_segments :  Vec<Vec<Wire>>,

}

impl DisplaysSource {

    fn parse_wires_group(wires :&str) -> Vec<Wire> {
        wires.split("").filter_map(|c|Wire::parse(c).ok()).collect()
    }
    pub fn parse(input :&str) -> Option<Self> {
        input.split("|").collect_tuple().map(|(digits,result)| {
            let digits_segments: Vec<_> = digits.split_whitespace().map(|seg_seq| Self::parse_wires_group(seg_seq)).collect();

            let output_segments: Vec<_> = result.split_whitespace().map(|seg_seq| Self::parse_wires_group(seg_seq)).collect();

            Self { digits_segments, output_segments }
        })
    }
    pub fn decode(&self) -> usize {

        let wire_to_length:Vec<_> = self.digits_segments
            .iter()
            .flat_map(|segments| {
                let len = segments.len();
                segments.iter().map(|w| (*w, len)).collect_vec()
            })
            .collect();

        let mut wire_to_digits : HashMap<Wire,Vec<usize>> =Wire::iter().map(|w|(*w,vec![])).collect();
        let mut digit_to_source_wires:HashMap<usize, HashSet<Wire>> = (0..10).map(|i|(i, HashSet::new())).collect();

        // stores all indexes than could match the digit
        let mut digit_to_displayed_index : HashMap<usize, HashSet<usize>> = (0..10).map(|i|(i, HashSet::new())).collect();


        for (idx, segments) in self.digits_segments.iter().enumerate() {
            let len =segments.len();
            for wire in segments {
                wire_to_digits.get_mut(wire).unwrap().append(&mut get_possible_digits_from_number_of_wire(len));

                for i in 0..10 {
                    if get_wires(i).len() == len {
                        //digit_to_source_wire will contains more candidates than actual
                        digit_to_source_wires.get_mut(&i).unwrap().insert(*wire);
                        digit_to_displayed_index.get_mut(&i).unwrap().insert(idx);
                    }
                }
            }

        }

        dbg!(&wire_to_digits);

        // This will stores all possible associations
        // let's init it from the digit each wire can map
        let mut wires_to_wire :HashMap<Wire,HashSet<Wire>> = Wire::iter()
            .map(|w| {
                let wires :HashSet<_>=wire_to_digits.get(w).unwrap()
                    .iter()
                    .flat_map(|d| get_wires(*d))
                    .collect();
                (*w, wires)
            })
            .collect();

        // then, let's remove wire of the digits that can't be mapped
        for j in 0..10 {
            for (wire, candidates) in wires_to_wire.iter_mut() {
                if !wire_to_digits.get(wire).unwrap().contains(&j) {
                    println!("removing {} digits from {:?} wire",j,wire);
                    for candidate in get_wires(j) {
                        candidates.take(&candidate);
                    }
                }
            }
        }



        for _ in 0..10 { // max 10 iterrations since there are 10 unknown
            // let's work with digits whose wire set is fully known

            let known_digits :Vec<_>= digit_to_source_wires.iter().filter(|(d,wires)|wires.len() == get_wires(**d).len()).collect();
            let almost_known_digits :Vec<_>= digit_to_source_wires.iter().filter(|(d,wires)|wires.len() +1 == get_wires(**d).len()).collect();
            dbg!(known_digits.len());
            dbg!(almost_known_digits.len());
            for (known, sources) in known_digits{
                let actual_wires = get_wires(*known);

                for (wire, candidates) in wires_to_wire.iter_mut() {
                    // (sources.contains && actual_wires.contains) or (!sources.contains && !actual_wires.contains)
                    // candidates.retain(|c|! (sources.contains(wire)^actual_wires.contains(c)));
                    let len =candidates.len();
                    if sources.contains(wire) {
                        candidates.retain(|c|actual_wires.contains(c));
                    }else {
                        candidates.retain(|c|!actual_wires.contains(c));
                    }
                    if len > candidates.len() {
                        println!("removed {} wires from {:?} candidates", len -candidates.len(),wire);
                    }
                }
            }

            // now, let's remove resolved mappings from other wires candidates
            let resolved_wires :Vec<(Wire,Wire)> = wires_to_wire.iter()
                .filter(|(wire,translations)|translations.len()==1)
                .map(|(wire,translations)|(*wire,*translations.iter().next().unwrap()))
                .collect();
            if dbg!(resolved_wires.len()) == wires_to_wire.len(){
                break;
            }
            for (displayed, actual) in resolved_wires{
                // direct Mapping
                for (wire, candidates) in wires_to_wire.iter_mut(){
                    if displayed!=*wire {
                        if candidates.take(&actual).is_some(){
                            println!("removing {:?} from {:?} candidates", &actual, wire);
                        }
                    }
                }
                // Digit mapping :  todo build from digit_to_sisplay_index ?
                for (digit, sources) in digit_to_source_wires.iter_mut(){
                    if !get_wires(*digit).contains(&actual){
                        if sources.take(&displayed).is_some(){
                            println!("removing {:?} from {:?} sources", &displayed, digit);
                        }
                    }
                }
                for (digit, display_indexes) in digit_to_displayed_index.iter_mut(){
                    if !get_wires(*digit).contains(&actual) {
                        // let's remove all indexes which cant be associated with the target digit anymore
                        let len =display_indexes.len();
                        display_indexes.retain(|idx|! self.digits_segments[*idx].contains(&displayed));
                        if len > display_indexes.len() {
                            println!("{:?} possible index removed from {:?}", len -display_indexes.len(), digit);
                        }
                    }
                }

                // should be done at once, but let's move on
                for (digit, display_indexes) in &digit_to_displayed_index{

                    let wires = digit_to_source_wires.get_mut(digit).unwrap();
                    let all_possible_index :Vec<_>= display_indexes.iter().flat_map(|idx|self.digits_segments[*idx].iter()).collect();
                    let len = wires.len();
                    wires.retain(|f| all_possible_index.contains(&f));
                    if len > wires.len() {
                        println!("{:?} possible wires removed from {:?}", len -wires.len(), digit);
                    }

                }
                for (digit, display_indexes) in &digit_to_displayed_index{
                    let acceptable_wires=get_wires(*digit);
                    for w in Wire::iter(){
                        if display_indexes.iter().map(|idx| &self.digits_segments[*idx]).all(|segments|segments.contains(w)) {
                            wires_to_wire.get_mut(w).unwrap().retain(|s|acceptable_wires.contains(s));
                        }
                    }


                }



                println!("correspondance possibles entre valeurs");
                for (digit,indexes) in &digit_to_displayed_index{
                    println!("  {}",digit);
                    for i in indexes {
                        println!("    {:?}", &self.digits_segments[*i]);
                    }

                }
            }
        }
        if wires_to_wire.iter().any(|(_,translations)|translations.len()!=1) {
            panic!("exit without resolution for {:?}", wires_to_wire);
        }

        let dic :HashMap<Wire,Wire> = wires_to_wire.iter()
            .map(|(w,translations)|(*w,*translations.iter().next().unwrap()))
            .collect();

        let mut result = 0;
        for segments in &self.output_segments {
            let translated :Vec<_>= segments.iter().map(|w|*dic.get(w).unwrap()).collect();
            result = 10*result + segments_to_value(translated); // could sort at reading
        }

        result

    }
}

fn parse_displays(input :&str) -> Vec<DisplaysSource> {
    input.lines().filter_map(|line| DisplaysSource::parse(line)).collect()
}

fn count_unique_numbers(sources: &Vec<DisplaysSource>) -> usize {

    let unique_lengths :HashSet<_> = [1, 4, 7, 8].iter().map(|d|get_wires(*d).len()).collect();

    sources
        .iter()
        .map(|ds|
            ds.output_segments
                .iter()
                .filter(|digit| {
                    let digit_len = digit.len();
                    unique_lengths.contains(&digit_len)
                })
                .count())
        .sum()

}
fn sum_decoded(sources: &Vec<DisplaysSource>) -> usize {
    sources.iter().map(|source|source.decode()).sum()
}

pub fn display_digits(){
    let input = include_str!("day8_digits_displays.txt");
    let sources = parse_displays(input);


    println!("number of unique numbers : {}", count_unique_numbers(&sources));
    println!("sum of decoded values : {}",sum_decoded(&sources));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_decode_line() {
        let line =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |cdfeb fcadb cdfeb cdbaf";

        let source = DisplaysSource::parse(line).unwrap();
        source.decode();

    }


    #[test]
    fn aoc_example_works() {
        let input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce";

        let sources = parse_displays(input);

        assert_eq!(26, count_unique_numbers(&sources));
        assert_eq!(61229, sum_decoded(&sources));
    }
}
