
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

const ALPHABET_SIZE: usize = 256;

fn get_bad_character_table(pattern: &str) -> Vec<[i32; ALPHABET_SIZE + 1]> {
    let mut result = Vec::with_capacity(pattern.len());
    for i in 0..pattern.len() {
        result.push([-1 as i32; ALPHABET_SIZE + 1]);
    }
    
    let mut alpha: [i32; ALPHABET_SIZE] = [-1; ALPHABET_SIZE];
    let mut indexes: [usize; ALPHABET_SIZE] = [1; ALPHABET_SIZE];

    for (index, value) in pattern.chars().enumerate() {
        alpha[value as usize] = index as i32;

        for (index_2, alpha_value) in alpha.iter().enumerate() {
            let inner_index = indexes[index_2];
            
            result[index_2][inner_index] = *alpha_value;
            indexes[index_2] += 1;
        }
    }
    return result;
}

/*
 * Used in strong good suffix rule.
 * L[i] = k, 
*/
fn get_good_suffix_table(pattern: &str) -> Vec<i32> {
    let mut result = Vec::with_capacity(pattern.len());

    return result;
}

pub fn contains(pattern: &str, input: File) -> bool {

    if pattern.len() == 0 {
        return false;
    }

            
    let mut buf_reader = BufReader::new(input);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);

    if contents.len() == 0 || pattern.len() > contents.len() {
        return false;
    }


    let bad_char_table = get_bad_character_table(pattern);
    let good_suffix = get_good_suffix_table(pattern);
    
    let mut k = pattern.len() - 1; // alignment of the end of the pattern relative to the search
    let mut prev_k = -1;

    while k < contents.len() {
        let mut pattern_index = pattern.len() - 1; // index to search in the pattern
        let mut contents_index = k;  // index to search in content
        
    }
    

    
    return contents.contains(pattern);
}
