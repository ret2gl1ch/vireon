pub fn greet() -> &'static str {
    "Hello from vireon-core"
}

#[cfg(test)]
mod test {

    use super::*;

    fn test_greet() {
        assert_eq!(greet(), "Hello from vireon-core");
    }
}
