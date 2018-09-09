# tweak

[![Crates.io](https://img.shields.io/crates/v/tweak.svg)](https://crates.io/crates/tweak)
[![Documentation](https://docs.rs/tweak/badge.svg)](https://docs.rs/tweak/)
[![Build Status](https://travis-ci.org/zoer/tweak.svg?branch=master)](https://travis-ci.org/zoer/tweak)

Tweak provides when/then clauses to your context in described state.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tweak = "*"
```

## Examples

Simple context manipulation.

```rust
extern crate tweak;

use tweak::Case;

struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

let mut xy = XY::new(5, 0);
let res = Case::<XY, &str>::new("coords")
    .when("x > 0", |xy| Ok(xy.x > 0))
    .then_case("tweak x", |case| {
        case.when("x == 5", |xy| Ok(xy.x == 5))
            .then("multiply x by 3", |xy| {
                xy.x *= 3;
                Ok(())
            })
            .when("when x > 10", |xy| Ok(xy.x > 10))
            .then("set x to 10", |xy| {
                xy.x = 10;
                Ok(())
            })
    })
    .when("y > 0", |xy| Ok(xy.y > 0))
    .then_case("tweak y", |case| {
        case.when("y > 0", |xy| Ok(xy.y > 0))
            .then("divide 10 by y", |xy| {
                xy.y = 10 / xy.y;
                Ok(())
            })
    })
    .run(&mut xy);

assert_eq!(Ok(true), res);
assert_eq!(xy.x, 10);
assert_eq!(xy.y, 0);
```

License: MIT
