use std::marker::PhantomData;

trait Number {}

struct Zero;
impl Number for Zero {}

struct Succ<N: Number> {
    _marker: PhantomData<N>,
}
impl<N: Number> Number for Succ<N> {}

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
