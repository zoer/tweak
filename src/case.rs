use super::{then::Then, when::When, Execute, Result, ThenFn, WhenFn};

/// Case holds group of when/then clauses
pub struct Case<C, E> {
    #[allow(dead_code)]
    name: String,
    cases: Vec<(Option<When<C, E>>, Option<Box<Execute<C, E>>>)>,
}

impl<C: 'static, E: 'static> Case<C, E> {
    /// Create a new case group
    ///
    /// # Examples
    ///
    /// ```
    /// use tweak::Case;
    ///
    /// let case = Case::<String, ()>::new("string manipulation");
    /// ```
    pub fn new<T: Into<String>>(name: T) -> Self {
        Case {
            name: name.into(),
            cases: vec![],
        }
    }

    // Describe a new condition to test.
    pub fn when<T>(mut self, name: T, f: WhenFn<C, E>) -> Self
    where
        T: Into<String>,
    {
        self.cases.push((Some(When::new(name, f)), None));
        self
    }

    /// Describe an action to run if its `then` block returns `false`.
    pub fn then<T>(mut self, name: T, f: ThenFn<C, E>) -> Self
    where
        T: Into<String>,
    {
        self.push_action(Box::new(Then::new(name, f)));
        self
    }

    /// Describe a group of then/when clauses to run if its `then` block returns `false`.
    pub fn then_case<T, F>(mut self, name: T, f: F) -> Self
    where
        T: Into<String>,
        F: FnOnce(Case<C, E>) -> Case<C, E>,
    {
        let case = f(Case::new(name));
        self.push_action(Box::new(case));
        self
    }

    fn push_action(&mut self, act: Box<Execute<C, E>>) {
        if let Some(case) = self.cases.last_mut() {
            case.1 = Some(act);
        }
    }

    /// Run all described statements.
    pub fn run(&self, ctx: &mut C) -> Result<bool, E> {
        self.exec(ctx)
    }
}

impl<C, E> Execute<C, E> for Case<C, E> {
    fn exec(&self, ctx: &mut C) -> Result<bool, E> {
        let mut changed = false;

        for (check, action) in self.cases.iter() {
            if check.is_none() {
                continue;
            }
            match check.as_ref().unwrap().check(ctx) {
                Ok(true) => {
                    if let Some(a) = action {
                        a.exec(ctx)?;
                    }
                    if !changed {
                        changed = true;
                    }
                }
                e @ Err(_) => return e,
                _ => {}
            }
        }

        Ok(changed)
    }
}

#[cfg(test)]
mod test {
    use super::Case;

    struct Ctx {
        x: i32,
    }
    impl Ctx {
        fn new(x: i32) -> Self {
            Ctx { x }
        }
    }

    #[test]
    fn cascade_when_error() {
        let mut c = Ctx::new(0);
        let res = Case::<Ctx, &str>::new("errored case")
            .when("check", |_| Err("when error"))
            .then("action", |ctx: &mut Ctx| {
                ctx.x = 5;
                Err("when error")
            })
            .run(&mut c);

        assert_eq!(res, Err("when error"));
        assert_eq!(c.x, 0);
    }

    #[test]
    fn cascade_then_error() {
        let mut c = Ctx::new(0);
        let res = Case::<Ctx, &str>::new("errored case")
            .when("check", |_| Ok(true))
            .then("check", |_| Err("then error"))
            .run(&mut c);

        assert_eq!(res, Err("then error"));
        assert_eq!(c.x, 0);
    }

    #[test]
    fn context_mutation() {
        let mut c = Ctx::new(5);
        let res = Case::<Ctx, &str>::new("errored case")
            .when("is 3", |ctx: &mut Ctx| Ok(ctx.x == 3))
            .then("multiply by 3", |ctx: &mut Ctx| {
                ctx.x *= 3;
                Ok(())
            })
            .when("is 5", |ctx: &mut Ctx| Ok(ctx.x == 5))
            .then("multiply by 5", |ctx: &mut Ctx| {
                ctx.x *= 5;
                Ok(())
            })
            .run(&mut c);

        assert_eq!(res, Ok(true));
        assert_eq!(c.x, 25);
    }

    #[test]
    fn nested_cases() {
        let mut c = Ctx::new(5);
        let res = Case::<Ctx, &str>::new("errored case")
            .when("is 3", |ctx: &mut Ctx| Ok(ctx.x == 3))
            .then_case("nested 3", |sub| {
                sub.when("check", |_ctx| Ok(true)).then("action", |ctx| {
                    ctx.x = 33;
                    Ok(())
                })
            })
            .when("is 5", |ctx: &mut Ctx| Ok(ctx.x == 5))
            .then_case("nested 5", |sub| {
                sub.when("check", |_ctx| Ok(true)).then("action", |ctx| {
                    ctx.x = 55;
                    Ok(())
                })
            })
            .run(&mut c);

        assert_eq!(res, Ok(true));
        assert_eq!(c.x, 55);
    }

    #[test]
    fn when_nothing_is_changed() {
        let mut c = Ctx::new(1);
        let res = Case::<Ctx, &str>::new("errored case")
            .when("is 3", |ctx: &mut Ctx| Ok(ctx.x == 3))
            .then("multiply by 3", |ctx: &mut Ctx| {
                ctx.x *= 3;
                Ok(())
            })
            .when("is 5", |ctx: &mut Ctx| Ok(ctx.x == 5))
            .then("multiply by 5", |ctx: &mut Ctx| {
                ctx.x *= 5;
                Ok(())
            })
            .run(&mut c);

        assert_eq!(res, Ok(false));
        assert_eq!(c.x, 1);
    }
}
