struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!(
        "[{};{};{};{}]",
        user1.username,
        user1.email,
        user1.active,
        user1.sign_in_count
    );
    println!(
        "[{};{};{};{}]",
        user2.username,
        user2.email,
        user2.active,
        user2.sign_in_count
    );
}
