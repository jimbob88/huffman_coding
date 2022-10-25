use std::collections::HashMap;

pub(crate) fn count_chars(input: &'static str) -> HashMap<char, i32> {
    // Source: https://stackoverflow.com/questions/64178272/what-is-the-idiomatic-rust-way-to-build-a-hashmap-of-character-counts
    // WARNING: There is no order to this HashMap
    let mut counts: HashMap<char, i32> = HashMap::new();

    for character in input.chars() {
        *counts.entry(character).or_insert(0) += 1;
    }

    counts
}

pub(crate) fn ordered_count_chars(input: &'static str) -> Vec<(char, i32)> {
    let counts = count_chars(input);
    let mut count_array: Vec<(char, i32)> = counts.into_iter().collect();
    count_array.sort_unstable_by_key(|t| (t.1, t.0));

    count_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_chars() {
        let test_string = "hello_world";
        let chars = count_chars(test_string);

        assert_eq!(*chars.get(&'h').unwrap(), 1);
        assert_eq!(*chars.get(&'e').unwrap(), 1);
        assert_eq!(*chars.get(&'l').unwrap(), 3);
        assert_eq!(*chars.get(&'o').unwrap(), 2);
        assert_eq!(*chars.get(&'_').unwrap(), 1);
        assert_eq!(*chars.get(&'w').unwrap(), 1);
        assert_eq!(*chars.get(&'r').unwrap(), 1);
        assert_eq!(*chars.get(&'d').unwrap(), 1);
    }

    #[test]
    fn test_ordered_count_chars_has_constant_order() {
        assert_eq!(ordered_count_chars("hello world"), ordered_count_chars("hello world"));
    }

    #[test]
    fn test_ordered_count_chars_has_constant_order_with_unordered_string() {
        assert_eq!(ordered_count_chars("hello wrold"), ordered_count_chars("hello world"));
    }

    #[cfg(feature="torture")]
    #[test]
    fn test_torture_constant_char_order() {
        // cargo test --features="torture"
        for _ in 0..1_000 {
            test_ordered_count_chars_has_constant_order()
        }
    }

}