use std::sync::{Arc, Mutex, MutexGuard};

/// `Arc<Mutex<T>>` alias type
pub type AM<T> = Arc<Mutex<T>>;
pub trait ArcMutexTrait<T> {
    fn new_am(t: T) -> AM<T>;
    fn lock_un(&self) -> MutexGuard<T>;
}
impl<T> ArcMutexTrait<T> for AM<T> {
    fn new_am(t: T) -> AM<T> {
        Arc::new(Mutex::new(t))
    }
    fn lock_un(&self) -> MutexGuard<T> {
        self.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn test_am_is_arc_mutex() {
        let am = AM::new_am(10);
        assert_eq!(am.clone().type_id(), TypeId::of::<Arc<Mutex<i32>>>());
    }
}
