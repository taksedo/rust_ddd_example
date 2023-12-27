use std::sync::{Arc, Mutex};

pub type AM<T> = Arc<Mutex<T>>;
