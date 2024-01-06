use std::marker::PhantomData;

#[derive(Debug)]
pub struct Repo<T> {
    n: PhantomData<T>,
}
