struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1=User{
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2=User{
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user2.email=String::from("anotheremail@example.com");

    let user3=User{
        active: user1.active,
        username: user1.username, // moves user1's username into user3's username
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    let _user4=User{
        email: String::from("another@example.com"),
        ..user3
    };
}

fn _build_user(email: String, username: String) -> User {
    User{
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn _build_user_short(email: String, username: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
