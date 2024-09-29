// e9 console
fn e9_write_byte(b: u8) {
    if b == b'\n' {
        e9_write_byte(b'\r');
    }
    unsafe {
        core::arch::asm!(r#"
        out 0e9h, al    
        "#, in("al") b);
    }
}

pub fn write_byte(b: u8) {
    e9_write_byte(b);
}
