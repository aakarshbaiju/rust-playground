// Named field struct
#[derive(Debug)] // macro used to allow :? debugs
struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

// Tuple like structs
struct Coordinates(i32, i32, i32);

// Unit structs
struct UnitStruct;

fn main() {
    let user1 = User{active: true, username: String::from("John"), sign_in_count: 0};
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);

    let user2 = build_user(String::from("Jane"));
    println!("{:?}", user2);

    let coords = Coordinates(1,2,3);
    println!("{} {} {}", coords.0, coords.1, coords.2);

    // 1..5, .. Range{start:1, end: 5}
}

fn build_user(username: String) -> User {
    User {
        username, // same as username: username
        active: true,
        sign_in_count: 1,
    }
}
