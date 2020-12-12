use crate::PathMatcher;
use std::path::{Path, PathBuf};

/// Checks a path for exact match.
pub fn equal<P: Into<PathBuf>>(path: P) -> impl PathMatcher {
    PathMatcherEqual(path.into())
}

struct PathMatcherEqual(PathBuf);

impl PathMatcher for PathMatcherEqual {
    fn matches(&self, path: &Path) -> bool {
        path == self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{equal, PathMatcher};
    use std::path::PathBuf;

    #[test]
    fn path_matcher_equal() {
        let matched_path = PathBuf::from("hello/world");
        assert!(equal("hello/world").matches(&matched_path));
        assert!(!equal("bye/world").matches(&matched_path));
    }
}
