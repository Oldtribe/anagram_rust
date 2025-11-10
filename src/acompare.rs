/// compare two strings, ignoring spaces
/// result: how many transpositions are needed at minimum
/// to turn one string into the other

use substring::Substring;

#[derive(PartialEq, Eq)]
pub struct Transposition {
    start: usize,
    destination: usize,
    span: usize
}

pub fn get_transpositions(s1: String, s2: String) -> Vec<Transposition> {
    // strip spaces from the two strings
    let mut s1 = s1;
    let mut s2 = s2;
    s1.retain(|c| c != ' ');
    s2.retain(|c| c != ' ');

    let mut transpositions = Vec::new();

    // do for all substrings of s1
    let l1 = s1.chars().count();
    for start in 0..l1 {
        for end in start..l1 {
            let span = end - start;
            for destination in 0..l1 - span {
                if s1.substring(start, end + 1) == s2.substring(destination, destination + span + 1) {
                    let t = Transposition {start, destination, span: span + 1};
                    transpositions.push(t);
                } 
            }
        }
    }
    transpositions
}

pub fn covers(t1: &Transposition, t2: &Transposition) -> bool {
    !((t1.start + t1.span <= t2.start ||
    t2.start + t2.span <= t1.start) &&
    (t1.destination + t1.span <= t2.destination ||
    t2.destination + t2.span <= t1.destination))
}

pub fn maximum_overlap<'a>(ts: &Vec<&'a Transposition>) -> Vec<&'a Transposition>  {
    // figure out the one transposition with the most overlaps
    let mut max_count = -1;
    let mut max_t: &Transposition = &ts[0];
    for t in ts {
        let count = ts.iter().fold(0, |acc, t2|if covers(t2, t) {acc + 1} else {acc});
        if count > max_count {
            max_count = count;
            max_t = t;
        }
    }
    let out = ts.iter().cloned().filter(|&t| !covers(max_t, t)).collect::<Vec<_>>();
    out
}

pub fn greedy_score(v: &Vec<&Transposition>) -> usize {
    let mut count = 1;
    let mut next = v;
    let mut news;
    loop {
        news = maximum_overlap(next);
        if news.len() == 0 {
            break;
        }
        count = count + 1;
        next = &news
    }
    count
}
#[cfg(test)]
mod tests {
    use super::Transposition;

    #[test]
    fn basic_transposition() {
        let ts = super::get_transpositions("abc".to_string(), "bca".to_string());
        assert!(ts[0] == Transposition{start:0, destination:2, span:1});
        assert!(ts[1] == Transposition{start:1, destination:0, span:1});
        assert!(ts[2] == Transposition{start:1, destination:0, span:2});
        assert!(ts[3] == Transposition{start:2, destination:1, span:1});
    }
    #[test]
    fn covers() {
        let ts = super::get_transpositions("abc".to_string(), "bca".to_string());
        assert!(!super::covers(&ts[0],&ts[1]));
        assert!(!super::covers(&ts[0],&ts[2]));
        assert!(!super::covers(&ts[0],&ts[3]));
        assert!(super::covers(&ts[1],&ts[2]));
        assert!(!super::covers(&ts[1],&ts[3]));
        assert!(super::covers(&ts[2],&ts[3]));
        assert!(!super::covers(&ts[1],&ts[0]));
        assert!(!super::covers(&ts[2],&ts[0]));
        assert!(!super::covers(&ts[3],&ts[0]));
        assert!(super::covers(&ts[2],&ts[1]));
        assert!(!super::covers(&ts[3],&ts[1]));
        assert!(super::covers(&ts[3],&ts[2]));
    }
    #[test]
    fn maximum_overlap() {
        let ts = super::get_transpositions("abcdabda".to_string(), "dabdacba".to_string());
        let pts = ts.iter().map(|t| t).collect::<Vec<_>>();
        let v = super::maximum_overlap(&pts);
        assert!(v.len() == 3);
    }
    #[test]
    fn greedy() {
        let ts = super::get_transpositions("12345678".to_string(), "56783421".to_string());
        let pts = ts.iter().map(|t| t).collect::<Vec<_>>();
        let c = super::greedy_score(&pts);
        assert!(c == 4);
    }
}

