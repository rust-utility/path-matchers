use crate::PathMatcher;
use glob::Pattern;
use std::path::Path;

pub use glob::PatternError;

/// Matches file paths against Unix shell style patterns.
pub fn glob(pattern: &str) -> Result<impl PathMatcher, PatternError> {
    Pattern::new(pattern).map(PathMatcherGlob)
}

struct PathMatcherGlob(Pattern);

impl PathMatcher for PathMatcherGlob {
    fn matches(&self, path: &Path) -> bool {
        self.0.matches_path(path)
    }
}

#[cfg(test)]
mod tests {

    use super::glob;
    use crate::PathMatcher;
    use std::path::PathBuf;

    #[test]
    fn path_matcher_glob() {
        let path = PathBuf::from("abc");
        assert_eq!(glob("abc").unwrap().matches(&path), true);
        assert_eq!(glob("a*").unwrap().matches(&path), true);
        assert_eq!(glob("a*e").unwrap().matches(&path), false);
    }
}
