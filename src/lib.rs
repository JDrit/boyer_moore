///
/// Example searches
///
/// ```
/// use boyer_moore::search::search;
///
/// let results: Vec<usize> = search::search_string("test", "search for test".to_string());
/// assert_eq!(1, results.len());
/// ```
pub mod search;
