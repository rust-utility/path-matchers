use crate::PathMatcher;
use std::path::Path;

/// Checks any of matchers in iterator is true.
pub fn any_of<I: IntoIterator<Item = Box<dyn PathMatcher>>>(iter: I) -> impl PathMatcher {
    PathMatcherAny(iter.into_iter().collect())
}

pub struct PathMatcherAny(Vec<Box<dyn PathMatcher>>);

impl PathMatcher for PathMatcherAny {
    fn matches(&self, path: &Path) -> bool {
        self.0.iter().any(|matcher| matcher.matches(path))
    }
}

/// Checks any of matchers is true.
///
/// You can use from 1 to 10 matchers with `any!`.
///
/// # Examples:
///
/// ```
/// use std::path::PathBuf;
/// use path_matchers::{any, equal, PathMatcher};
///
/// let path1 = PathBuf::from("readme.md");
/// let path2 = PathBuf::from("todo.md");
///
/// assert!(any!(equal("readme.md"), equal("other.md")).matches(&path1));
/// assert!(!any!(equal("readme.md"), equal("other.md")).matches(&path2));
/// assert!(any!(equal("readme.md"), equal("other.md"), equal("todo.md")).matches(&path2));
/// ```
#[macro_export]
macro_rules! any {
    ($a1: expr) => {
        $crate::path_matcher_any::any1($a1)
    };
    ($a1: expr, $a2: expr) => {
        $crate::path_matcher_any::any2($a1, $a2)
    };
    ($a1: expr, $a2: expr, $a3: expr) => {
        $crate::path_matcher_any::any3($a1, $a2, $a3)
    };
    ($a1: expr, $a2: expr, $a3: expr, $a4: expr) => {
        $crate::path_matcher_any::any4($a1, $a2, $a3, $a4)
    };
    ($a1: expr, $a2: expr, $a3: expr, $a4: expr, $a5: expr) => {
        $crate::path_matcher_any::any5($a1, $a2, $a3, $a4, $a5)
    };
    ($a1: expr, $a2: expr, $a3: expr, $a4: expr, $a5: expr, $a6: expr) => {
        $crate::path_matcher_any::any6($a1, $a2, $a3, $a4, $a5, $a6)
    };
    ($a1: expr, $a2: expr, $a3: expr, $a4: expr, $a5: expr, $a6: expr, $a7: expr) => {
        $crate::path_matcher_any::any7($a1, $a2, $a3, $a4, $a5, $a6, $a7)
    };
    ($a1: expr, $a2: expr, $a3: expr, $a4: expr, $a5: expr, $a6: expr, $a7: expr, $a8: expr) => {
        $crate::path_matcher_any::any8($a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8)
    };
    ($a1: expr, $a2: expr, $a3: expr, $a4: expr, $a5: expr, $a6: expr, $a7: expr, $a8: expr, $a9: expr) => {
        $crate::path_matcher_any::any9($a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9)
    };
    ($a1: expr, $a2: expr, $a3: expr, $a4: expr, $a5: expr, $a6: expr, $a7: expr, $a8: expr, $a9: expr, $a10: expr) => {
        $crate::path_matcher_any::any10($a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9, $a10)
    };
}

macro_rules! any_path_matcher {
    ($fn: ident, $id: ident, $($ty: ident : $tyy: ident),+) => {
        pub fn $fn<$($tyy : PathMatcher),+>($($ty: $tyy),+) -> impl PathMatcher {
            $id{$($ty),+}
        }
        struct $id<$($tyy : PathMatcher),+>{$($ty: $tyy),+}
        impl <$($tyy : PathMatcher),+> PathMatcher for $id < $($tyy),+ > {
            fn matches(&self, path: &Path) -> bool {
                $(self.$ty.matches(path) )||+
            }
        }
    };
}

any_path_matcher!(any1, Any1, a1: A1);

any_path_matcher!(any2, Any2, a1: A1, a2: A2);

