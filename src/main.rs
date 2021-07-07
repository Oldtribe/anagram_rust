use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

/// CharCount holds a count of a single character
#[derive(PartialEq, Eq, Hash)]
pub struct CharCount {
    letter: char,
    count: u32,
}

impl fmt::Display for CharCount {
    /// Formatter for CharCount
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} times {})", self.count, self.letter)
    }
}
impl fmt::Debug for CharCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&self.count)
         .field(&self.letter)
         .finish()
    }
}
impl CharCount {
    /// Returns a new CharCount from a given letter
    fn new(letter: char) -> CharCount {
        CharCount {letter: letter, count: 1}
    }
}

/// CharList stores a list of letters and their counts.
/// The items in the list are guaranteed to be in order.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct CharList {
    length: u32,
    list: Vec<CharCount>,
}

impl CharList {
    /// create a new CharList
    fn new() -> CharList {
        CharList {length: 0, list: Vec::new()}
    }
    /// initialize with an existing charCount
    fn init(count: CharCount) -> CharList {
        let mut l = Vec::new();
        let length = count.count;
        l.push(count);
        CharList {length: length, list: l}
    }
    /// combine two CharLists into one new list.
    fn combine(first: CharList, second: CharList) -> CharList {
        let mut result = Vec::new();
        let mut iter1 = first.list.iter();
        let mut iter2 = second.list.iter();
        let mut item1 = iter1.next();
        let mut item2 = iter2.next();
        loop {
            match item1 {
                None => {
                    match item2 {
                        None => {
                            // both iterators are done, so we can return
                            return CharList {length: first.length + second.length, list: result}
                        },
                        Some(cc2) => {
                            // first is done, so just add this
                            result.push(CharCount {letter: cc2.letter, count: cc2.count});
                            item2 = iter2.next();
                        }
                    }
                },
                Some(cc1) => {
                    match item2 {
                        None => {
                            result.push(CharCount {letter: cc1.letter, count: cc1.count});
                            item1 = iter1.next();
                        },
                        Some(cc2) => {
                            // figure out which one is lower, and push just that
                            if cc1.letter < cc2.letter {
                                result.push(CharCount {letter: cc1.letter, count: cc1.count});
                                item1 = iter1.next();
                            } else if cc1.letter > cc2.letter {
                                result.push(CharCount {letter: cc2.letter, count: cc2.count});
                                item2 = iter2.next();
                            } else {
                                result.push( CharCount {letter: cc1.letter, count: cc1.count + cc2.count});
                                item1 = iter1.next();
                                item2 = iter2.next();
                            }
                        }
                    }
                }
            }
        }
    }
    /// create a CharList out of a String
    fn from_string(s: &str) -> CharList {
        let mut acc = CharList::new();
        for c in s.chars() {
            let cl = CharList::init(CharCount::new(c));
            acc = CharList::combine(acc, cl);
        }
        acc
    }
}
impl fmt::Display for CharList {
    /// Formatter for CharList
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.list)
    }
}

#[test]
fn combine_same() {
    let mut a: CharCount = CharCount::new('a');
    a.inc();
    let l1: CharList = CharList::init(a);
    let l2: CharList = CharList::init(CharCount {letter: 'a', count: 3});
    let l3: CharList = CharList::combine(l1, l2);
    assert!(l3.list.len() == 1);
    assert!(l3.list.get(0).unwrap().letter == 'a');
    assert!(l3.list.get(0).unwrap().count == 5);
}

#[test]
fn combine_different() {
    let l1: CharList = CharList::init(CharCount {letter: 'b', count: 3});
    let l2: CharList = CharList::init(CharCount {letter: 'a', count: 2});
    let l3: CharList = CharList::combine(l1, l2);
    assert!(l3.list.len() == 2);
    assert!(l3.list.get(0).unwrap().letter == 'a');
    assert!(l3.list.get(0).unwrap().count == 2);
    assert!(l3.list.get(1).unwrap().letter == 'b');
    assert!(l3.list.get(1).unwrap().count == 3);
}

