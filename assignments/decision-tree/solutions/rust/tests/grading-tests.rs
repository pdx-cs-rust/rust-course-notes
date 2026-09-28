use dtree::*;

#[test]
fn test_dtree_new() {
    let dt = DTree::new();
    assert_eq!(&dt.paths(), &["/"]);
}

#[test]
fn test_dtree_mkdir() {
    let mut dt = DTree::new();
    dt.mkdir("bar").unwrap();
    dt.mkdir("bletch").unwrap();
    let mut paths = dt.paths();
    paths.sort();
    assert_eq!(&paths, &["/bar/", "/bletch/"]);
    match dt.mkdir("a/b") {
        Err(DirError::SlashInName("a/b")) => (),
        _ => panic!("should have failed"),
    }
}

#[test]
fn test_dtree_with_subdir_mut() {
    let mut dt = DTree::new();
    dt.with_subdir_mut(&[], |dt| dt.mkdir("bar"))
        .unwrap()
        .unwrap();
    dt.with_subdir_mut(&["bar"], |dt| dt.mkdir("bletch"))
        .unwrap()
        .unwrap();
    dt.with_subdir_mut(&[], |dt| dt.mkdir("bogus").unwrap())
        .unwrap();
    let mut paths = dt.paths();
    paths.sort();
    assert_eq!(&paths, &["/bar/bletch/", "/bogus/"]);
}

#[test]
fn test_os_state() {
    let mut os = OsState::new();
    os.mkdir("bar").unwrap();
    os.chdir(&["bar"]).unwrap();
    os.mkdir("bletch").unwrap();
    os.chdir(&[]).unwrap();
    os.mkdir("bogus").unwrap();
    let mut paths = os.paths().unwrap();
    paths.sort();
    assert_eq!(&paths, &["/bar/bletch/", "/bogus/"]);
}
