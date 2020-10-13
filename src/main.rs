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
