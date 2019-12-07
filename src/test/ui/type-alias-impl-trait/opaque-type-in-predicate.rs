// run-pass

// Regression test for issue #65918 - checks that we do not
// ICE when attempting to normalize a predicate containting
// opaque types

#![feature(type_alias_impl_trait)]

use std::marker::PhantomData;

/* copied Index and TryFrom for convinience (and simplicity) */
trait MyIndex<T> {
    type O;
    fn my_index(self) -> Self::O;
}
trait MyFrom<T>: Sized {
    type Error;
    fn my_from(value: T) -> Result<Self, Self::Error>;
}

/* MCVE starts here */
trait F {}
impl F for () {}
type DummyT<T> = impl F;
fn _dummy_t<T>() -> DummyT<T> {}

struct Phantom1<T>(PhantomData<T>);
struct Phantom2<T>(PhantomData<T>);
struct Scope<T>(Phantom2<DummyT<T>>);

impl<T> Scope<T> {
    fn new() -> Self {
        Scope(Phantom2(PhantomData))
    }
}

impl<T> MyFrom<Phantom2<T>> for Phantom1<T> {
    type Error = ();
    fn my_from(_: Phantom2<T>) -> Result<Self, Self::Error> {
        Ok(Phantom1(PhantomData))
    }
}

impl<T: MyFrom<Phantom2<DummyT<U>>>, U> MyIndex<Phantom1<T>> for Scope<U> {
    type O = T;
    fn my_index(self) -> Self::O {
        MyFrom::my_from(self.0).ok().unwrap()
    }
}

fn main() {
    let _pos: Phantom1<DummyT<()>> = Scope::new().my_index();
}