#[test]
fn combine_into_middle() {
    let l1: CharList = CharList::init(CharCount {letter: 'c', count: 3});
    let l2: CharList = CharList::init(CharCount {letter: 'a', count: 2});
    let l3: CharList = CharList::combine(l1, l2);
    let l4: CharList = CharList::init(CharCount {letter: 'b', count: 1});
    let l5: CharList = CharList::combine(l4, l3);

    assert!(l5.list.len() == 3);
    assert!(l5.list.get(0).unwrap().letter == 'a');
    assert!(l5.list.get(0).unwrap().count == 2);
    assert!(l5.list.get(1).unwrap().letter == 'b');
    assert!(l5.list.get(1).unwrap().count == 1);
    assert!(l5.list.get(2).unwrap().letter == 'c');
    assert!(l5.list.get(2).unwrap().count == 3);
}

#[test]
fn from_string() {
    let l: CharList = CharList::from_string("01102010221");
    assert!(l.list.len() == 3);
    assert!(l.list.get(0).unwrap().letter == '0');
    assert!(l.list.get(0).unwrap().count == 4);
    assert!(l.list.get(1).unwrap().letter == '1');
    assert!(l.list.get(1).unwrap().count == 4);
    assert!(l.list.get(2).unwrap().letter == '2');
    assert!(l.list.get(2).unwrap().count == 3);
}

/// Enum MatchResult holds the result of subtracting one charlist from another
#[derive(PartialEq, Eq, Debug)]
pub enum MatchResult {
    NoMatch,
    FullMatch,
    PartialMatch(CharList)
}

