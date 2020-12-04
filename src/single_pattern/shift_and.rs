// Copyright 2020 Alexander Korn
//
// Licensed under the MIT license

pub(crate) fn shift_and_single_masks(pattern: &[u8]) -> (Vec<usize>, usize, usize) {
    let mut masks = vec![0; 256];
    let mut bit = 1;

    for c in pattern {
        masks[*c as usize] |= bit;

        bit *= 2;
    }

    (masks, 1, bit / 2)
}

fn shift_and_with_masks(
    text: &[u8],
    masks: &[usize],
    ones: usize,
    accept: usize,
) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let mut active: usize = 0;

    for (i, c) in text.iter().enumerate() {
        active = ((active << 1) | ones) & masks[*c as usize];

        let found = active & accept;
        if found != 0 {
            res.push((i, found));
        }
    }

    res
}

/// An implementation of the Shift-And algorithm.
///
/// Takes a `pattern`, and text `text`.
///
/// If the given text contains the given pattern, the algorithm returns the
/// indexes of the first letters of all found occurrences.
/// If the pattern could not be found in the text, an empty vector is returned.
///
/// # Runtime
///
/// The worst case runtime is `O(n * m)`, with `n` being the length of the text
/// and `m` being the length of the pattern.
///
/// # When to Use It
///
/// The algorithm can be useful if you want to match short patterns.
///
/// # How It Works
///
/// The algorithm works by simulating a nondeterministic finite automaton (NFA)
/// in memory using cheap bit operations.
pub fn shift_and(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let mut res = Vec::new();
    let m = pattern.len();
    let (mask, ones, accept) = shift_and_single_masks(pattern);

    for (i, _) in shift_and_with_masks(text, &mask, ones, accept) {
        res.push(i - m + 1);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_and() {
        let text = "gccttaacattattacgccta".as_bytes();
        let pattern = "tta".as_bytes();

        let mut matches = shift_and(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }
}
