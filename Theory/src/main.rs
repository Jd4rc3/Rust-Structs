fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.username);
    // Struct update syntax -----------------------------------------------------------------

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // print every field of user2
    println!("----------------------------update syntax ---------------------------------");
    println!("{}", user2.active);
    println!("{}", user2.username);
    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);
    //Destructuring
    let User {
        active,
        username,
        email,
        sign_in_count,
    } = user2;

    // println!("{}", user1.username)

    // Tuple struct -----------------------------------------------------------------
    println!("----------------------------Tuple struct ---------------------------------");
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    //Descructuring
    let Color(r, g, b) = black;

    println!("r: {}, g: {}, b: {}", r, g, b);

    // Unit-like struct -----------------------------------------------------------------
    println!("----------------------------Unit-like struct ---------------------------------");
    struct UnitLike;
    let unit_like = UnitLike;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}