use crate::charcount::CharCount;
use std::fmt;

/// CharList stores a list of letters and their counts.
/// The items in the list are guaranteed to be in order.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct CharList {
    length: usize,
    list: Vec<CharCount>,
}

impl CharList {
    /// create a new CharList
    pub fn new() -> CharList {
        CharList {
            length: 0,
            list: Vec::new(),
        }
    }
    /// initialize with an existing charCount
    pub fn init(count: CharCount) -> CharList {
        let mut l = Vec::new();
        let length = count.count;
        l.push(count);
        CharList {
            length: length,
            list: l,
        }
    }
    /// combine two CharLists into one new list.
    pub fn combine(first: CharList, second: CharList) -> CharList {
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
                            return CharList {
                                length: first.length + second.length,
                                list: result,
                            };
                        }
                        Some(cc2) => {
                            // first is done, so just add this
                            result.push(CharCount {
                                letter: cc2.letter,
                                count: cc2.count,
                            });
                            item2 = iter2.next();
                        }
                    }
                }
                Some(cc1) => {
                    match item2 {
                        None => {
                            result.push(CharCount {
                                letter: cc1.letter,
                                count: cc1.count,
                            });
                            item1 = iter1.next();
                        }
                        Some(cc2) => {
                            // figure out which one is lower, and push just that
                            if cc1.letter < cc2.letter {
                                result.push(CharCount {
                                    letter: cc1.letter,
                                    count: cc1.count,
                                });
                                item1 = iter1.next();
                            } else if cc1.letter > cc2.letter {
                                result.push(CharCount {
                                    letter: cc2.letter,
                                    count: cc2.count,
                                });
                                item2 = iter2.next();
                            } else {
                                result.push(CharCount {
                                    letter: cc1.letter,
                                    count: cc1.count + cc2.count,
                                });
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
    pub fn from_string(s: &str) -> CharList {
        let mut acc = CharList::new();
        for c in s.chars() {
            let cl = CharList::init(CharCount::new(c));
            acc = CharList::combine(acc, cl);
        }
        acc
    }

    /// subtract two CharLists
    pub fn subtract(big: &CharList, small: &CharList) -> MatchResult {
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
                                return MatchResult::PartialMatch(CharList {
                                    length: big.length - small.length,
                                    list: result,
                                });
                            }
                        }
                        Some(_) => return MatchResult::NoMatch,
                    }
                }
                Some(cc1) => {
                    match smallc {
                        None => {
                            result.push(CharCount {
                                letter: cc1.letter,
                                count: cc1.count,
                            });
                            bigc = bigiter.next();
                        }
                        Some(cc2) => {
                            // figure out which one is lower, and push just that
                            if cc1.letter < cc2.letter {
                                result.push(CharCount {
                                    letter: cc1.letter,
                                    count: cc1.count,
                                });
                                bigc = bigiter.next();
                            } else if cc1.letter > cc2.letter {
                                return MatchResult::NoMatch;
                            } else {
                                if cc1.count < cc2.count {
                                    return MatchResult::NoMatch;
                                } else if cc1.count > cc2.count {
                                    result.push(CharCount {
                                        letter: cc1.letter,
                                        count: cc1.count - cc2.count,
                                    });
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
    pub fn filter(big: &CharList, small: &CharList) -> bool {
        let mut bigiter = big.list.iter();
        let mut smalliter = small.list.iter();
        let mut bigc = bigiter.next();
        let mut smallc = smalliter.next();
        loop {
            match bigc {
                None => match smallc {
                    None => {
                        return true;
                    }
                    Some(_) => {
                        return false;
                    }
                },
                Some(cc1) => {
                    match smallc {
                        None => {
                            bigc = bigiter.next();
                        }
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

    pub fn length(&self) -> usize {
        self.length
    }
}
impl fmt::Display for CharList {
    /// Formatter for CharList
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.list)
    }
}

#[cfg(test)]
mod tests {

    use super::CharCount;
    use super::CharList;
    use super::MatchResult;

    #[test]
    fn combine_same() {
        let l1: CharList = CharList::init(CharCount {
            letter: 'a',
            count: 2,
        });
        let l2: CharList = CharList::init(CharCount {
            letter: 'a',
            count: 3,
        });
        let l3: CharList = CharList::combine(l1, l2);
        assert!(l3.list.len() == 1);
        assert!(l3.list.get(0).unwrap().letter == 'a');
        assert!(l3.list.get(0).unwrap().count == 5);
    }

    #[test]
    fn combine_different() {
        let l1: CharList = CharList::init(CharCount {
            letter: 'b',
            count: 3,
        });
        let l2: CharList = CharList::init(CharCount {
            letter: 'a',
            count: 2,
        });
        let l3: CharList = CharList::combine(l1, l2);
        assert!(l3.list.len() == 2);
        assert!(l3.list.get(0).unwrap().letter == 'a');
        assert!(l3.list.get(0).unwrap().count == 2);
        assert!(l3.list.get(1).unwrap().letter == 'b');
        assert!(l3.list.get(1).unwrap().count == 3);
    }

    #[test]
    fn combine_into_middle() {
        let l1: CharList = CharList::init(CharCount {
            letter: 'c',
            count: 3,
        });
        let l2: CharList = CharList::init(CharCount {
            letter: 'a',
            count: 2,
        });
        let l3: CharList = CharList::combine(l1, l2);
        let l4: CharList = CharList::init(CharCount {
            letter: 'b',
            count: 1,
        });
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
}

/// Enum MatchResult holds the result of subtracting one charlist from another
#[derive(PartialEq, Eq, Debug)]
pub enum MatchResult {
    NoMatch,
    FullMatch,
    PartialMatch(CharList),
}
