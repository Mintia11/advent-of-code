pub trait StrExt {
    fn char(&self, idx: usize) -> char;
}

impl<T: AsRef<str>> StrExt for T {
    #[inline]
    fn char(&self, idx: usize) -> char {
        self.as_ref().chars().nth(idx).unwrap()
    }
}
