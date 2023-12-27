use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CursorPagedModel<T, ID> {
    pub list: Vec<T>,
    pub next: Option<ID>,
    pub count: usize,
}

impl<T, ID> CursorPagedModel<T, ID> {
    pub fn new(list: Vec<T>, next: Option<ID>) -> Self {
        let count = list.len();
        Self { list, next, count }
    }
}
