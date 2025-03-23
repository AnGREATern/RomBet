use uuid::Uuid;
use std::marker::PhantomData;

pub struct Id<T> {
    value: Uuid,
    marker: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    pub fn new() -> Self {
        let value = Uuid::now_v7();
        let marker = PhantomData;
        Self { value, marker }
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Id<T> {}