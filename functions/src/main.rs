fn main() {
    println!("Hello, world!");
    print_phrase("print my argument");
    println!("{}", gcd(20, 4));
    println!("{}", multiple_return_values(true));

}

// rust users snake casing for functions
fn print_phrase(phrase: &str) {
   println!("{}", phrase);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

fn multiple_return_values(flag: bool) -> bool {
    if flag == true {
        true
    } else {
        false
    }
}
