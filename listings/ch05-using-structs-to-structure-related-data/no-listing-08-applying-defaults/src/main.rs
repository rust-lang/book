struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// ANCHOR: here
fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        ..
    }
}
// ANCHOR_END: here

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
