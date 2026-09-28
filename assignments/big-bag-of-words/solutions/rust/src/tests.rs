use crate::*;

#[test]
fn test_search() {
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
fn test_is_word() {
    assert!(is_word("Hellö"));
    assert!(!is_word("can't"));
}
