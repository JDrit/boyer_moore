
extern crate ansi_term;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::mem;
use self::ansi_term::Colour;

use search::tree;

const ALPHABET_SIZE: usize = 256;

macro_rules! init_array(
    ($ty:ty, $len:expr, $val:expr) => (
        {
            let mut array: [$ty; $len] = unsafe { mem::uninitialized() };
            for i in array.iter_mut() {
                unsafe { ::std::ptr::write(i, $val); }
            }
            array
        }
    )
);

macro_rules! max(
    ($e1: expr, $e2: expr) => (
        {
            if $e1 > $e2 {
                $e1
            } else {
                $e2
            }
        }
    )
);

fn match_length(pattern: &Vec<char>, mut i1: usize, mut i2: usize) -> usize {
    if i1 == i2 {
        return pattern.len() - i1;
    } else {
        let mut count: usize = 0;
        while i1 < pattern.len() && i2 < pattern.len() && pattern[i1] == pattern[i2] {
            count += 1;
            i1 += 1;
            i2 += 1;            
        }
        return count;
    }
}

fn preprocess(pattern: &str) -> Vec<usize> {
    if pattern.len() == 0 {
        return vec![];
    } else if pattern.len() == 1 {
        return vec![1];
    } else {
        let chars: Vec<char> = pattern.chars().collect();
        let mut result = vec![0; pattern.len()];
        result[0] = pattern.len();
        result[1] = match_length(&chars, 0, 1);

        for i in 2..(1 + result[1]) {
            result[i] = result[1] - i + 1;
        }

        let mut left = 0;
        let mut right = 0;

        for i in (2 + result[1])..(pattern.len()) {
            if i <= right {
                let k = i - left;
                let b = result[k];
                let a = right - i + 1;
                if b < a {
                    result[i] = b;
                } else {
                    result[i] = match_length(&chars, a, right + 1);
                    left = i;
                    right = i + result[i] - 1;
                }
            } else {
                result[i] = match_length(&chars, 0, i);
                if result[i] > 0 {
                    left = i;
                    right = i + result[i] - 1;
                }
            }
        }
        return result;
    }
}

fn get_good_suffix(pattern: &str) -> Vec<i32> {
    let p: String = pattern.to_string().chars().rev().collect();
    let p_str: &str = p.as_str();
    
    let mut result: Vec<i32> = vec![-1; pattern.len()];
    let mut preprocess = preprocess(p_str);
    preprocess.reverse();

    for i in 0..(pattern.len() - 1) {
        let j = pattern.len() - preprocess[i];
        if j != preprocess.len() {
            result[j] = i as i32;
        }
    }

    return result;
}

fn get_full_shift(pattern: &str) -> Vec<usize> {
    let mut result = vec![0 ; pattern.len()];
    let mut z = preprocess(pattern);
    z.reverse();
    let mut longest: usize = 0;

    for (index, value) in z.iter().enumerate() {
        if *value == index + 1 {
            longest = max!(longest, *value);
        }
        let i = result.len() - index - 1;
        result[i] = longest; 
    }

    return result;    
}

fn get_bad_character(pattern: &str) -> [Vec<i32>; ALPHABET_SIZE] {
    let mut result: [Vec<i32>; ALPHABET_SIZE] =
        init_array!(Vec<i32>, ALPHABET_SIZE, Vec::with_capacity(pattern.len()));

    for v in result.iter_mut() {
        v.push(-1);
    }

    let mut alpha: [i32; ALPHABET_SIZE] = [-1 ; ALPHABET_SIZE];
    for (i, c) in pattern.chars().enumerate() {
        alpha[c as usize] = i as i32;
        for (j, a) in alpha.iter().enumerate() {
            result[j].push(*a);
        }
    }
    
    return result;
}

/// Returns the contents of the file
fn load_file<'a>(input: File) -> String {
    let mut buf_reader = BufReader::new(input);
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    return contents;
}


