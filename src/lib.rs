/*!
# A collection of path matchers.

## Features

- Matches path with another path or glob expression.
- Allows to combine multiple path matchers.

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
path-matchers = "1.0"
```

Use it where appropriate:

```rust
use std::path::PathBuf;
use path_matchers::{any, glob, PathMatcher, PathMatcherExt};

fn main() {
    let path1 = PathBuf::from("images/big/best.png");
    let path2 = PathBuf::from("images/small/32x32/best.jpg");

    // check both paths matches `images/**/best.*`
    let all_best_images = glob("images/**/best.*").unwrap();

    assert!(all_best_images.matches(&path1));
    assert!(all_best_images.matches(&path2));

    let all_jpgs = glob("images/**/*.jpg").unwrap();
    assert!(!all_jpgs.matches(&path1));
    assert!(all_jpgs.matches(&path2));

    let all_pngs = glob("images/**/*.png").unwrap();
    assert!(all_pngs.matches(&path1));
    assert!(!all_pngs.matches(&path2));

    // now we can combine two matchers to match both jpgs and pngs
    let all_pics = all_jpgs.or(all_pngs);
    assert!(all_pics.matches(&path1));
    assert!(all_pics.matches(&path2));

    // you can also use macro for the same
    let all_jpgs = glob("images/**/*.jpg").unwrap();
    let all_pngs = glob("images/**/*.png").unwrap();
    let all_pics = any!(all_jpgs, all_pngs);
    assert!(all_pics.matches(&path1));
    assert!(all_pics.matches(&path2));
}
```

*/

mod path_matcher_and;
#[doc(hidden)]
pub mod path_matcher_any;
mod path_matcher_equal;
mod path_matcher_func;
#[cfg(feature = "glob")]
mod path_matcher_glob;
mod path_matcher_not;
mod path_matcher_or;
mod path_matcher_starts_with;
#[cfg(test)]
mod path_matcher_true;

use std::path::Path;

/// Allows to match a path against implemented condition.
pub trait PathMatcher {
    /// Matches path against this matcher.
    fn matches(&self, path: &Path) -> bool;

    /// Converts this matcher to boxed representation.
    ///
    /// Can be used for [`any_of`].
    fn boxed<'a>(self) -> Box<dyn PathMatcher + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }
}

/// Extends [`PathMatcher`] with combinator functions.
pub trait PathMatcherExt: PathMatcher + Sized {
    fn and<R: PathMatcher>(self, right: R) -> PathMatcherAnd<Self, R> {
        PathMatcherAnd(self, right)
    }

    fn or<R: PathMatcher>(self, right: R) -> PathMatcherOr<Self, R> {
        PathMatcherOr(self, right)
    }
}

impl<F> PathMatcherExt for F where F: PathMatcher + Sized {}

impl<F> PathMatcher for F
where
    F: Fn(&Path) -> bool,
{
    fn matches(&self, path: &Path) -> bool {
        self(path)
    }
}

pub use path_matcher_and::and;
use path_matcher_and::PathMatcherAnd;
pub use path_matcher_any::any_of;
pub use path_matcher_equal::equal;
pub use path_matcher_func::func;
#[cfg(feature = "glob")]
pub use path_matcher_glob::{glob, PatternError};
pub use path_matcher_not::not;
pub use path_matcher_or::or;
use path_matcher_or::PathMatcherOr;
pub use path_matcher_starts_with::starts_with;
