// Copyright 2020 Alexander Korn
//
// Licensed under the MIT license

/// A simple algorithm for matching a single pattern on a text returning the first occurrence.
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
/// [`naive_all`](naive_all) instead.
///
/// # Runtime
///
/// The worst case runtime is `O(n * m)`, with `n` being the length of the text
/// (perhaps starting at `i0`) and `m` being the length of the pattern.
///
/// # When to Use It
///
/// Probably never. This algorithm is the most simple approach to matching a
/// pattern on a text. Could be useful for comparing runtimes or correctness
/// with other algorithms.
///
/// # How It Works
///
/// The algorithm iterates over each index `i` of the text's characters starting
/// at `i0` and compares the following `m` characters starting at `i` with the
/// pattern, `m` being the length of the pattern.
///
/// After an occurrence has been found, the algorithm returns the index
/// marking the first character of the occurrence and therefore terminates.
pub fn naive(pattern: &[u8], text: &[u8], i0: usize) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    for i in i0..(n - m + 1) {
        if &text[i..i + m] == pattern {
            return Some(i);
        }
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
///
/// # When to Use It
///
/// Probably never. This algorithm is the most simple approach to matching a
/// pattern on a text. Could be useful for comparing runtimes or correctness
/// with other algorithms.
///
/// # How It Works
///
/// The algorithm calls the [`naive`](naive) algorithm starting with a the `i0`
/// parameter being `0`. After the `naive` algorithm returns, it calls that same
/// algorithm again with an `i0` parameter of the position in text right after
/// the found occurrence.
pub fn naive_all(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i0 = 0;

    while let Some(occ) = naive(pattern, text, i0) {
        res.push(occ);

        i0 = occ + 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_all() {
        let text = "gccttaacattattacgccta".as_bytes();
        let pattern = "tta".as_bytes();

        let mut matches = naive_all(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }
}
