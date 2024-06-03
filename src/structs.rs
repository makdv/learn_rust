struct User {
    active: bool,
    username: String,
    email: String,
}
struct Color(i32, i32, i32);

pub fn run() {
    let mut user = User {
        active: true,
        username: String::from("root"),
        email: String::from("test@gmail.com"),
    };
    let user2 = User {
        email: String::from("test2@gmail.com"),
        ..user
    };
    let tuple_struct = Color(1, 2, 3);

    // let _username = user.username; // moved
    let ref username = user2.username; // not moved;
                                       // let username = &user2.username; // not moved;
                                       // let username = user2.username.clone(); // not moved;

    println!("username: {} {}", username, user2.username);
}
