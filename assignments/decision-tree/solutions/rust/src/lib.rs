//! Directory Tree Simulator: Provides a directory tree structure and an operating system stub
//! structure to interact with it.
//!
//! ## Paths
//!
//! A directory can be specified by a *path*: either an *absolute path* from the root or a
//! *relative path* from the current working directory. In this library, paths are normally
//! specified as sequences of path *segments*: each path segment specifies a "next" directory
//! along the path. For conventional reasons, a directory name / path segment must be valid UTF-8
//! and is not allowed to contain a `/` character.

// Bart Massey 2021

/// Errors during directory interaction.
#[derive(Debug)]
pub enum DirError<'a> {
    /// The character `/` in component names is disallowed,
    /// to make path separators easier.
    SlashInName(&'a str),
    /// Only one subdirectory of a given name can exist in any directory.
    DirExists(&'a str),
    /// Traversal failed due to missing subdirectory.
    InvalidChild(&'a str),
}

impl std::fmt::Display for DirError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SlashInName(name) => write!(f, "{name}: slash in name is invalid"),
            Self::DirExists(name) => write!(f, "{name}: directory exists"),
            Self::InvalidChild(name) => write!(f, "{name}: invalid element in path"),
        }
    }
}

impl std::error::Error for DirError<'_> {}

/// Result type for directory errors.
pub type Result<'a, T> = std::result::Result<T, DirError<'a>>;

/// A directory entry. Component names are stored externally.
#[derive(Debug, Clone)]
pub struct DEnt<'a> {
    pub name: &'a str,
    pub subdir: DTree<'a>,
}

/// A directory tree.
#[derive(Debug, Clone, Default)]
pub struct DTree<'a> {
    pub children: Vec<DEnt<'a>>,
}

/// Operating system state: the directory tree and the current working directory.
/// The current working directory is represented as a path from the root.
#[derive(Debug, Clone, Default)]
pub struct OsState<'a, 'b> {
    pub dtree: DTree<'a>,
    pub cwd: Vec<&'b str>,
}

fn sanitize(name: &str) -> Result<'_, &str> {
    if name.contains('/') {
        return Err(DirError::SlashInName(name));
    }
    Ok(name)
}

impl<'a> DEnt<'a> {
    pub fn new(name: &'a str) -> Result<'a, Self> {
        let dent = DEnt {
            name: sanitize(name)?,
            subdir: DTree::new(),
        };
        Ok(dent)
    }
}

impl<'a> DTree<'a> {
    /// Create a new empty directory tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Make a subdirectory with the given name in this directory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("test").unwrap();
    /// assert_eq!(&dt.paths(), &["/test/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::SlashInName` if `name` contains `/`.
    /// * `DirError::DirExists` if `name` already exists.
    pub fn mkdir(&mut self, name: &'a str) -> Result<'_, ()> {
        let name = sanitize(name)?;
        if self.with_subdir(&[name], |_| ()).is_ok() {
            return Err(DirError::DirExists(name));
        }
        self.children.push(DEnt::new(name)?);
        Ok(())
    }

    /// Traverse to the subdirectory given by `path` and then call `f` to visit the subdirectory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("test").unwrap();
    /// let paths = dt.with_subdir(&["test"], |dt| dt.paths()).unwrap();
    /// assert_eq!(&paths, &["/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if `path` is invalid.
    pub fn with_subdir<'b, 'c, F, R>(&'b self, path: &[&'c str], f: F) -> Result<'c, R>
    where
        F: FnOnce(&'b DTree<'a>) -> R,
    {
        let mut cur = self;
        for p in path {
            let mut found = false;
            for child in &cur.children {
                if &child.name == p {
                    cur = &child.subdir;
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(DirError::InvalidChild(p));
            }
        }
        Ok(f(cur))
    }

    /// Traverse to the subdirectory given by `path` and then call `f` to visit the subdirectory
    /// mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("a").unwrap();
    /// dt.with_subdir_mut(&["a"], |dt| dt.mkdir("b").unwrap()).unwrap();
    /// assert_eq!(&dt.paths(), &["/a/b/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if `path` is invalid.
    pub fn with_subdir_mut<'b, 'c, F, R>(&'b mut self, path: &[&'c str], f: F) -> Result<'c, R>
    where
        F: FnOnce(&'b mut DTree<'a>) -> R,
    {
        fn find_child<'a, 'b, 'c>(
            cur: &'b mut DTree<'a>,
            p: &'c str,
        ) -> Result<'c, &'b mut DTree<'a>> {
            for c in &mut cur.children {
                if c.name == p {
                    return Ok(&mut c.subdir);
                }
            }
            Err(DirError::InvalidChild(p))
        }