any_path_matcher!(any3, Any3, a1: A1, a2: A2, a3: A3);

any_path_matcher!(any4, Any4, a1: A1, a2: A2, a3: A3, a4: A4);

any_path_matcher!(any5, Any5, a1: A1, a2: A2, a3: A3, a4: A4, a5: A5);

any_path_matcher!(any6, Any6, a1: A1, a2: A2, a3: A3, a4: A4, a5: A5, a6: A6);

any_path_matcher!(
    any7,
    Any7,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
    a7: A7
);

any_path_matcher!(
    any8,
    Any8,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
    a7: A7,
    a8: A8
);

any_path_matcher!(
    any9,
    Any9,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
    a7: A7,
    a8: A8,
    a9: A9
);

any_path_matcher!(
    any10,
    Any10,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
    a7: A7,
    a8: A8,
    a9: A9,
    a10: A10
);

#[cfg(test)]
mod tests {
    use crate::{any_of, equal, not, path_matcher_true::path_matcher_true as true_, PathMatcher};
    use std::path::PathBuf;

    #[test]
    fn path_matcher_any_of() {
        let some_path = PathBuf::from("anything");
        assert_eq!(any_of(vec![true_().boxed()]).matches(&some_path), true);
        assert_eq!(
            any_of(vec![not(true_()).boxed()]).matches(&some_path),
            false
        );
        assert_eq!(
            any_of(vec![true_().boxed(), true_().boxed()]).matches(&some_path),
            true
        );
        assert_eq!(
            any_of(vec![not(true_()).boxed(), true_().boxed()]).matches(&some_path),
            true
        );
        assert_eq!(
            any_of(vec![true_().boxed(), not(true_()).boxed()]).matches(&some_path),
            true
        );
        assert_eq!(
            any_of(vec![not(true_()).boxed(), not(true_()).boxed()]).matches(&some_path),
            false
        );
    }

    #[test]
    fn path_matcher_any() {
        let some_path = PathBuf::from("anything");
        assert_eq!(any!(true_()).matches(&some_path), true);
        assert_eq!(any!(not(true_())).matches(&some_path), false);
        assert_eq!(any!(true_(), true_()).matches(&some_path), true);
        assert_eq!(any!(not(true_()), true_()).matches(&some_path), true);
        assert_eq!(any!(true_(), not(true_())).matches(&some_path), true);
        assert_eq!(any!(not(true_()), not(true_())).matches(&some_path), false);

        let path = PathBuf::from("1");
        assert!(any!(equal("1")).matches(&path));
        assert!(any!(equal("1"), equal("2")).matches(&path));
        assert!(any!(equal("1"), equal("2"), equal("3")).matches(&path));
        assert!(any!(equal("1"), equal("2"), equal("3"), equal("4")).matches(&path));
        assert!(any!(equal("1"), equal("2"), equal("3"), equal("4"), equal("5")).matches(&path));
        assert!(any!(
            equal("1"),
            equal("2"),
            equal("3"),
            equal("4"),
            equal("5"),
            equal("6")
        )
        .matches(&path));
        assert!(any!(
            equal("1"),
            equal("2"),
            equal("3"),
            equal("4"),
            equal("5"),
            equal("6"),
            equal("7")
        )
        .matches(&path));
        assert!(any!(
            equal("1"),
            equal("2"),
            equal("3"),
            equal("4"),
            equal("5"),
            equal("6"),
            equal("7"),
            equal("8")
        )
        .matches(&path));
        assert!(any!(
            equal("1"),
            equal("2"),
            equal("3"),
            equal("4"),
            equal("5"),
            equal("6"),
            equal("7"),
            equal("8"),
            equal("9")
        )
        .matches(&path));
        assert!(any!(
            equal("1"),
            equal("2"),
            equal("3"),
            equal("4"),
            equal("5"),
            equal("6"),
            equal("7"),
            equal("8"),
            equal("9"),
            equal("10")
        )
        .matches(&path));
    }
}
