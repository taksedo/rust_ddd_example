use std::sync::{Arc, Mutex};

/// `Arc<Mutex<T>>` alias type
pub type AM<T> = Arc<Mutex<T>>;

/// A Wrapper for `Arc<Mutex<T>>`
#[allow(dead_code)]
pub struct AMW<T>(Arc<Mutex<T>>);

impl<T> AMW<T> {
    /// Creates a new `Arc<Mutex<T>>` with the given `value`.
    ///
    /// # Examples
    ///
    ///```
    /// # use std::sync::{Arc, Mutex};
    /// # use types::base::AMW;
    ///
    /// let arc_mutex: Arc<Mutex<i32>> = AMW::new(10);
    /// ```
    ///
    pub fn new(value: T) -> Arc<Mutex<T>> {
        Arc::new(Mutex::new(value))
    }
}