        let mut cur = self;
        for p in path {
            cur = find_child(cur, p)?;
        }
        Ok(f(cur))
    }

    /// Produce a list of the paths to each reachable leaf, in no particular order.
    ///
    /// Since the primary use of this function is for user-readable output, the choice was made to
    /// represent each path as a `String` rather than a `Vec` of path segments. A path string
    /// returned by this function will have `/` at the beginning, at the end, and separating each
    /// segment in the path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("a").unwrap();
    /// dt.with_subdir_mut(&["a"], |dt| dt.mkdir("b").unwrap()).unwrap();
    /// dt.with_subdir_mut(&["a"], |dt| dt.mkdir("c").unwrap()).unwrap();
    /// dt.with_subdir_mut(&["a", "b"], |dt| dt.mkdir("d").unwrap()).unwrap();
    /// let mut paths = dt.paths();
    /// paths.sort();
    /// assert_eq!(&paths, &["/a/b/d/", "/a/c/"]);
    /// ```
    pub fn paths(&self) -> Vec<String> {
        fn descend(prefix: String, cur: &DTree) -> Vec<String> {
            if cur.children.is_empty() {
                return vec![prefix];
            }
            let mut result = Vec::new();
            for d in &cur.children {
                let mut subprefix = prefix.clone();
                subprefix += d.name;
                subprefix.push('/');
                result.extend(descend(subprefix, &d.subdir));
            }
            result
        }

        descend("/".to_string(), self)
    }
}

impl<'a, 'b> OsState<'a, 'b> {
    /// Create a new directory tree in the operating system.  Current working directory is the
    /// root.
    pub fn new() -> Self {
        Self::default()
    }

    /// If `path` is empty, change the working directory to the root.  Otherwise change the
    /// working directory to the subdirectory given by `path` relative to the current working
    /// directory.  (There is no notion of `.` or `..`: `path` must be a valid sequence of
    /// component names.)
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::OsState;
    /// let mut s = OsState::new();
    /// s.mkdir("a").unwrap();
    /// s.chdir(&["a"]).unwrap();
    /// s.mkdir("b").unwrap();
    /// s.chdir(&["b"]).unwrap();
    /// s.mkdir("c").unwrap();
    /// s.chdir(&[]).unwrap();
    /// assert_eq!(&s.paths().unwrap(), &["/a/b/c/"]);
    /// s.chdir(&["a", "b", "c"]).unwrap();
    /// assert_eq!(s.paths().unwrap(), &["/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if the new working directory is invalid. On error, the original
    ///   working directory will be retained.
    pub fn chdir(&mut self, path: &[&'b str]) -> Result<'b, ()> {
        if path.is_empty() {
            self.cwd.clear();
            return Ok(());
        }
        let npath = self.cwd.len();
        self.cwd.extend(path);
        if let Err(e) = self.dtree.with_subdir(&self.cwd, |_| ()) {
            self.cwd.truncate(npath);
            return Err(e);
        }
        Ok(())
    }

    /// Make a new subdirectory with the given `name` in the working directory.
    ///
    /// # Errors
    ///
    /// * `DirError::SlashInName` if `name` contains `/`.
    /// * `DirError::InvalidChild` if the current working directory is invalid.
    /// * `DirError::DirExists` if `name` already exists.
    pub fn mkdir(&mut self, name: &'a str) -> Result<'_, ()> {
        self.dtree.with_subdir_mut(&self.cwd, |dt| dt.mkdir(name))?
    }

    /// Produce a list of the paths from the working directory to each reachable leaf, in no
    /// particular order. See [DTree::paths] for details.
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if the current working directory is invalid.
    pub fn paths(&self) -> Result<'_, Vec<String>> {
        self.dtree.with_subdir(&self.cwd, |dt| dt.paths())
    }
}
