pub fn input_to_numbers(input: &str) -> Vec<u32> {
    input
        .split('\n')
        .map(|n| {
            n.parse()
                .unwrap_or_else(|e| panic!("could not parse '{}' to number, err: {:?}", n, e))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_to_numbers() {
        assert_eq!(
            input_to_numbers(
                "1
2
3
4"
            ),
            vec![1, 2, 3, 4]
        )
    }

    #[test]
    #[should_panic]
    fn test_input_to_numbers_invalid_input() {
        input_to_numbers(
            "1
hello
3
4",
        );
    }
}
