fn main() {
    let mut users: Vec<User> = Vec::new();
    users.push(User { name: String::from("Stonoga") });
    users.push(User { name: String::from("Kowalski")});
    users.push(User { name: String::from("Nowak")});

    print_user(find_user("Nowak", &users));
    print_user(find_user("Burak", &users));
}


struct User {
    name: String,
}

fn find_user<'a>(username: &str, users: &'a Vec<User>) -> Option<&'a User> {
    for user in users {
        if user.name == username {
            return Option::Some(user);
        }
    };

    return None;
}

fn print_user(user: Option<&User>) {
    match user {
        Some(user) => println!("User found and its name is {}", user.name),
        None => println!("User not found")
    }
}