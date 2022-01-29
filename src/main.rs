fn main() {
    println!("Hello, world!");
}

fn snake_to_camel(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut was_underscore = false;
    for (i, c) in chars.iter().enumerate() {
        if *c == '_' && i + 1 < chars.len() {
            was_underscore = true;
        } else if was_underscore {
            if c.is_alphanumeric() {
                out.push_str(&c.to_uppercase().to_string());
            }
            was_underscore = false;
        } else {
            out.push(*c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_through_snake_to_camel() {
        test_snake_to_camel("test", "test");
    }

    fn test_snake_to_camel(input: &str, expected: &str) {
        let output = snake_to_camel(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_simple_snake_to_camel() {
        test_snake_to_camel("the_test", "theTest");
    }

    #[test]
    fn test_long_snake_to_camel() {
        test_snake_to_camel("This_is_the_test", "ThisIsTheTest");
    }

    #[test]
    fn test_single_letter_word_in_snake_to_camel() {
        test_snake_to_camel("This_is_a_test", "ThisIsATest");
    }
}
