use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    username: String,
    role: String,
}

fn find_admin(json_data: &str) -> Option<String> {
    let users: Vec<User> = match serde_json::from_str(json_data) {
        Ok(users_vec) => users_vec,
        Err(_) => return None,
    };

    let admin_option = users.into_iter().find(|user| user.role == "admin");

    match admin_option {
        Some(admin) => serde_json::to_string_pretty(&admin).ok(),
        None => None,
    }
}

fn main() {
    let user_data = r#"
    [
        {"id": 1, "username": "user1", "role": "user"},
        {"id": 2, "username": "admin_user", "role": "admin"},
        {"id": 3, "username": "user2", "role": "user"}
    ]
    "#;

    match find_admin(user_data) {
        Some(admin_json) => println!("Found admin:\n{}", admin_json),
        None => println!("No admin user found."),
    }
}
