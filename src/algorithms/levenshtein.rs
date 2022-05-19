use std::cmp::{max, min};

/// Main levenshtein distance computations
///
/// This function implements calculation of the [levenshtein
/// distance](https://en.wikipedia.org/wiki/Levenshtein_distance) between two
/// strings, with specified costs for insertion, deletion, and substitution, and
/// a limit. The other functions in this module simply wrap it, and it's
/// generally easier to use any of those (e.g. [`levenshtein_limit`]) unless you
/// need all the functionality that this has to offer.
///
/// This is an implementation of the iterative algorithm on the Wikipedia page.
pub fn levenshtein_limit_weight(
    a: &str,
    b: &str,
    limit: u32,
    ins_cost: u32,
    del_cost: u32,
    sub_cost: u32,
) -> u32 {
    let a_len = a.len() as u32;
    let b_len = b.len() as u32;

    let diff = max(a_len, b_len) - min(a_len, b_len);

    if diff >= limit {
        return limit;
    }

    // Create two working vectors
    let v_len = a.len() + 1;
    let mut v_prev: Vec<u32> = (0..(v_len as u32)).collect();
    let mut v_curr: Vec<u32> = vec![0; v_len];

    let mut deletion_cost: u32;
    let mut insertion_cost: u32;
    let mut substitution_cost: u32;

    for (i, b_char) in b.chars().enumerate() {
        v_curr[0] = (i + 1) as u32;

        // Fill out the rest of the row
        for (j, a_char) in a.chars().enumerate() {
            // calculating costs for A[i+1][j+1]
            deletion_cost = (v_prev[j + 1] + 1) * del_cost;
            insertion_cost = (v_curr[j] + 1) * ins_cost;
            substitution_cost = (match a_char == b_char {
                true => v_prev[j],
                false => v_prev[j] + 1,
            }) * sub_cost;

            v_curr[j + 1] = min(min(deletion_cost, insertion_cost), substitution_cost);
        }
        let current_max = v_curr.last().copied().unwrap_or_default();
        if current_max >= limit {
            return limit;
        }

        // Move current row to previous for the next loop
        // "Current" is always overwritten so we can just swap
        std::mem::swap(&mut v_prev, &mut v_curr);
    }

    // Remember we swapped
    v_prev.last().copied().unwrap_or_default()
}

/// Levenshtein distance computation with weights
///
/// See [`levenshtein_limit_weight`] for details; this function is simply a
/// wrapper to remove limit function.
///
/// # Example
///
/// ```
/// use stringmetrics::algorithms::levenshtein_limit;
/// let a = "abcdefg";
/// let b = "mmmmmmm";
/// assert_eq!(levenshtein_limit(a, b, 3), 3);
/// ```
#[inline]
pub fn levenshtein_weight(a: &str, b: &str, ins_cost: u32, del_cost: u32, sub_cost: u32) -> u32 {
    levenshtein_limit_weight(a, b, u32::MAX, ins_cost, del_cost, sub_cost)
}

/// Levenshtein distance computation with a limit
///
/// This will limitate the levshtein distance up to a given maximum value. The
/// usual reason for wanting to do this is to avoid unnecessary computation when
/// a match between two strings can quickly be pruned as "different".
///
/// Behind the scenes, this wraps [`levenshtein_limit_weight`].
///
/// # Example
///
/// ```
/// use stringmetrics::algorithms::levenshtein_limit;
/// let a = "abcdefg";
/// let b = "mmmmmmm";
/// assert_eq!(levenshtein_limit(a, b, 3), 3);
/// ```
#[inline]
pub fn levenshtein_limit(a: &str, b: &str, limit: u32) -> u32 {
    levenshtein_limit_weight(a, b, limit, 1, 1, 1)
}

/// Basic Levenshtein distance computation
///
/// This runs the levenshtein distance algorithm on all strings with all costs
/// equal to 1 and no limits, which is suitable for cases where an exact
/// distance is needed, mainly those where the strings are known to not be "very
/// different" (e.g., strings of different lengths). In many cases it is better
/// to use [`levenshtein_limit`] to avoid unnecessary computation.
///
/// Behind the scenes, this wraps [`levenshtein_limit_weight`].
///
/// # Example
///
/// ```
/// use stringmetrics::algorithms::levenshtein;
/// let a = "this is a book";
/// let b = "i am a cook";
/// assert_eq!(levenshtein(a, b), 6);
/// ```
#[inline]
pub fn levenshtein(a: &str, b: &str) -> u32 {
    levenshtein_limit_weight(a, b, u32::MAX, 1, 1, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein("", ""), 0);
    }

    #[test]
    fn test_levenshtein_equal() {
        assert_eq!(levenshtein("abcdef", "abcdef"), 0);
    }

    #[test]
    fn test_levenshtein_one_empty() {
        assert_eq!(levenshtein("abcdef", ""), 6);
        assert_eq!(levenshtein("", "abcdef"), 6);
    }

    #[test]
    fn test_levenshtein_basic() {
        assert_eq!(levenshtein("abcd", "ab"), 2);
        assert_eq!(levenshtein("abcd", "ad"), 2);
        assert_eq!(levenshtein("abcd", "cd"), 2);
        assert_eq!(levenshtein("abcd", "a"), 3);
        assert_eq!(levenshtein("abcd", "c"), 3);
        assert_eq!(levenshtein("to be a bee", "not to bee"), 6);
    }

    #[test]
    fn test_levenshtein_limit_one_empty() {
        assert_eq!(levenshtein_limit("abcdef", "", 3), 3);
        assert_eq!(levenshtein_limit("", "abcdef", 3), 3);
        assert_eq!(levenshtein_limit("abcdef", "", 8), 6);
        assert_eq!(levenshtein_limit("", "abcdef", 8), 6);
    }

    #[test]
    fn test_levenshtein_limit() {
        // Most of this is tested via levenshtein()
        // just need to validate limits
        assert_eq!(levenshtein_limit("abcdef", "000000", 3), 3);
        assert_eq!(levenshtein_limit("ab", "cccc", 3), 3);
    }

    #[test]
    fn test_levenshtein_weight_insertion() {
        assert_eq!(levenshtein_weight("000", "000a", 10, 2, 10), 2);
    }

    #[test]
    fn test_levenshtein_weight_deletion() {
        assert_eq!(levenshtein_weight("000a", "000", 2, 10, 10), 2);
    }


    // #[test]
    // fn test_levenshtein_weight_substitution() {
    //     assert_eq!(levenshtein_weight("kitten", "sitten", 10, 10, 2), 2);
    // }
}
