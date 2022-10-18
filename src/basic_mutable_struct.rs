struct User {
    username: String,
    email: String,
}

fn main() {
    let mut user = User{
        username: String::from("heyho"),
        email: String::from("sinan.gencogluu@gmail.com"),
    };

    println!("before:{}, {}", user.email, user.username);
    build_user(&mut user);
    println!("after:{}", user.email);
}

fn build_user(user:&mut User) {
    user.email = String::from("mutable reference");
}