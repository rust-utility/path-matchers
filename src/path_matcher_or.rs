use crate::PathMatcher;
use std::path::Path;

/// Checks at least one of two matchers is true.
pub fn or<L: PathMatcher, R: PathMatcher>(left: L, right: R) -> impl PathMatcher {
    PathMatcherOr(left, right)
}

pub struct PathMatcherOr<L: PathMatcher, R: PathMatcher>(pub(crate) L, pub(crate) R);

impl<L: PathMatcher, R: PathMatcher> PathMatcher for PathMatcherOr<L, R> {
    fn matches(&self, path: &Path) -> bool {
        self.0.matches(path) || self.1.matches(path)
    }
}

#[cfg(test)]
mod tests {

    use super::or;
    use crate::{not, path_matcher_true::path_matcher_true as true_, PathMatcher};
    use std::path::PathBuf;

    #[test]
    fn path_matcher_or() {
        let path = PathBuf::from("hello/world");
        assert_eq!(or(true_(), true_()).matches(&path), true);
        assert_eq!(or(true_(), not(true_())).matches(&path), true);
        assert_eq!(or(not(true_()), true_()).matches(&path), true);
        assert_eq!(or(not(true_()), not(true_())).matches(&path), false);
    }
}
