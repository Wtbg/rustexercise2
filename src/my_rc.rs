use std::ops::Deref;
use std::cell::RefCell;
pub struct MyRc<T: Clone> {
    pub shared: RefCell<T>,
    pub count: RefCell<usize>,
}

impl<T: Clone> MyRc<T> {
    #[allow(dead_code)]
    pub fn new(value: T) -> MyRc<T> {
        MyRc {
            shared: RefCell::new(value),
            count: RefCell::new(1),
        }
    }

    #[allow(dead_code)]
    pub fn strong_count(&self) -> usize {
        *self.count.borrow()
    }

    #[allow(dead_code)]
    pub fn clone(&self) -> MyRc<T> {
        *self.count.borrow_mut() += 1;
        MyRc {
            shared: self.shared.clone(),
            count: self.count.clone(),
        }
    }
}

impl<T: Clone> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.shared.as_ptr() }
    }
}
