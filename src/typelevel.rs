use std::marker::PhantomData;

trait Number {
    fn to_int() -> u32;
}
trait Add<T: Number> {
    type Output: Number;
}
trait Multiply {
    type Output;
}

struct Zero;
impl Number for Zero {
    fn to_int() -> u32 {
        0
    }
}
impl<Rhs> Add<Rhs> for Zero
where
    Rhs: Number,
{
    type Output = Rhs;
}

struct Succ<N: Number> {
    _marker: PhantomData<N>,
}
impl<N: Number> Number for Succ<N> {
    fn to_int() -> u32 {
        1 + N::to_int()
    }
}
impl<Lhs, Rhs> Add<Rhs> for Succ<Lhs>
where
    Lhs: Number + Add<Rhs>,
    Rhs: Number,
{
    type Output = Succ<<Lhs as Add<Rhs>>::Output>;
}

struct EqualityConstraint {}
trait Equal<Rhs, Lhs> {}
impl Equal<Zero, Zero> for Zero {}
impl<N> Equal<Succ<N>, Succ<N>> for Succ<N> where N: Number {}

struct List<T, N: Number> {
    list: Vec<T>,
    _marker: PhantomData<N>,
}

impl<T, N: Number> List<T, N> {
    fn nil() -> List<T, N>
    where
        EqualityConstraint: Equal<N, Zero>,
    {
        Self {
            list: Vec::new(),
            _marker: PhantomData,
        }
    }
}
