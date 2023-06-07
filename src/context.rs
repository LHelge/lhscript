pub struct Context {
    pub should_exit: bool,
}

impl Context {
    pub fn new() -> Self {
        Self { should_exit: false }
    }
}
