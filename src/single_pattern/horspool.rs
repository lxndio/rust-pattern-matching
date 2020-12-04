fn horspool_shift(pattern: &[u8]) -> Vec<usize> {
    let mut shift = vec![pattern.len(); 256];
    let m = pattern.len();

    // Iterate over 0..m - 1
    for (j, c) in pattern.iter().enumerate().take(m - 1) {
        shift[*c as usize] = m - 1 - j;
    }

    shift
}

/// An implementation of the Horspool algorithm.
///
/// Takes a `pattern`, a text `text` and a value `i0` specifying at which index
/// of the text it should start to search for the next occurence.
///
/// If the given text contains the given pattern, the algorithm returns the
/// index `i` of the first letter of the first occurrence with `i >= i0`.
/// If the pattern could not be found in the text, `None` is returned.
///
/// This algorithm terminates after finding one occurrence of the given pattern
/// in the given text. If you want to find all occurrences, consider using
/// [`horspool_all`](horspool_all) instead.
pub fn horspool(pattern: &[u8], text: &[u8], i0: usize) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    let shift = horspool_shift(pattern);
    let mut last = i0 + m - 1;
    let p_last = pattern[m - 1];

    loop {
        while last < n && text[last] != p_last {
            last += shift[text[last] as usize];
        }

        if last >= n {
            break;
        }

        if text[last - (m - 1)..last] == pattern[0..m - 1] {
            return Some(last - m + 1);
        }

        last += shift[p_last as usize];
    }

    None
}

/// A simple algorithm for matching a single pattern on a text returning all occurrences.
///
/// Takes a `pattern`, and text `text`.
///
/// If the given text contains the given pattern, the algorithm returns the
/// indexes of the first letters of all found occurrences.
/// If the pattern could not be found in the text, an empty vector is returned.
pub fn horspool_all(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i0 = 0;

    while let Some(occ) = horspool(pattern, text, i0) {
        res.push(occ);

        i0 = occ + 1; // TODO or `+ m`?
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horspool_all() {
        let text = "gccttaacattattacgccta".as_bytes();
        let pattern = "tta".as_bytes();

        let mut matches = horspool_all(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }
}
