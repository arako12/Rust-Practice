pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greet() -> &'static str {
    "Hello, test!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

     #[test]
    fn it_greets() {
        assert_eq!(greet(), "Hello, test!");
    }
}
