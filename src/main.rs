fn main() {
    println!("Hello, world!");
}

fn snake_to_camel(s: &str) -> String {
    s.split('_').enumerate().map(|(i, part)| {
        if i > 0 {
            capitalize(part)
        } else {
            part.to_string()
        }
    }).collect::<String>()
}

fn capitalize(s: &str) -> String {
    let first = &s[0..1];
    let rest = &s[1..];
    let mut out = String::new();
    out.push_str(&first.to_uppercase().to_string());
    out.push_str(rest);
    out
}

fn camel_to_snake(s: &str) -> String {
    let mut prev_lowercase = false;
    let mut out = String::new();
    for c in s.chars() {
        if prev_lowercase && c.is_uppercase() {
            prev_lowercase = false;
            out.push('_');
            out.push_str(&c.to_lowercase().to_string());
        } else {
            prev_lowercase = true;
            out.push(c);
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

    #[test]
    fn test_pass_through_camel_to_snake() {
        test_camel_to_snake("test", "test");
    }

    fn test_camel_to_snake(input: &str, expected: &str) {
        let output = camel_to_snake(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_simple_camel_to_snake() {
        test_camel_to_snake("theTest", "the_test");
    }

    #[test]
    fn test_long_camel_to_snake_with() {
        test_camel_to_snake("thisIsTheTest", "this_is_the_test");
    }

    #[test]
    fn test_long_camel_to_snake_with_initial_capital() {
        test_camel_to_snake("ThisIsTheTest", "This_is_the_test");
    }
}
