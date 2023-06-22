fn main() {
    // Integers

    let a = 98_222;
    let b = 0xff;
    let c = 0o77;
    let d = 0b1111_0000;
    let e = b'A';

    let f: u8 = 255;

    /* Integer overflow
       let f: u8 = 256;

       In debug builds, Rust will panic
       In release builds, Rust will perform two's complement wrapping
       meaning values greater than the maximum will wrap to the minimum values
       So let f: u8 = 256; == let f: u8 = 0;
    */

    // Floating-Point Numbers

    let g = 2.0;
    let f: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let division = 56.7 / 32.2;
    let remainder = 43 % 5;

    // Booleans

    let t = true;
    let f: bool = false;

    // Character

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound Types
    let tup = ("QQQ", 400);
    let (ticker, price) = tup;
    let ticker = tup.0;

    let http_error_codes = [200, 404, 500];
    let not_found = http_error_codes[1];

    let byte = [0; 8];
}
