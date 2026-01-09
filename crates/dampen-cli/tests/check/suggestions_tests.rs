use dampen_cli::commands::check::suggestions::*;

#[test]
fn test_levenshtein_basic_cases() {
    // Identical strings
    assert_eq!(levenshtein_distance("hello", "hello"), 0);

    // One character difference
    assert_eq!(levenshtein_distance("on_click", "on_clik"), 1);

    // Multiple differences
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
}

#[test]
fn test_levenshtein_empty_strings() {
    assert_eq!(levenshtein_distance("", ""), 0);
    assert_eq!(levenshtein_distance("hello", ""), 5);
    assert_eq!(levenshtein_distance("", "world"), 5);
}

#[test]
fn test_levenshtein_insertions() {
    assert_eq!(levenshtein_distance("click", "clicck"), 1);
    assert_eq!(levenshtein_distance("btn", "button"), 3);
}

#[test]
fn test_levenshtein_deletions() {
    assert_eq!(levenshtein_distance("button", "buton"), 1);
    assert_eq!(levenshtein_distance("handler", "hand"), 3);
}

#[test]
fn test_levenshtein_substitutions() {
    assert_eq!(levenshtein_distance("click", "clack"), 1);
    assert_eq!(levenshtein_distance("press", "prxss"), 1);
}

#[test]
fn test_find_closest_match_exact() {
    let candidates = vec!["on_click", "on_press", "on_change"];
    let result = find_closest_match("on_click", &candidates, 3);
    assert_eq!(result, Some(("on_click", 0)));
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
fn test_find_closest_match_multiple_candidates_same_distance() {
    let candidates = vec!["click", "clack", "clock"];
    let result = find_closest_match("cluck", &candidates, 3);
    // Should return one of them (any is valid since distance is the same)
    assert!(result.is_some());
    let (matched, distance) = result.unwrap();
    assert_eq!(distance, 1);
    assert!(candidates.contains(&matched));
}

#[test]
fn test_find_closest_match_prefers_shorter_distance() {
    let candidates = vec!["increment", "incremnt", "increment_by"];
    let result = find_closest_match("incremnt", &candidates, 3);
    assert_eq!(result, Some(("incremnt", 0))); // Exact match
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
fn test_suggest_empty_candidates() {
    let candidates: Vec<&str> = vec![];
    let result = suggest("on_click", &candidates, 3);
    assert_eq!(result, "");
}

#[test]
fn test_suggest_threshold_boundary() {
    let candidates = vec!["on_click"];

    // Distance is 4 (4 substitutions: on_cxxxk), exceeds threshold of 3
    let result = suggest("on_cxxxk", &candidates, 3);
    assert_eq!(result, "");

    // With threshold of 4, should suggest
    let result = suggest("on_cxxxk", &candidates, 4);
    assert!(result.contains("on_click"));
}

// Property-based tests
#[test]
fn test_levenshtein_symmetry() {
    // Property: distance(a, b) == distance(b, a)
    let pairs = vec![
        ("hello", "world"),
        ("click", "clack"),
        ("increment", "decrement"),
        ("on_press", "on_release"),
    ];

    for (a, b) in pairs {
        assert_eq!(
            levenshtein_distance(a, b),
            levenshtein_distance(b, a),
            "Symmetry failed for '{}' and '{}'",
            a,
            b
        );
    }
}

#[test]
fn test_levenshtein_identity() {
    // Property: distance(a, a) == 0
    let strings = vec!["hello", "on_click", "increment", "value"];

    for s in strings {
        assert_eq!(levenshtein_distance(s, s), 0, "Identity failed for '{}'", s);
    }
}

#[test]
fn test_levenshtein_triangle_inequality() {
    // Property: distance(a, c) <= distance(a, b) + distance(b, c)
    let triples = vec![
        ("hello", "hallo", "hollo"),
        ("click", "clack", "clock"),
        ("increment", "incremnt", "incrment"),
    ];

    for (a, b, c) in triples {
        let dist_ac = levenshtein_distance(a, c);
        let dist_ab = levenshtein_distance(a, b);
        let dist_bc = levenshtein_distance(b, c);

        assert!(
            dist_ac <= dist_ab + dist_bc,
            "Triangle inequality failed: d({}, {}) = {} > d({}, {}) + d({}, {}) = {} + {}",
            a,
            c,
            dist_ac,
            a,
            b,
            b,
            c,
            dist_ab,
            dist_bc
        );
    }
}

#[test]
fn test_levenshtein_non_negativity() {
    // Property: distance(a, b) >= 0 for all a, b
    let pairs = vec![
        ("hello", "world"),
        ("", ""),
        ("a", "b"),
        ("long_string", "short"),
    ];

    for (a, b) in pairs {
        let dist = levenshtein_distance(a, b);
        assert!(dist >= 0, "Non-negativity failed for '{}' and '{}'", a, b);
    }
}

#[test]
fn test_find_closest_match_consistency() {
    // Property: If a candidate is an exact match, it should always be returned
    let candidates = vec!["on_click", "on_press", "on_change", "on_toggle"];

    for &candidate in &candidates {
        let result = find_closest_match(candidate, &candidates, 3);
        assert_eq!(
            result,
            Some((candidate, 0)),
            "Exact match consistency failed for '{}'",
            candidate
        );
    }
}

#[test]
fn test_find_closest_match_threshold_respected() {
    // Property: No result should have distance > threshold
    let candidates = vec!["on_click", "on_press"];
    let test_cases = vec![("on_clik", 1), ("on_clk", 2), ("on_xxx", 5)];

    for (target, expected_min_dist) in test_cases {
        let threshold = 2;
        let result = find_closest_match(target, &candidates, threshold);

        if let Some((_, distance)) = result {
            assert!(
                distance <= threshold,
                "Threshold violated for '{}': got distance {}, threshold {}",
                target,
                distance,
                threshold
            );
        } else {
            // If no result, verify that all distances exceed threshold
            assert!(
                expected_min_dist > threshold,
                "No result for '{}' but expected distance {} <= threshold {}",
                target,
                expected_min_dist,
                threshold
            );
        }
    }
}
