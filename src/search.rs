
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::mem;

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

/*
 * Returns the contents of the file
 */
fn load_file<'a>(input: File) -> String {
    let mut buf_reader = BufReader::new(input);
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    return contents;
}

/*
 * Searches the file for the given pattern and returns the list
 * of places that it occurs in.
 */
pub fn search_file(pattern: &str, input: File) -> Vec<usize> {
    return search_string(pattern, load_file(input));    
}

pub fn search_string(pattern: &str, contents: String) -> Vec<usize> {
    let results = search(pattern, contents.chars().collect());
    return results;
}

fn search(pattern: &str, contents: Vec<char>) -> Vec<usize> {
    let mut results = Vec::new();
    let p_vec: Vec<char> = pattern.chars().collect();

    if contents.len() == 0 || pattern.len() > contents.len() {
        return results;
    }
    let bad_char_table = get_bad_character(pattern);

    // alignment of the end of the pattern relative to the search
    let mut k: usize = pattern.len() - 1; 
    let mut prev_k: i32 = -1;

    while k < contents.len() {
        let mut p_index = pattern.len() - 1; // index to search in the pattern
        let mut c_index = k;                 // index to search in content
        let mut valid = false;

        while c_index as i32 > prev_k && p_vec[p_index] == contents[c_index] {
            if p_index == 0 || c_index as i32 == prev_k {
                valid = true;
                break;
            }
            p_index -= 1;
            c_index -= 1;                
        }

        if valid { // match found
            let i = k + 1 - pattern.len();
            println!("found at: {}", i);
            results.push(i);
            k += 1;
        } else { // no match
            let bad_char = bad_char_table[contents[c_index] as usize][p_index];
            let char_shift;
            if bad_char == -1 {
                char_shift = p_index + 1;
            } else {
                char_shift = p_index - bad_char as usize;
            }

            if char_shift >= p_index + 1 {
                prev_k = k as i32;
            }
            k+= char_shift;
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
        let results = search_string("jd ", "search jd in the string jd of jd".to_string());
        assert_eq!(2, results.len(), "three results");
    }

    #[test]
    fn all_matches() {
        let results = search_string("j", "jjjjjjjjjj".to_string());
        assert_eq!(10, results.len(), "all matches");
    }
}
