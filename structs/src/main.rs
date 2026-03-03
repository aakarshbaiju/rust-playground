// Named field struct
#[derive(Debug)] // macro used to allow :? debugs
struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

// Tuple like structs
struct Coordinates(i32, i32, i32);

// Unit structs: mostly used alongside traits
struct UnitStruct;

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn change_width(&mut self, new_width: u32) {
        // self is a reference so it cannot be written to if not marked mutable
        self.width = new_width;
    }
}

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


    // variable should also be mutable for change_width to work
    let mut sq = Square {width: 5, height: 5};

    println!("{}", sq.area());
    println!("{}", sq.get_width());
    sq.change_width(10);
    println!("{}", sq.area());
    println!("{}", sq.get_width());
}

fn build_user(username: String) -> User {
    User {
        username, // same as username: username
        active: true,
        sign_in_count: 1,
    }
}
