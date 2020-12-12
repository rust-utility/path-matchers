use crate::PathMatcher;
use std::path::Path;

/// Creates matcher based on function.
pub fn func<F: Fn(&Path) -> bool>(f: F) -> impl PathMatcher {
    PathMatcherFunc(f)
}

pub struct PathMatcherFunc<F: Fn(&Path) -> bool>(F);

impl<F: Fn(&Path) -> bool> PathMatcher for PathMatcherFunc<F> {
    fn matches(&self, path: &Path) -> bool {
        self.0(path)
    }
}

#[cfg(test)]
mod tests {

    use super::func;
    use crate::PathMatcher;
    use std::path::PathBuf;

    #[test]
    fn path_matcher_and() {
        let path1 = PathBuf::from("hello/world");
        let path2 = PathBuf::from("hello/world2");
        let func_matcher = func(|p| p == path1);
        assert!(func_matcher.matches(&path1));
        assert!(!func_matcher.matches(&path2));
    }
}
