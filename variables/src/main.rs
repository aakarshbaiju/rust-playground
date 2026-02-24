fn main() {
    let mut x = 5;

    // This is a macro (println!)
    println!("The value of x is {}", x);

    // This won't work as the variable is immutable by default
    // let y = 5
    // y = 6;

    x = 6;
    println!("The value of x is {}", x);

    // Constants are always maintained as all caps per convention
    const SECONDS: i8 = 60;
    println!("The value of seconds is {}", SECONDS)
}
