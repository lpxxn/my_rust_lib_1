pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    #[test]
    fn add() {
        assert_eq!(crate::add(2, 2), 4);
    }
}

