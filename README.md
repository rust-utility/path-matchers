# A collection of useful path matchers

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

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
