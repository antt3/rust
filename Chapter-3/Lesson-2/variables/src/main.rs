fn main() {
    // Integers
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)

    // Floating-Point Numbers
    let f = 2.0; // f64
    let g: f32 = 3.0; // f32

    // Addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // Remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f = false;

    // Character
    let lower_z = 'z';
    let upper_z = 'Z';
    let heart_eyed_cat = '😻';

    // Tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Arrays
    let arr = [1, 2, 3, 4, 5];
    let three = arr[2];

    let byte = [0; 8];
}
