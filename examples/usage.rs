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

fn main() {
    let mut xy = XY::new(5, 0);
    let res = Case::<XY, &str>::new("coords")
        .when("x > 0", |ctx| Ok(ctx.x > 0))
        .then_case("tweak x", |case| {
            case.when("x == 5", |ctx| Ok(ctx.x == 5))
                .then("multiply x by 3", |ctx| {
                    ctx.x *= 3;
                    Ok(())
                })
                .when("when x > 10", |ctx| Ok(ctx.x > 10))
                .then("set x to 10", |ctx| {
                    ctx.x = 10;
                    Ok(())
                })
        })
        .when("y > 0", |ctx| Ok(ctx.y > 0))
        .then_case("tweak y", |case| {
            case.when("y > 0", |ctx| Ok(ctx.y > 0))
                .then("divide 10 by y", |ctx| {
                    ctx.y = 10 / ctx.y;
                    Ok(())
                })
        })
        .run(&mut xy);

    assert_eq!(Ok(true), res);
    assert_eq!(xy.x, 10);
    assert_eq!(xy.y, 0);
}
