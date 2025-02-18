// Chapter 1: Where in the Genome Does Replication Begin?

pub fn pattern_count(text: &str, pattern: &str) -> i32 {
    // Count the number of times `pattern` occurs as a substring of `text`
    let mut count: i32 = 0;
    let k = pattern.len();

    for i in 0..=(text.len()-k) {
        if text[i..(i+k)] == *pattern {
            count += 1;
        }
    }

    return count;
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_count_case1() {
        let text = "ACGTACGTACGT";
        let pattern = "CG";
        assert_eq!(pattern_count(text, pattern), 3);
    }

    #[test]
    fn pattern_count_case2() {
        let text = "ATGCGCGTA";
        let pattern = "GCG";
        assert_eq!(pattern_count(text, pattern), 2);
    }

    #[test]
    fn pattern_count_case3() {
        let text = "AAAGAGTGTCTGA";
        let pattern = "AAA";
        assert_eq!(pattern_count(text, pattern), 1);
    }

    #[test]
    fn pattern_count_case4() {
        let text = "AGCGTGCCGAAATTT";
        let pattern = "TTT";
        assert_eq!(pattern_count(text, pattern), 1);
    }

    #[test]
    fn pattern_count_case5() {
        let text = "GGACTTACTGACGTACG";
        let pattern = "ACT";
        assert_eq!(pattern_count(text, pattern), 2);
    }

    #[test]
    fn pattern_count_case6() {
        let text = "ATCCGATCCCATGCCCATG";
        let pattern = "CC";
        assert_eq!(pattern_count(text, pattern), 5);
    }
}
