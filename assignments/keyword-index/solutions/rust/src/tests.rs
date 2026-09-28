use crate::*;

#[test]
fn test_search() {
    let t1 = "Hello World!".to_string();
    let target = KWIndex::new().extend_from_text(&t1);
    assert_eq!(0, target.count_matches("Nope!"));
    assert_eq!(1, target.count_matches("World"));
    let t2 = "Goodbye Cruel World!".to_string();
    let target = target.extend_from_text(&t2);
    assert_eq!(2, target.count_matches("World"));
}

#[test]
fn test_nth_uppercase() {
    let text = "A'INT that A SHAME now SON";
    let kwindex = KWIndex::new().extend_from_text(text);

    let shame1 = text.split_whitespace().nth(3).unwrap();
    assert_eq!(shame1, "SHAME");
    let shame2 = kwindex.nth_uppercase(1).unwrap();
    assert!(std::ptr::eq(shame1, shame2));

    let son1 = text.split_whitespace().nth(5).unwrap();
    assert_eq!(son1, "SON");
    let son2 = kwindex.nth_uppercase(2).unwrap();
    assert!(std::ptr::eq(son1, son2));

    assert!(kwindex.nth_uppercase(3).is_none());
}

#[test]
fn test_is_word() {
    assert!(is_word("Hellö"));
    assert!(!is_word("can't"));
}
