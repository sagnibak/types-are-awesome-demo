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

fn main() {
    let mut arr = vec![1, 2, 3];
    let ptr = &arr;
    Vec::drain(&mut arr, ..);
    println!("{:?}", ptr);

    /*
        Assembly output from printing
    main:
        .cfi_startproc
        pushq	%rax
        .cfi_def_cfa_offset 16
        movq	%rsi, %rcx
        movslq	%edi, %rdx
        leaq	_ZN7scratch4main17h283335f15886455aE(%rip), %rax
        movq	%rax, (%rsp)
        leaq	.L__unnamed_1(%rip), %rsi
        movq	%rsp, %rdi
        callq	*_ZN3std2rt19lang_start_internal17he05790f0cb2000dfE@GOTPCREL(%rip)
        popq	%rcx
        .cfi_def_cfa_offset 8
        retq

        Assembly output from dropping and forgetting
    main:
        .cfi_startproc
        pushq	%rax
        .cfi_def_cfa_offset 16
        movq	%rsi, %rcx
        movslq	%edi, %rdx
        leaq	_ZN7scratch4main17h283335f15886455aE(%rip), %rax
        movq	%rax, (%rsp)
        leaq	.L__unnamed_1(%rip), %rsi
        movq	%rsp, %rdi
        callq	*_ZN3std2rt19lang_start_internal17he05790f0cb2000dfE@GOTPCREL(%rip)
        popq	%rcx
        .cfi_def_cfa_offset 8
        retq

         */
}