impl CharList {
    /// subtract two CharLists 
    fn subtract(big: &CharList, small: &CharList) -> MatchResult {
        let mut result = Vec::new();
        let mut bigiter = big.list.iter();
        let mut smalliter = small.list.iter();
        let mut bigc = bigiter.next();
        let mut smallc = smalliter.next();
        loop {
            match bigc {
                None => {
                    match smallc {
                        None => {
                            // both iterators are done, so we can return a full match
                            if result.len() == 0 {
                                return MatchResult::FullMatch;
                            } else {
                                return MatchResult::PartialMatch(CharList {length: big.length - small.length, list: result});
                            }
                        },
                        Some(_) => {
                            return MatchResult::NoMatch
                        }
                    }
                },
                Some(cc1) => {
                    match smallc {
                        None => {
                            result.push(CharCount {letter: cc1.letter, count: cc1.count});
                            bigc = bigiter.next();
                        },
                        Some(cc2) => {
                            // figure out which one is lower, and push just that
                            if cc1.letter < cc2.letter {
                                result.push(CharCount {letter: cc1.letter, count: cc1.count});
                                bigc = bigiter.next();
                            } else if cc1.letter > cc2.letter {
                                return MatchResult::NoMatch
                            } else {
                                if cc1.count < cc2.count {
                                    return MatchResult::NoMatch
                                } else if cc1.count > cc2.count {
                                    result.push( CharCount {letter: cc1.letter, count: cc1.count - cc2.count});
                                    bigc = bigiter.next();
                                    smallc = smalliter.next();
                                } else {
                                    bigc = bigiter.next();
                                    smallc = smalliter.next();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    /// filter = like subtract, but a boolean result 
    fn filter(big: &CharList, small: &CharList) -> bool {
        let mut bigiter = big.list.iter();
        let mut smalliter = small.list.iter();
        let mut bigc = bigiter.next();
        let mut smallc = smalliter.next();
        loop {
            match bigc {
                None => {
                    match smallc {
                        None => {
                            return true;
                        },
                        Some(_) => {
                            return false;
                        }
                    }
                },
                Some(cc1) => {
                    match smallc {
                        None => {
                            bigc = bigiter.next();
                        },
                        Some(cc2) => {
                            // figure out which one is lower, and push just that
                            if cc1.letter < cc2.letter {
                                bigc = bigiter.next();
                            } else if cc1.letter > cc2.letter {
                                return false;
                            } else {
                                if cc1.count < cc2.count {
                                    return false;
                                } else {
                                    bigc = bigiter.next();
                                    smallc = smalliter.next();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

}

#[test]
fn subtract_nomatch_atall() {
    let b: CharList = CharList::from_string("abcde");
    let s: CharList = CharList::from_string("f");
    let m: MatchResult = CharList::subtract(&b, &s);
    assert!(m == MatchResult::NoMatch);
}

#[test]
fn subtract_nomatch_intheend() {
    let b: CharList = CharList::from_string("abcde");
    let s: CharList = CharList::from_string("abf");
    let m: MatchResult = CharList::subtract(&b, &s);
    assert!(m == MatchResult::NoMatch);
}

#[test]
fn subtract_nomatch_intheveryend() {
    let b: CharList = CharList::from_string("abcde");
    let s: CharList = CharList::from_string("abcdef");
    let m: MatchResult = CharList::subtract(&b, &s);
    assert!(m == MatchResult::NoMatch);
}

#[test]
fn subtract_fullmatch() {
    let b: CharList = CharList::from_string("abcde");
    let s: CharList = CharList::from_string("ebcda");
    let m: MatchResult = CharList::subtract(&b, &s);
    assert!(m == MatchResult::FullMatch);
}

#[test]
fn subtract_partialmatch() {
    let b: CharList = CharList::from_string("abcdef");
    let s: CharList = CharList::from_string("ebcda");
    let m: MatchResult = CharList::subtract(&b, &s);
    assert!(m == MatchResult::PartialMatch(CharList::from_string("f")));
}
#[test]
fn filter() {
    let b: CharList = CharList::from_string("abcdef");
    let s: CharList = CharList::from_string("ebcda");
    let m = CharList::filter(&b, &s);
    assert!(m);
}
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
struct Opt {
    /// the goal word to be anagrammatized
    #[structopt(short)]
    goal: String,
    /// The path to the file where words are
    #[structopt(short, parse(from_os_str))]
    wordfile: std::path::PathBuf,
    /// minimum length of a candidate word
    #[structopt(short = "m", default_value = "4")]
    minimum_candidate: usize,
    /// minimum count of words in one anagram
    #[structopt(short = "M", default_value = "5")]
    maximum_words_in_anagram: usize,
}

fn main() {

    let opt = Opt::from_args();
    let words = read_words(opt.wordfile, opt.minimum_candidate);
    let goal = CharList::from_string(&opt.goal);
    let mut candidates: Vec<&Box<CharList>> = Vec::new();
    for key in words.keys() {
        candidates.push(key)
    }
    let candidates = filter_candidates(&goal, candidates);
    let anagrams = anagram(&goal, candidates, opt.maximum_words_in_anagram);
    for set in anagrams {
        for clist in set {
            print!(" [");
            let wordset = words.get(clist).unwrap();
            for word in wordset {
                print!("{} ", word)
            }
            print!("]");
        }
        println!("");
    }

}

fn anagram<'a>(goal: &CharList, words: Vec<&'a Box<CharList>>, iteration_level: usize) -> Vec<Vec<&'a Box<CharList>>> {
    let mut results:  Vec<Vec<&Box<CharList>>> = Vec::new();
    if iteration_level == 0 {
        return results;
    }

    for (index, w) in words.iter().enumerate() {
        let m = CharList::subtract(goal, &**w);
        match m {
            MatchResult::NoMatch => (),
            MatchResult::FullMatch => {
                // add to results
                let v = vec![*w];
                results.push(v);
            },
            MatchResult::PartialMatch(remains) => {
                // create a new candidate list from words starting here, filtered
                let mut candidates = Vec::new();
                for newindex in index .. words.len() {
                    candidates.push(words[newindex]);
                }
                let candidates = filter_candidates(goal, candidates);
                let new_anagrams = anagram(&remains, candidates, iteration_level - 1);
                for news in new_anagrams {
                    let mut first = vec![*w];
                    for x in news {
                        first.push(x);
                    }
                    results.push(first);
                }
            }
        }
    }
    return results;
}

fn filter_candidates<'a>(goal: &CharList, candidates: Vec<&'a Box<CharList>>) -> Vec<&'a Box<CharList>>{
    let mut v: Vec<&Box<CharList>> = Vec::new();
    for c in candidates {
        if c.length <= goal.length {
            if CharList::filter(goal, &**c) {
                v.push(c);
            }
        }
    }
    // sort longest candidates to the front, this lessens the amount of backtracking
    v.sort_by(|c1, c2| c2.length.cmp(&c1.length));
    return v;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_words(filename: std::path::PathBuf, minimum_length: usize) -> HashMap<Box<CharList>, Vec<String>> {
    let mut map = HashMap::new();
    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(word) => {
                        if word.len() > minimum_length {
                            let key = Box::new(CharList::from_string(&word));
                            if !map.contains_key(&key) {
                                map.insert(key, vec![word]);
                            } else {
                                map.get_mut(&key).unwrap().push(word);
                            }
                        }
                    },
                    Err(_) => return map,
                }
            }
        }
        Err(_) => panic!("cannot open file"),
    };
    return map;
}


