struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = User {
        active: true,
        name: String::from("lazy dev"),
        email: String::from("lazyDev@github.com"),
        sign_in_count: 100,
    };
    println!("{}", user.name);
    println!("{}", user.active);
    println!("{}", user.email);
    println!("{}", user.sign_in_count);
}

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}
