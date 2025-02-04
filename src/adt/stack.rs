pub trait Stack<T> {
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
    fn size(&self) -> usize;
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<T>;
}