///
/// Searches the file for the given pattern and returns the list
/// of places that it occurs in.
/// # Arguments
/// * `pattern` - the string to search for
/// * `input` - the file to check
pub fn search_file(pattern: &str, input: File) {
    let file_contents = load_file(input);
    let chars = file_contents.chars().collect();
    let results = search_string(pattern, file_contents);
    
    for result in results {
        print_result(result, pattern, 1, &chars);
    }
}

///
/// Pretty prints the match found.
/// # Arguments
/// * `result` - the beggining offset that the pattern matched at
/// * `pattern` - the actual pattern that matched
/// * `line` - which line the result was found on
/// * `chars` - the search contents
///
fn print_result(result: usize, pattern: &str, line: i32, chars: &Vec<char>) {
    let mut min_offset = result;
    let mut max_offset = result;

    while min_offset != 0 && chars[min_offset] != '\n' {
        min_offset -= 1;
    }
    
    while max_offset != chars.len() - 1 && chars[max_offset] != '\n' {
        max_offset += 1;
    }
    
    let prefix: String = chars[(min_offset + 1)..result].into_iter()
        .cloned().collect();
    let suffix: String = chars[(result + pattern.len())..max_offset].into_iter()
        .cloned().collect();

    println!("{}: {}{}{}", line, prefix, Colour::Green.paint(pattern), suffix);
}

///
/// Finds and prints all the occurences of the `pattern` in the `contents`
/// search string.
/// # Arguments
/// * `pattern` the string to search for
/// * `contents` the body to search within
///
/// # Result
/// The list of offsets that pattern was found at
///
/// ```
/// use boyer_moore::search::search;
///
/// let results: Vec<usize> = search::search_string("test", "search for test".to_string());
/// assert_eq!(1, results.len());
/// ```
///
///
pub fn search_string(pattern: &str, contents: String) -> Vec<usize> {
    let chars = contents.chars().collect();
    return search(pattern, &chars);
}

