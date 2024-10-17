#[derive(Debug)]
pub enum CGroupLimitValue<T> {
    Max,
    Value(T),
}
