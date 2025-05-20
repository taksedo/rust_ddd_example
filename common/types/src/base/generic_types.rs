use std::{cell::RefCell, rc::Rc, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;

/// `Arc<tokio::sync::Mutex<T>>` alias type
pub type AM<T> = Arc<Mutex<T>>;

#[async_trait]
pub trait AMTrait<T: ?Sized> {
    fn new_am(t: T) -> AM<T>
    where
        T: Sized;
}

#[async_trait]
impl<T: ?Sized + Send> AMTrait<T> for AM<T> {
    fn new_am(t: T) -> AM<T>
    where
        T: Sized,
    {
        Arc::new(Mutex::new(t))
    }
}

/// `Rc<RefCell<T>>` alias type
pub type RCell<T> = Rc<RefCell<T>>;

pub trait RcRefCellTrait<T> {
    fn new_rc(t: T) -> RCell<T>;
}

impl<T> RcRefCellTrait<T> for RCell<T> {
    fn new_rc(t: T) -> RCell<T> {
        Rc::new(RefCell::new(t))
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
