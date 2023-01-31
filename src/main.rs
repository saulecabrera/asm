use std::arch::global_asm;

fn main() {
    let simple_result = unsafe { simple() }; 
    assert!(simple_result == 42);

    let basic = unsafe { basic() };
    assert!(basic == 30);

    let basic_with_locals = unsafe { basic_with_locals() };
    assert!(basic_with_locals == 20);

    let basic_with_params = unsafe { basic_with_params(34, 12) };
    assert!(basic_with_params == 46);

    let mut out = 0;
    let _ = unsafe { simple_with_trampoline(0, 0, simple as *const (), &mut out as *mut i32) };
    assert!(out == 42);
}

global_asm!(r#"
    .global _simple_with_trampoline
    _simple_with_trampoline:
	stp x29, x30, [sp, #-0x10]!
        mov x26, x3
        blr x2
        mov x3, x26
        str w0, [x3]
	ldp x29, x30, [sp], #0x10
	ret 
"#);


global_asm!(r#"
    .global _simple
    _simple:
	stp x29, x30, [sp, #-0x10]!
	mov x29, sp
	mov x28, sp
	mov x16, #0x2a
	mov x0, x16
	ldp x29, x30, [sp], #0x10
	ret 
"#);

global_asm!(r#"
    .global _basic
    _basic:
	stp	x29, x30, [sp, #-0x10]!
	mov	x29, sp
	mov	x28, sp
	mov	x16, #0xa
	mov	w0, w16
	add	w0, w0, #0x14
	ldp	x29, x30, [sp], #0x10
	ret	
"#);

global_asm!(r#"
    .global _basic_with_locals
    _basic_with_locals:
	stp	x29, x30, [sp, #-0x10]!
	mov	x29, sp
	mov	x28, sp
	sub	sp, sp, #8
	mov	x28, sp
	mov	x16, #0
	stur	x16, [x28]
	mov	x16, #0x14
	mov	w0, w16
	stur	w0, [x28]
	ldur	w0, [x28]
	ldur	w1, [x28, #4]
	add	w1, w1, w0, uxtx
	mov	x0, x1
	add	sp, sp, #8
	mov	x28, sp
	ldp	x29, x30, [sp], #0x10
	ret	
"#);

global_asm!(r#"
    .global _basic_with_params
    _basic_with_params:
	stp	x29, x30, [sp, #-0x10]!
	mov	x29, sp
	mov	x28, sp
	sub	sp, sp, #8
	mov	x28, sp
	stur	w0, [x28, #4]
	stur	w1, [x28]
	ldur	w0, [x28]
	ldur	w1, [x28, #4]
	add	w1, w1, w0, uxtx
	mov	x0, x1
	add	sp, sp, #8
	mov	x28, sp
	ldp	x29, x30, [sp], #0x10
	ret	
"#);

extern "C" {
    fn simple()-> u8;
    fn basic() -> u8;
    fn basic_with_locals() -> u8;
    fn basic_with_params(a: i32, b: i32) -> i32;
    fn simple_with_trampoline(a: i32, b: i32, c: *const (), d: *mut i32);
}
