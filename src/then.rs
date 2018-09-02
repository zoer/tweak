use super::{Execute, Result, ThenFn};

pub(crate) struct Then<C, E> {
    #[allow(dead_code)]
    name: String,
    f: ThenFn<C, E>,
}

impl<C, E> Then<C, E> {
    pub fn new<T: Into<String>>(name: T, f: ThenFn<C, E>) -> Self {
        Self {
            name: name.into(),
            f: f,
        }
    }
}

impl<C, E> Execute<C, E> for Then<C, E> {
    fn exec(&self, ctx: &mut C) -> Result<bool, E> {
        match (self.f)(ctx) {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}
