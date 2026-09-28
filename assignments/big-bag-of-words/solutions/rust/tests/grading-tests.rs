use std::sync::LazyLock;

use bbow::*;

static SHORT_BBOW: LazyLock<Bbow> = LazyLock::new(|| {
    Bbow::new().extend_from_text(
        "Four-score AND seven years' ago our Fourfathers and Seven Sons
         said \"ago west young sons\"."
    )
});

#[test]
fn test_basics() {
    let short = SHORT_BBOW.clone();
    assert!(!short.is_empty());
    let nshort = short.len();
    assert_eq!(10, nshort);
    let nshort_count = short.count();
    assert_eq!(14, nshort_count);

    let empty = Bbow::new();
    assert!(empty.is_empty());
}

#[test]
fn test_search_tiny() {
    let t1 = "Hello World!".to_string();
    let target = Bbow::new().extend_from_text(&t1);
    assert_eq!(0, target.match_count("Nope!"));
    assert_eq!(0, target.match_count("World"));
    assert_eq!(1, target.match_count("world"));
    let t2 = "Goodbye Cruel World!".to_string();
    let target = target.extend_from_text(&t2);
    assert_eq!(2, target.match_count("world"));
}

#[test]
fn test_search_short() {
    let short = SHORT_BBOW.clone();
    let match_counts = &[
        (0, "nonexistent"),
        (0, "four-score"),
        (0, "AND"),
        (0, "Seven"),
        (2, "and"),
        (2, "seven"),
        (1, "years"),
        (2, "ago"),
        (2, "sons"),
    ];
    for &(count, word) in match_counts {
        assert_eq!(count, short.match_count(word));
    }
}

#[test]
fn test_double_extend_words() {
    let tiny = Bbow::new()
        .extend_from_text("hello world")
        .extend_from_text("world war");
    let mut words: Vec<_> = tiny.words().collect();
    words.sort();
    assert_eq!(words, vec!["hello", "war", "world"]);
    let match_counts = &[
        (1, "hello"),
        (1, "war"),
        (2, "world"),
    ];
    for &(count, word) in match_counts {
        assert_eq!(count, tiny.match_count(word));
    }
}
