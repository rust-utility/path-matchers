use crate::PathMatcher;
use std::path::Path;

/// Checks both matchers are true.
pub fn and<L: PathMatcher, R: PathMatcher>(left: L, right: R) -> impl PathMatcher {
    PathMatcherAnd(left, right)
}

pub struct PathMatcherAnd<L: PathMatcher, R: PathMatcher>(pub(crate) L, pub(crate) R);

impl<L: PathMatcher, R: PathMatcher> PathMatcher for PathMatcherAnd<L, R> {
    fn matches(&self, path: &Path) -> bool {
        self.0.matches(path) && self.1.matches(path)
    }
}

#[cfg(test)]
mod tests {

    use super::and;
    use crate::{not, path_matcher_true::path_matcher_true as true_, PathMatcher};
    use std::path::PathBuf;

    #[test]
    fn path_matcher_and() {
        let path = PathBuf::from("hello/world");
        assert_eq!(and(true_(), true_()).matches(&path), true);
        assert_eq!(and(true_(), not(true_())).matches(&path), false);
        assert_eq!(and(not(true_()), true_()).matches(&path), false);
        assert_eq!(and(not(true_()), not(true_())).matches(&path), false);
    }
}
