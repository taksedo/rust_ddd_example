pub trait BusinessError {}

pub trait ToError<T> {
    fn to_error(self) -> T;
}
