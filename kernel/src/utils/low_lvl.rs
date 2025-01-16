use core::arch::asm;

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;

    asm!(
        "in al, dx",
        out("al") ret,
        in("dx") port,
        options(nomem, nostack),
    );

    ret
}

#[inline(always)]
pub unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack),
    );
}