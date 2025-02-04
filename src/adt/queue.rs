pub trait Queue<T> {
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
    fn size(&self) -> usize;
    fn offer(&mut self, item: T);
    fn poll(&mut self) -> Option<T>;
    fn peek(&self) -> Option<T>;
}