//! Extension trait for controlled trimming of prefixes and suffixes of `&str` and `String`.
//!
//! Provided methods trim only if the given pattern matches exact number of times, otherwise
//! they return the unmodified `&str`. This can be used for primitive parsing and text analysis.

#![feature(pattern)]
#![no_std]

use core::str::pattern::{Pattern, ReverseSearcher, Searcher, SearchStep};

/// The extension trait adding methods for controlled trimming
pub trait TrimMatchesExactlyExt {
    /// Returns '&str' with pattern matches trimmed from its beginning given number of times.
    /// If pattern can't be trimmed off that many times, returns `Err` with an untrimmed `&str`.
    /// This can be used for primitive parsing and text analysis.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. 'Right' in this context means the last
    /// position of that byte string; for a language like Arabic or Hebrew
    /// which are 'right to left' rather than 'left to right', this will be
    /// the _left_ side, not the right.
    ///
    /// # Examples
    /// ```
    /// # use trim_matches_exactly::TrimMatchesExactlyExt;
    /// assert_eq!(Ok("trimmed"), "not trimmed".trim_left_matches_exactly("not ", 1));
    /// assert_eq!(Err("not trimmed"), "not trimmed".trim_left_matches_exactly("very ", 1));
    /// assert_eq!(Ok("trimmed"), "tttrimmed".trim_left_matches_exactly('t', 2));
    /// ```
    fn trim_left_matches_exactly<'a, P: Pattern<'a>>(&'a self, pat: P, count: usize)
        -> Result<&'a str, &'a str>;
    /// Returns '&str' with pattern matches trimmed from its end given number of times.
    /// If pattern can't be trimmed off that many times, returns `Err` with an untrimmed `&str`.
    /// This can be used for primitive parsing and text analysis.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. 'Right' in this context means the last
    /// position of that byte string; for a language like Arabic or Hebrew
    /// which are 'right to left' rather than 'left to right', this will be
    /// the _left_ side, not the right.
    ///
    /// # Examples
    /// ```
    /// # use trim_matches_exactly::TrimMatchesExactlyExt;
    /// assert_eq!(Ok("trim"), "trim me!".trim_right_matches_exactly(" me!", 1));
    /// assert_eq!(Err("trim me!"), "trim me!".trim_right_matches_exactly(" you!", 1));
    /// assert_eq!(Ok("trim"), "trimmm".trim_right_matches_exactly('m', 2));
    /// ```
    fn trim_right_matches_exactly<'a, P: Pattern<'a>>(&'a self, pat: P, count: usize)
        -> Result<&'a str, &'a str>
        where P::Searcher: ReverseSearcher<'a>;
}

impl TrimMatchesExactlyExt for str {
    fn trim_left_matches_exactly<'a, P: Pattern<'a>>(&'a self, pat: P, count: usize)
            -> Result<&'a str, &'a str> {
        let mut matcher = pat.into_searcher(self);
        unsafe {
            let mut trim_idx = 0;
            for _ in 0..count {
                match matcher.next() {
                    SearchStep::Match(_, match_end) => trim_idx = match_end,
                    _ => return Err(self),
                }
            }
            Ok(self.slice_unchecked(trim_idx, self.len()))
        }
    }

    fn trim_right_matches_exactly<'a, P: Pattern<'a>>(&'a self, pat: P, count: usize)
            -> Result<&'a str, &'a str>
            where P::Searcher: ReverseSearcher<'a> {
        let mut matcher = pat.into_searcher(self);
        unsafe {
            let mut trim_idx = self.len();
            for _ in 0..count {
                match matcher.next_back() {
                    SearchStep::Match(match_start, _) => trim_idx = match_start,
                    _ => return Err(self),
                }
            }
            Ok(self.slice_unchecked(0, trim_idx))
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    mod trim_left_matches_exactly {
        use super::*;

        fn assert_trim(expected: Result<&str, &str>, haystack: &str, needle: &str, count: usize) {
            let actual = haystack.trim_left_matches_exactly(needle, count);

            assert_eq!(expected, actual,
                "For haystack '{}', needle '{}' and count '{}'", haystack, needle, count);
        }

        #[test]
        fn returns_trimmed_or_original_str() {
            assert_trim(Ok("aab"),  "aab", "",  0);
            assert_trim(Ok("aab"),  "aab", "",  1);
            assert_trim(Err("aab"), "aab", "",  2);
            assert_trim(Ok("aab"),  "aab", "a", 0);
            assert_trim(Ok("ab"),   "aab", "a", 1);
            assert_trim(Ok("b"),    "aab", "a", 2);
            assert_trim(Err("aab"), "aab", "a", 3);
            assert_trim(Ok("aab"),  "aab", "b", 0);
            assert_trim(Err("aab"), "aab", "b", 1);
        }
    }

    mod trim_right_matches_exactly {
        use super::*;

        fn assert_trim(expected: Result<&str, &str>, haystack: &str, needle: &str, count: usize) {
            let actual = haystack.trim_right_matches_exactly(needle, count);

            assert_eq!(expected, actual,
                "For haystack '{}', needle '{}' and count '{}'", haystack, needle, count);
        }

        #[test]
        fn returns_trimmed_or_original_str() {
            assert_trim(Ok("baa"),  "baa", "",  0);
            assert_trim(Ok("baa"),  "baa", "",  1);
            assert_trim(Err("baa"), "baa", "",  2);
            assert_trim(Ok("baa"),  "baa", "a", 0);
            assert_trim(Ok("ba"),   "baa", "a", 1);
            assert_trim(Ok("b"),    "baa", "a", 2);
            assert_trim(Err("baa"), "baa", "a", 3);
            assert_trim(Ok("baa"),  "baa", "b", 0);
            assert_trim(Err("baa"), "baa", "b", 1);
        }
    }
}
