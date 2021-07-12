use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

pub mod charcount;
pub mod charlist;

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
}

#[tokio::main]
pub async fn main() {
    let opt = Opt::from_args();
    let words = read_words(opt.wordfile, opt.minimum_candidate);
    let goal = CharList::from_string(&opt.goal.to_lowercase());
    let mut candidates: Vec<&Box<CharList>> = Vec::new();
    for key in words.keys() {
        candidates.push(key)
    }
    let candidates = filter_candidates(&goal, &candidates[..]);
    let anagrams = anagram_async(&goal, candidates, opt.maximum_words_in_anagram).await;
    for set in anagrams {
        for clist in set {
            print!(" [ ");
            let wordset = words.get(clist).unwrap();
            for word in wordset {
                print!("{} ", word)
            }
            print!("]");
        }
        println!("");
    }

}

async fn anagram_async<'a>(
    goal: &CharList,
    words: Vec<&'a Box<CharList>>,
    iteration_level: usize,
) -> Vec<Vec<&'a Box<CharList>>> {
    let mut results: Vec<Vec<&Box<CharList>>> = Vec::new();
    if iteration_level == 0 {
        return results;
    }

    for (index, _) in words.iter().enumerate() {

        let news = try_one_word_async(goal, &words[index..], iteration_level).await;
        for n in news {
            results.push(n);
        }
    }
    return results;
}

fn anagram<'a>(
    goal: &CharList,
    words: Vec<&'a Box<CharList>>,
    iteration_level: usize,
) -> Vec<Vec<&'a Box<CharList>>> {
    let mut results: Vec<Vec<&Box<CharList>>> = Vec::new();
    if iteration_level == 0 {
        return results;
    }

    for (index, _) in words.iter().enumerate() {

        let news = try_one_word(goal, &words[index..], iteration_level);
        for n in news {
            results.push(n);
        }
    }
    return results;
}
async fn try_one_word_async<'a>(goal: &CharList, candidates: &[&'a Box<CharList>], iteration_level: usize) -> Vec<Vec<&'a Box<CharList>>> {
    
    let mut results: Vec<Vec<&Box<CharList>>> = Vec::new();
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
fn try_one_word<'a>(goal: &CharList, candidates: &[&'a Box<CharList>], iteration_level: usize) -> Vec<Vec<&'a Box<CharList>>> {
    
    let mut results: Vec<Vec<&Box<CharList>>> = Vec::new();
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
    candidates: &[&'a Box<CharList>],
) -> Vec<&'a Box<CharList>> {
    let mut v: Vec<&Box<CharList>> = Vec::new();
    for c in candidates {
        if c.length() <= goal.length() {
            if CharList::filter(goal, &**c) {
                v.push(c);
            }
        }
    }
    // sort longest candidates to the front, this lessens the amount of backtracking
    v.sort_by(|c1, c2| c2.length().cmp(&c1.length()));
    return v;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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
                            if !map.contains_key(&key) {
                                map.insert(key, vec![word]);
                            } else {
                                let candidates = map.get_mut(&key).unwrap();
                                if !candidates.contains(&word) {
                                    candidates.push(word);
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
