use crate::PathMatcher;
use std::path::Path;

pub(crate) fn path_matcher_true() -> impl PathMatcher {
    PathMatcherTrue
}

struct PathMatcherTrue;

impl PathMatcher for PathMatcherTrue {
    fn matches(&self, _path: &Path) -> bool {
        true
    }
}
