pub mod user;
pub use user::User;

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    #[test]
    fn add() {
        assert_eq!(crate::add(2, 2), 4);
    }
    #[test]
    fn user_name() {
        let u = crate::User::new_user(String::from("li"), 10);
        assert_eq!(u.name(), "li")
    }
}

