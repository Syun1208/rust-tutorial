
fn main() {
    // Integer: i8, i16, i32, i64, isize, usize, u8, u16, u32, u64
    let _ai1:i8 = 16;
    let _ai2:i16 = 16;
    let _ai3:i32 = 16;
    let _ai4:i64 = 16;
    let _ai5:isize = 16;
    let _bi1:u8 = 24;
    let _bi2:u16 = 24;
    let _bi3:u32 = 24;
    let _bi4:u64 = 24;
    let _bi5:usize = 24;

    // Floating-point: f32, f64
    let _af1:f32 = 6_069.5;
    let _af2:f64 = 7_498.8;

    // Booleans
    let _is_prime:bool = true;

    // Characters
    let _emoji:char = '😁';

    // String
    let owner:&str = "anh Long dep trai sieu cap vu tru";

    // Compiler: `rustc path_to_your_rust_file` or `cargo run`
    println!("{}", owner);
}
