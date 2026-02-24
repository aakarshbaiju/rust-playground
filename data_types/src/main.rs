fn main() {
    // INTEGER TYPES
    // integer of 8 bits
    let x: i8 = 10;
    println!("{}", x);

    // integers can be signed or unsigned.
    // If value is always positive, then using unsigned is better.
    // let y: u8 = 10; // will show up as a warning since variable is unused
    let _y: u8 = 10; // _ prefix will remove warning

    // creates decimal variable with a value of 255
    let decimal = 02_55;

    // creates a hex variable with a value of 255 (represented in hexadecimal)
    let hex = 0xff;

    // creates a octal variable with a value of 255 (represented in octadecimal)
    let octal = 0o377;

    // creates a binary variable with a value of 255 (repesented in binary)
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    // gives the ascii value of 'A'
    let byte = b'A';
    println!("{}", byte);


    // FLOATING POINT TYPES
    let x = 2.0; // defaults to f64
    let y: f32 = 1.0;

    // BOOLEAN TYPES
    let t = true; // Compiler can automatically infer types based on assignment
    let f: bool = false;

    // CHARACTER TYPE
    let c  = 'c';
    println!("{}", c);

    // +, -, *, /, %
    let a = 10;
    let b = 4;
    let remainder = a % b;

    println!("{}", remainder)
}
