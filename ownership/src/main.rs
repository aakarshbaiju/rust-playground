fn main() {
    // let var = 1; // this is created on the stack
    // let mut s = "hello".to_string(); // created on the heap
    // s.push_str(", world"); // heap allows to grow
    assignment();
    return;

    let x = vec!["tylers".to_string()];
    let y = x;

    // This won't work since value of x is now moved to y
    println!("{:?}", y);

    // This creates a deep copy of y. Cloning is an expensive operation
    let z = y.clone();
    println!("{:?}", z);

    let s = String::from("takes");
    take_ownership(s);

    // This won't work as ownership is passed to function
    // println!("{}", s);
    let val = 1;
    make_copy(val);
    println!("{}", val);

    let given = give_ownership();
    println!("{}", given);

    let str3: String = take_and_give_ownership(given);

    // This won't work as ownership is passed to function
    // println!("{}", given);

    // New owner
    println!("{}", str3);

    if (true) {
        let str4 = str3;
    } else {
        let str5 = str3;
    }

    // This wont' work due to move
    // println!("{}", str3);

    let mut str1 = String::from("home");
    let mut str2: String;

    // loop {
    // ownership changes during previous iteration
    // str2 = str1;
    // }

    // REFERENCES
    // Shared Reference: lets you read but not modify, you can have a lot of shared references

    let mut s = String::from("Hello");
    change_string(&mut s);
    println!("{}", s);
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world!")
}

fn take_ownership(s: String) {
    let strin = s;
    println!("{}", strin);
}

fn give_ownership() -> String {
    "given".to_string()
}

fn take_and_give_ownership(s: String) -> String {
    s
}

fn make_copy(one: i32) {
    let val = one;
    println!("{}", val);
}

// var would be dropped after main execution
// s is also dropped
