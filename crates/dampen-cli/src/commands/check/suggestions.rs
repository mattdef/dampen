/// Levenshtein distance algorithm for string similarity.
///
/// Calculates the minimum number of single-character edits (insertions, deletions, or substitutions)
/// required to change one string into another.
///
/// # Arguments
///
/// * `a` - First string to compare
/// * `b` - Second string to compare
///
/// # Returns
///
/// The Levenshtein distance between the two strings.
///
/// # Examples
///
/// ```
/// use dampen_cli::commands::check::suggestions::levenshtein_distance;
///
/// assert_eq!(levenshtein_distance("on_click", "on_clik"), 1);
/// assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
/// ```
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();

    // Early exit optimizations
    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    // Create distance matrix
    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    // Initialize first row and column
    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    // Compute distances using dynamic programming
    for (i, a_char) in a.chars().enumerate() {
        for (j, b_char) in b.chars().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };

            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1, // deletion
                    matrix[i + 1][j] + 1, // insertion
                ),
                matrix[i][j] + cost, // substitution
            );
        }
    }

    matrix[a_len][b_len]
}

/// Finds the closest match to a target string from a list of candidates.
///
/// Returns the closest match if the Levenshtein distance is within the threshold.
///
/// # Arguments
///
/// * `target` - The string to find a match for
/// * `candidates` - List of candidate strings to compare against
/// * `threshold` - Maximum Levenshtein distance to consider (default: 3)
///
/// # Returns
///
/// An `Option` containing the closest match and its distance, or `None` if no match is within the threshold.
///
/// # Examples
///
/// ```
/// use dampen_cli::commands::check::suggestions::find_closest_match;
///
/// let candidates = vec!["on_click", "on_press", "on_change"];
/// let result = find_closest_match("on_clik", &candidates, 3);
///
/// assert_eq!(result, Some(("on_click", 1)));
/// ```
pub fn find_closest_match<'a>(
    target: &str,
    candidates: &[&'a str],
    threshold: usize,
) -> Option<(&'a str, usize)> {
    let mut best_match: Option<(&'a str, usize)> = None;

    for &candidate in candidates {
        let distance = levenshtein_distance(target, candidate);

        if distance <= threshold {
            match best_match {
                None => best_match = Some((candidate, distance)),
                Some((_, best_distance)) if distance < best_distance => {
                    best_match = Some((candidate, distance));
                }
                _ => {}
            }
        }
    }

    best_match
}

/// Generates a suggestion message for an unknown string.
///
/// # Arguments
///
/// * `unknown` - The unknown string
/// * `candidates` - List of valid candidates
/// * `threshold` - Maximum distance for suggestions (default: 3)
///
/// # Returns
///
/// A formatted suggestion string, or an empty string if no suggestion found.
///
/// # Examples
///
/// ```
/// use dampen_cli::commands::check::suggestions::suggest;
///
/// let candidates = vec!["on_click", "on_press"];
/// let result = suggest("on_clik", &candidates, 3);
///
/// assert_eq!(result, " Did you mean 'on_click'? (distance: 1)");
/// ```
pub fn suggest(unknown: &str, candidates: &[&str], threshold: usize) -> String {
    if let Some((suggestion, distance)) = find_closest_match(unknown, candidates, threshold) {
        format!(" Did you mean '{}'? (distance: {})", suggestion, distance)
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance_identical() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
    }

    #[test]
    fn test_levenshtein_distance_one_substitution() {
        assert_eq!(levenshtein_distance("on_click", "on_clik"), 1);
    }

    #[test]
    fn test_levenshtein_distance_one_insertion() {
        assert_eq!(levenshtein_distance("click", "clicck"), 1);
    }

    #[test]
    fn test_levenshtein_distance_one_deletion() {
        assert_eq!(levenshtein_distance("button", "buton"), 1);
    }

    #[test]
    fn test_levenshtein_distance_multiple_edits() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn test_levenshtein_distance_empty_strings() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("hello", ""), 5);
        assert_eq!(levenshtein_distance("", "world"), 5);
    }

    #[test]
    fn test_find_closest_match_within_threshold() {
        let candidates = vec!["on_click", "on_press", "on_change"];
        let result = find_closest_match("on_clik", &candidates, 3);
        assert_eq!(result, Some(("on_click", 1)));
    }

    #[test]
    fn test_find_closest_match_exceeds_threshold() {
        let candidates = vec!["on_click", "on_press"];
        let result = find_closest_match("completely_different", &candidates, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_closest_match_multiple_candidates() {
        let candidates = vec!["increment", "decrement", "increment_by"];
        let result = find_closest_match("incremnt", &candidates, 3);
        assert_eq!(result, Some(("increment", 1)));
    }

    #[test]
    fn test_suggest_with_match() {
        let candidates = vec!["on_click", "on_press"];
        let result = suggest("on_clik", &candidates, 3);
        assert_eq!(result, " Did you mean 'on_click'? (distance: 1)");
    }

    #[test]
    fn test_suggest_without_match() {
        let candidates = vec!["on_click", "on_press"];
        let result = suggest("completely_different", &candidates, 3);
        assert_eq!(result, "");
    }

    #[test]
    fn test_suggest_threshold_boundary() {
        let candidates = vec!["on_click"];
        // Distance is 3 (3 substitutions: xxx->lic), which equals threshold of 3
        let result = suggest("on_cxxxk", &candidates, 3);
        assert!(result.contains("on_click"));

        // With threshold of 2, should not suggest (distance exceeds threshold)
        let result = suggest("on_cxxxk", &candidates, 2);
        assert_eq!(result, "");
    }
}
