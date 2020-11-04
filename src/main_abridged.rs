#![allow(unused_variables)]

use std::collections::VecDeque;
use std::marker::PhantomData;

trait Number {
    fn to_int() -> i32;
}

#[derive(Debug, Clone)]
struct Zero;
impl Number for Zero {
    fn to_int() -> i32 {
        0
    }
}

#[derive(Debug, Clone)]
struct Succ<N: Number> {
    _marker: PhantomData<N>,
}
impl<N: Number> Number for Succ<N> {
    fn to_int() -> i32 {
        1 + N::to_int()
    }
}

trait Equal<N> {}
impl Equal<Zero> for Zero {}
impl<M, N> Equal<Succ<N>> for Succ<M>
where
    M: Number + Equal<N>,
    N: Number,
{
}

trait IsGreaterThan<N: Number> {}
impl<N> IsGreaterThan<Zero> for Succ<N> where N: Number {}
impl<M, N> IsGreaterThan<Succ<N>> for Succ<M>
where
    M: Number + IsGreaterThan<N>,
    N: Number,
{
}

trait Add<N: Number> {
    type Output: Number;
}

impl<N> Add<N> for Zero
where
    N: Number,
{
    type Output = N;
}

impl<M, N> Add<N> for Succ<M>
where
    M: Number + Add<N>,
    N: Number,
{
    type Output = Succ<<M as Add<N>>::Output>;
}

#[derive(Debug, Clone)]
struct List<T, N: Number> {
    vals: VecDeque<T>,
    _marker: PhantomData<N>,
}

fn nil<T>() -> List<T, Zero> {
    List {
        vals: VecDeque::new(),
        _marker: PhantomData,
    }
}

fn cons<T, N>(val: T, mut rest: List<T, N>) -> List<T, Succ<N>>
where
    N: Number,
{
    rest.vals.push_front(val);
    List {
        vals: rest.vals,
        _marker: PhantomData,
    }
}

fn zip<T, U, L, R>(left: List<T, L>, right: List<U, R>) -> List<(T, U), L>
where
    L: Number + Equal<R>,
    R: Number,
{
    List {
        vals: left.vals.into_iter().zip(right.vals.into_iter()).collect(),
        _marker: PhantomData,
    }
}

fn main() {
    let a = cons(1, cons(2, cons(3, nil())));
    let hello = cons("h", cons("e", cons("l", cons("l", cons("o", nil())))));
    let world = cons("w", cons("o", cons("r", cons("l", cons("d", nil())))));
    // let a_hello = zip(a, hello); // does not compile!
    let hello_world = zip(hello, world);
    println!("{:?}", hello_world.vals);
}
