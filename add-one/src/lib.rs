pub fn add_one(x: isize) -> isize {
    x + 1
}

#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn test_add_one() {
        let result = add_one(1);
        assert_eq!(result, 2);
    }
}
