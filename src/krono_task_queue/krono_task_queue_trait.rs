use crate::threads::GreenThread;

pub trait KronoTaskQueueTraits {
    fn new() -> Self;
    fn push(&mut self, gthread: GreenThread);
    fn pop(&mut self) -> Option<GreenThread>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}