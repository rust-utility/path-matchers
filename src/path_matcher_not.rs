use crate::PathMatcher;
use std::path::Path;

/// Reverts the match condition of the underlying matcher.
pub fn not<A: PathMatcher>(matcher: A) -> impl PathMatcher {
    PathMatcherNot(matcher)
}

struct PathMatcherNot<A: PathMatcher>(A);

impl<A: PathMatcher> PathMatcher for PathMatcherNot<A> {
    fn matches(&self, path: &Path) -> bool {
        !self.0.matches(path)
    }
}

#[cfg(test)]
mod tests {

    use super::{not, PathMatcher};
    use crate::path_matcher_true::path_matcher_true;
    use std::path::PathBuf;

    #[test]
    fn path_matcher_not() {
        let path = PathBuf::from("hello/world");
        assert_eq!(not(path_matcher_true()).matches(&path), false);
    }
}
