fn main() {
    let mut users: Vec<User> = Vec::new();
    users.push(User { name: String::from("Stonoga") });
    users.push(User { name: String::from("Kowalski") });
    users.push(User { name: String::from("Nowak") });

    print_user(find_user("Nowak", &users));
    print_user(find_user("Burak", &users));
}


struct User {
    name: String,
}

struct NotFoundUser {
    name: String,
}

fn find_user<'a>(username: &str, users: &'a Vec<User>) -> Result<&'a User, NotFoundUser> {
    for user in users {
        if user.name == username {
            return Result::Ok(user);
        }
    };

    return Result::Err(NotFoundUser { name: String::from(username) });
}

fn print_user(user: Result<&User, NotFoundUser>) {
    match user {
        Ok(user) => println!("User found and its name is {}", user.name),
        Err(user) => println!("User {} not found", user.name)
    }
}