///
/// Finds the occurences of the pattern in the search area. Returns the
/// starting index of every occurence.
///
/// # Arguments
/// * `pattern` - the string to try and search for
/// * `contents` - the body to search in
///
/// # Result
/// The list of offsets that the pattern was found at
///
fn search(pattern: &str, contents: &Vec<char>) -> Vec<usize> {
    let mut results = Vec::new();
    let p_vec: Vec<char> = pattern.chars().collect();

    if contents.len() == 0 || pattern.len() > contents.len() {
        return results;
    }
    let bad_char_table = get_bad_character(pattern);
    let good_suffix = get_good_suffix(pattern);
    let full_shift = get_full_shift(pattern);

    // alignment of the end of the pattern relative to the search
    let mut k: usize = pattern.len() - 1; 
    let mut prev_k: i32 = -1;

    while k < contents.len() {
        let mut p_index: usize = pattern.len() - 1; // index to search in the pattern
        let mut c_index: usize = k;                 // index to search in content
        let mut valid = false;

        while p_vec[p_index] == contents[c_index] {

            if p_index == 0 || c_index as i32 == prev_k + 1 {
                valid = true;
                break;
            } else {
                p_index -= 1;
                c_index -= 1;
            }
        }

        if valid { // match found
            let i = k + 1 - pattern.len();
            results.push(i);
            k += 1;
        } else { // no match, calculate shift distance
            let bad_char = bad_char_table[contents[c_index] as usize][p_index];
            let char_shift = p_index as i32 - bad_char;

            let suffix_shift;
            if p_index + 1 == pattern.len() {
                suffix_shift = 1;
            } else if good_suffix[p_index + 1] == -1 {
                // matched suffix does not appear anywhere in the input pattern
                suffix_shift = (pattern.len() - full_shift[p_index + 1]) as i32;
            } else {
                // matched suffix does appear in the input pattern
                suffix_shift = pattern.len() as i32 - good_suffix[p_index + 1] - 1;

            }
            let shift = max!(char_shift, suffix_shift);
            if shift >= p_index as i32 + 1 {
                prev_k = k as i32;
            }
            k += shift as usize;           
        }
    }
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_search() {
        let results = search_string("pattern", "a pattern to find".to_string());
        assert_eq!(1, results.len(), "only one result should be returned");
        assert_eq!(2, results[0], "correct index");
    }

    #[test]
    fn pattern_at_end() {
        let results = search_string("test", "find test".to_string());
        assert_eq!(1, results.len());
        assert_eq!(5, results[0]);
    }

    #[test]
    fn another_test() {
        let results: Vec<usize> = search_string("test", "search for test".to_string());
        assert_eq!(1, results.len());
    }

    #[test]
    fn no_results() {
        let results = search_string("jfkdl", "a pattern to find".to_string());
        assert_eq!(0, results.len(), "no results found");
    }
    
    #[test]
    fn single_character_pattern() {
        let results = search_string("p", "abcdefghijklmnopqrstuvwxyz".to_string());
        assert_eq!(1, results.len(), "only one result");
        assert_eq!(15, results[0], "correct index");
    }

    #[test]
    fn two_character_pattern_at_beginning() {
        let results = search_string("ab", "abcdefghijklmnopqrstuvwxyz".to_string());
        assert_eq!(1, results.len(), "only one result");
        assert_eq!(0, results[0], "correct index");
    }

    #[test]
    fn two_character_pattern_at_end() {
        let results = search_string("yz", "abcdefghijklmnopqrstuvwxyz".to_string());
        assert_eq!(1, results.len(), "only one result");
        assert_eq!(24, results[0], "correct index");
    }

    #[test]
    fn two_character_pattern() {
        let results = search_string("mn", "abcdefghijklmnopqrstuvwxyz".to_string());
        assert_eq!(1, results.len(), "only one result");
        assert_eq!(12, results[0], "correct index");
    }
    
    #[test]
    fn two_character_repeat() {
        let results = search_string("ab", "abababababab".to_string());
        assert_eq!(6, results.len(), "correct number of results");
    }

    #[test]
    fn multiple_results() {
        let input = "search jdd in the string jdd of jdd".to_string();
        let results = search_string("jdd", input);
        assert_eq!(3, results.len(), "three results");
    }

    #[test]
    fn all_matches() {
        let results = search_string("j", "jjjjjjjjjj".to_string());
        assert_eq!(10, results.len(), "all matches");
    }

    #[test]
    fn match_length_equal() {
        let chars = "this is a test of this".chars().collect();
        let result = super::match_length(&chars, 0, 0);
        assert_eq!(chars.len(), result, "match should be entire string");
    }

    #[test]
    fn match_length_different() {
        let chars = "this is a this of this".chars().collect();
        let result = super::match_length(&chars, 0, 10);
        assert_eq!(5, result, "match all of `this `");
    }

    #[test]
    fn match_length_no_match() {
        let chars = "this is a -es- of no-hing".chars().collect();
        let result = super::match_length(&chars, 0, 10);
        assert_eq!(0, result, "no match");
    }

    #[test]
    fn small_preprocess() {
        let result = super::preprocess("a");
        assert_eq!(1, result.len(), "correct size");
        assert_eq!(1, result[0], "correct substring size");
    }

    #[test]
    fn empty_preprocess() {
        assert_eq!(0, super::preprocess("").len(), "empty result");
    }

    #[test]
    fn simple_preprocess() {
        let input = "hanhan";
        let result = super::preprocess(input);        
        assert_eq!(input.len(), result[0]);
        assert_eq!(0, result[1]);
        assert_eq!(3, result[3]);
    }

    #[test]
    fn full_shift_simple() {
        let input = "hanhan";
        let result = super::get_full_shift(input);

        assert_eq!(input.len(), result.len());
        assert_eq!(3, result[3]);
        assert_eq!(3, result[2]);
    }

    #[test]
    fn full_shift_no_result() {
        let input = "abcdefghijklm";
        let result = super::get_full_shift(input);
        assert_eq!(13, result[0], "the full body is a prefix and suffix");
        
        for i in 1..13 {
            assert_eq!(0, result[i], "should be no results for {}", i);
        }
    }
}
