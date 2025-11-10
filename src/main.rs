//! A program to generate anagrams
//!
//! # Input arguments
//! 
//! -g goalword
//! The word whose anagrams are searched for.
//!
//! -w wordfile
//! File that contains candidate words. Preferably in UTF-8 format.
//!
//! -m minimum_word_length
//! Use only candidate words that are at least this long.
//! Default value is 4 characters.
//!
//! -M maximum_candidates
//! For each anagram use at most this many candidate words.
//! Default 5 words.
//! Use value 1 for single-word anagrams.

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

use rayon::prelude::*;

pub mod charcount;
pub mod charlist;
pub mod acompare;

use charlist::CharList;
use charlist::MatchResult;

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
    /// maximum count of anagrams to print
    #[structopt(short = "c", default_value = "10")]
    maximum_anagrams: i32,
}

/// # Input arguments
/// 
/// -g goalword
/// The word whose anagrams are searched for.
///
/// -w wordfile
/// File that contains candidate words. Preferably in UTF-8 format.
///
/// -m minimum_word_length
/// Use only candidate words that are at least this long.
/// Default value is 4 characters.
///
/// -M maximum_candidates
/// For each anagram use at most this many candidate words.
/// Default 5 words.
/// Use value 1 for single-word anagrams.
pub fn main() {
    let opt = Opt::from_args();

    println!("Reading candidate words...");
    
    let words = read_words(opt.wordfile, opt.minimum_candidate);
    let goal = CharList::from_string(&opt.goal.to_lowercase());
    let mut candidates: Vec<&CharList> = Vec::new();
    for key in words.keys() {
        candidates.push(key)
    }
    
    println!("Creating anagrams...");

    let candidates = filter_and_sort_candidates(&goal, &candidates[..]);
    let anagrams = anagram(&goal, candidates, opt.maximum_words_in_anagram);

    println!("Sorting anagrams...");

    let mut all_anagrams = Vec::new();
    for a in anagrams {
        let strings = turn_into_strings(&a, &words);
        for s in strings.unwrap() {
            /*
            println!("{}", &s);
            */
            all_anagrams.push(s);
        }
    }
    let goalstring = opt.goal.to_string();

    struct AWithCount {
        count: usize,
        string: String
    }
    let mut sorted_anagrams = Vec::new();

    for string in all_anagrams {
        let ts = acompare::get_transpositions(goalstring.clone(), string.clone());
        let pts = ts.iter().map(|t| t).collect::<Vec<_>>();
        let count = acompare::greedy_score(&pts);        
        sorted_anagrams.push(AWithCount{count, string});
    }

    sorted_anagrams.sort_by(|c1, c2| {
        c2.count.cmp(&c1.count)
    });

    println!();

    let mut count = opt.maximum_anagrams;
    for a in sorted_anagrams {
        println!("{}", a.string);
        count = count - 1;
        if count <= 0 {
            break;
        }
    }

}

fn turn_into_strings(set: &[&CharList], words: &HashMap<Box<CharList>, Vec<String>>) -> Option<Vec<String>> {
    let rests = set.split_first();
    if let Some((first, rest)) = rests {
        let mut outs = Vec::new();
        let wordset = words.get(*first).unwrap();
        if let Some(trailing_strings) = turn_into_strings(rest, words) {
            for w in wordset {
                for s in &trailing_strings {
                    outs.push(format!("{} {}", w.clone(), s.clone()));
                }
            }
        } else {
            for w in wordset {
                outs.push(format!("{}", w.clone()));
            }
        }
        return Some(outs);
    }
    return None;
}

fn anagram<'a>(
    goal: &CharList,
    words: Vec<&'a CharList>,
    iteration_level: usize,
) -> Vec<Vec<&'a CharList>> {
    let results: Vec<Vec<&CharList>> = Vec::new();
    if iteration_level == 0 {
        return results;
    }

    let results = words
        .par_iter()
        .enumerate()
        .map(|(index, _)| {
            try_one_word(goal, &words[index..], iteration_level)
        })
        .flatten()
        .collect::<Vec<_>>();
    return results;
}

fn try_one_word<'a>(
    goal: &CharList,
    candidates: &[&'a CharList],
    iteration_level: usize,
) -> Vec<Vec<&'a CharList>> {
    let mut results: Vec<Vec<&CharList>> = Vec::new();
    let m = CharList::subtract(goal, candidates[0]);

    match m {
        MatchResult::NoMatch => (),
        MatchResult::FullMatch => {
            // add to results
            results.push(vec![candidates[0]]);
        }
        MatchResult::PartialMatch(remains) => {
            let word = candidates[0];
            let candidates = filter_candidates(goal, candidates);
            let new_anagrams = anagram(&remains, candidates, iteration_level - 1);
            for news in new_anagrams {
                let mut first = vec![word];
                for x in news {
                    first.push(x);
                }
                results.push(first);
            }
        }
    }
    return results;
}

fn filter_candidates<'a>(
    goal: &CharList,
    candidates: &[&'a CharList],
) -> Vec<&'a CharList> {
    let x = candidates
        .iter()
        .cloned()
        .filter(|&c| c.length() <= goal.length() && CharList::may_be_contained(goal, c))
        .collect::<Vec<_>>();
    return x;
}

fn filter_and_sort_candidates<'a>(
    goal: &CharList,
    candidates: &[&'a CharList],
) -> Vec<&'a CharList> {
    let mut x = candidates
        .iter()
        .cloned()
        .filter(|&c| c.length() <= goal.length() && CharList::may_be_contained(goal, c))
        .collect::<Vec<_>>();

    // sort longest candidates to the front, this lessens the amount of backtracking
    x.sort_by(|c1, c2| c2.length().cmp(&c1.length()));

    return x;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// read_words reads a file of words, then builds a CharList of each.
// it returns a HashMap where the CharList of each word is the key, and a vector of all words that have this CharList are the value.
// This way, anagrammatic single words like 'karies', 'rieska' and 'eskari' occupy one slot in the HashMap.
fn read_words(
    filename: std::path::PathBuf,
    minimum_length: usize,
) -> HashMap<Box<CharList>, Vec<String>> {
    let mut map = HashMap::new();
    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(word) => {
                        if word.len() >= minimum_length {
                            let key = Box::new(CharList::from_string(&word.to_lowercase()));
                            let candidates = map.get_mut(&key);
                            match candidates {
                                // Key does not exist. add it
                                None => {
                                    map.insert(key, vec![word]);
                                }
                                // Key exists, append word to the value vector
                                Some(words) => {
                                    if !words.contains(&word) {
                                        words.push(word)
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => return map,
                }
            }
        }
        Err(_) => panic!("cannot open file"),
    };
    return map;
}
