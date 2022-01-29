fn main() {
    println!("Hello, world!");
}

fn snake_to_camel(s: &str) -> String {
    let parts = snake_case_parts(s);
    join_camel_case(&parts)
}

fn snake_case_parts(s: &str) -> Vec<String> {
    s.split('_').map(|s| s.to_string()).collect()
}

fn join_camel_case(parts: &Vec<String>) -> String {
    parts.iter().enumerate().map(|(i, part)| {
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
    let parts = camel_case_parts(s);
    join_snake_case(&parts)
}

fn join_snake_case(parts: &Vec<String>) -> String {
    parts.join("_")
}

fn camel_case_parts(s: &str) -> Vec<String> {
    let mut prev_lowercase = false;
    let mut parts = Vec::new();
    let mut part = String::new();
    for c in s.chars() {
        if prev_lowercase && c.is_uppercase() {
            prev_lowercase = false;
            if parts.is_empty() {
                parts.push(part);
            } else {
                parts.push(part.to_lowercase());
            }
            part = String::new();
        } else {
            prev_lowercase = true;
        }
        part.push(c);
    }
    if !part.is_empty() {
        if parts.is_empty() {
            parts.push(part);
        } else {
            parts.push(part.to_lowercase());
        }
    }
    parts
}

fn kebab_to_camel(s: &str) -> String {
    let parts = s.split("-").map(|p| p.to_string()).collect::<Vec<String>>();
    join_camel_case(&parts)
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

    #[test]
    fn test_kebab_to_camel() {
        let input = "this-is-the-test";
        let output = kebab_to_camel(input);
        assert_eq!(output, "thisIsTheTest");
    }
}
