use std::collections::HashMap;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    age: u32,
}

struct UserManager {
    users: HashMap<u32, User>,
}

impl UserManager {
    fn new() -> Self {
        UserManager {
            users: HashMap::new(),
        }
    }

    fn add_user(&mut self, id: u32, name: String, age: u32) {
        let user = User { id, name, age };
        self.users.insert(id, user);
    }

    fn get_user(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    fn get_all_users(&self) -> Vec<&User> {
        let mut user_list = Vec::new();
        for (_, user) in &self.users {
            user_list.push(user);
        }
        user_list
    }

    fn average_age(&self) -> f32 {
        let total_age: u32 = self.users.values().map(|user| user.age).sum();
        let count = self.users.len() as f32;
        total_age as f32 / count
    }
}

fn main() {
    let mut user_manager = UserManager::new();
    user_manager.add_user(1, String::from("Alice"), 30);
    user_manager.add_user(2, String::from("Bob"), 25);
    user_manager.add_user(3, String::from("Charlie"), 35);

    println!("All users: {:?}", user_manager.get_all_users());
    println!("Average age: {}", user_manager.average_age());
}
