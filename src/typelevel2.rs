use std::marker::PhantomData;

trait Number {
    // fn to_int() -> u32;  // useful for debugging
}

struct Zero;
impl Number for Zero {}

struct Succ<N: Number> {
    _marker: PhantomData<N>,
}
impl<N: Number> Number for Succ<N> {}

// struct Add<M, N>
// where
//     M: Number,
//     N: Number,
// {
//     _marker: PhantomData<M, N>,
// }

trait Add<M>
where
    M: Number,
{
    type Output;
}

enum Number {
    Zero,
    Succ(Box<Number>),
    Add(Box<Number>, Box<Number>),
}

trait Equal<N> {}

impl Equal<Zero> for Zero {}
impl<M, N> Equal<Succ<N>> for Succ<M> where N: Equal<M> {}
