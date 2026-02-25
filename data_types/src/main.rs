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

    println!("{}", remainder);

    compound_datatypes();
}

fn compound_datatypes() {
    // TUPLE (fixed length items with different types for each item)
    let tup = (500, "hi", true);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    let (mut x, y, z) = tup;
    println!("{} {} {}", x, y, z);
    x = 2;

    // ARRAYS (items with fixed datatypes)
    let array = [1,2,3];
    println!("{}, {}, {}", array[0], array[1], array[2]);

    // let array2: [i32; 3] = [4,5,6];
    // This won't work as array is defined as mutable
    // array2[0] = 10;

    let mut array2: [i32; 3] = [4,5,6];
    println!("{}, {}, {}", array2[0], array2[1], array2[2]);
    array2[0] = 10;
    println!("{}, {}, {}", array2[0], array2[1], array2[2]);

    // Results in index out of bounds error
    // println!("{}", array2[3])

    // VECTORS -> resizeable arrays
    let mut nums = vec![1,2,3];
    nums.push(4); // will only work if nums is mutable

    // Printing like this shows default formatter error, instead use debug formatter notation
    // println!("{}", nums);

    println!("{:?}", nums);
    let lastVal = nums.pop();
    println!("{:?}", lastVal); // This will be an Option type and print as Some(4) because pop might not necessarily have a result if empty
    println!("{:?}", nums);


    let mut vec = Vec::new(); // constructor implementation opposed to doing vec! macro
    vec.push("Test");
    vec.push("String");
    println!("{:?}", vec);
    vec.reverse();
    println!("{:?}", vec);

    let mut vec2 = Vec::<i32>::with_capacity(2);
    println!("{:?}", vec2);
    println!("{}", vec2.capacity());

    // Creating a vector using iterator
    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    // SLICES: region of an array or vector that can be of any length and cannot be directly stored in a variable
    let sv: &[i32] = &v[2..4]; // sv is a pointer to the vector (fat pointer)
    println!("{:?}", sv);


}
