pub fn main() {
    /* Integers
       Length   Signed   Unsigned
       8-bit    i8       u8
       16-bit   i16      u16
       32-bit   i32      u32
       64-bit   i64      u64
       128-bit  i128     u128
       arch     isize    usize
    */

    let small_number: u8 = 255; // unsigned 8-bit (0-255)
    let big_number: u128 = 123456789012345678; // unsigned 128-bit (2**128)
    let small_number2: i8 = -127; // signed 8-bit (-128 - 127)
    let big_number2: i128 = -123456789012345678; // signed 128-bit

    println!("{} {}", small_number, big_number);
    println!("{} {}", small_number2, big_number2);

    /*
        Numeral    System    Description           Example
        Decimal    Base-10   common form           98_222
        Hex        Base-16   prefixed with 0x      0xFF
        Octal      Base-8    prefixed with 0o      0o77
        Binary     Base-2    prefixed with 0b      0b1111_0000
        Byte       u8(only)  ASCII characters,     b'A'
                             prefixed with b
    */

    let decimal = 98_222;
    let hex = 0xFF;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!(
        "decimal:{} hex:{} octal:{} binary:{} byte:{}",
        decimal, hex, octal, binary, byte
    );
}
