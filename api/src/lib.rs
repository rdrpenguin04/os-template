#![no_std]

pub struct Console;

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let data = s.as_bytes();
        unsafe { print_bytes(data.as_ptr(), data.len()); }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        write!($crate::Console, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::Console.write_str("\n").unwrap();
    }};
    ($($arg:tt)*) => {{
        use ::core::fmt::Write;
        writeln!($crate::Console, $($arg)*).unwrap();
    }};
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("\x1b[31;1merror: the OS encountered a panic. {info}");
    hcf();
}

pub fn hcf() -> ! {
    unsafe { hcf_real(); }
}

unsafe extern "C" {
    fn hcf_real() -> !;

    fn print_bytes(data: *const u8, len: usize);
}
