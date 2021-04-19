use std::path::{Path, PathBuf};

use crate::PathMatcher;

/// Checks a path starts with another path.
pub fn starts_with<P: Into<PathBuf>>(p: P) -> impl PathMatcher {
    PathMatcherStartsWith(p.into())
}

struct PathMatcherStartsWith(PathBuf);

impl PathMatcher for PathMatcherStartsWith {
    fn matches(&self, path: &Path) -> bool {
        path.starts_with(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{starts_with, PathMatcher};
    use std::path::PathBuf;

    #[test]
    fn path_matcher_starts_with() {
        let matched_path = PathBuf::from("hello/world");
        assert!(starts_with("hello").matches(&matched_path));
        assert!(!starts_with("bye").matches(&matched_path));
    }
}
