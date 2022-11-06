pub struct Defer<F: FnMut()> {
    f: F,
}

impl<F: FnMut()> Defer<F> {
    pub fn new(function: F) -> Self {
        Self {
            f: function,
        }
    }
}

impl<F: FnMut()> Drop for Defer<F> {
    fn drop(&mut self) {
        (self.f)();
    }
}

#[macro_export]
macro_rules! defer {
    ($($s:stmt);*) => {
        ::gensym::gensym!{ $crate::defer!($($s);*) }
    };
    ($id:ident, $($s:stmt);*) => {
        let $id = $crate::Defer::new(||{ $($s)* });
    };
}
