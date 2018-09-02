use super::{Result, WhenFn};

pub(crate) struct When<C, E> {
    #[allow(dead_code)]
    name: String,
    f: WhenFn<C, E>,
}

impl<C, E> When<C, E> {
    pub fn new(name: impl Into<String>, f: WhenFn<C, E>) -> Self {
        Self {
            name: name.into(),
            f: f,
        }
    }

    pub fn check(&self, ctx: &mut C) -> Result<bool, E> {
        (self.f)(ctx)
    }
}
