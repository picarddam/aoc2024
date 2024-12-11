use std::arch::asm;

/// Quotient and remainder in a single operation without 0 division check.
/// # Safety
/// UB when b == 0
pub unsafe fn asm_div_rem(a: u64, b: u64) -> (u64, u64) {
    let mut tmp: u64 = a;
    let mut remainder: u64 = 0;
    unsafe {
        asm!(
            "div {divisor}",
            inout("eax") tmp,
            inout("edx") remainder,
            divisor = in(reg) b,
            options(pure, nomem, nostack),
        );
    }
    (tmp, remainder)
}
