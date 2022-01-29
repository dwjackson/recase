use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum CaseType {
    Camel,
    Snake,
    Kebab,
}

#[derive(Debug, PartialEq)]
enum CaseTypeParseError {
    BadCaseType,
}

impl FromStr for CaseType {
    type Err = CaseTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.to_lowercase();
        match normalized.as_str() {
            "camel" => Ok(CaseType::Camel),
            "snake" => Ok(CaseType::Snake),
            "kebab" => Ok(CaseType::Kebab),
            _ => Err(CaseTypeParseError::BadCaseType),
        }
    }
}

fn convert(input: &str, input_type: CaseType, output_type: CaseType) -> String {
    let make_parts = match input_type {
        CaseType::Camel => camel_case_parts,
        CaseType::Snake => snake_case_parts,
        CaseType::Kebab => kebab_case_parts,
    };
    let join_parts = match output_type {
        CaseType::Camel => join_camel_case,
        CaseType::Snake => join_snake_case,
        CaseType::Kebab => join_kebab_case,
    };
    let parts = make_parts(input);
    join_parts(&parts)
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

fn kebab_case_parts(s: &str) -> Vec<String> {
    s.split("-").map(|p| p.to_string()).collect()
}

fn join_kebab_case(parts: &Vec<String>) -> String {
    parts.join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_through_snake_to_camel() {
        test_snake_to_camel("test", "test");
    }

    fn test_snake_to_camel(input: &str, expected: &str) {
        let output = convert(input, CaseType::Snake, CaseType::Camel);
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
        let output = convert(input, CaseType::Camel, CaseType::Snake);
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
        let output = convert(input, CaseType::Kebab, CaseType::Camel);
        assert_eq!(output, "thisIsTheTest");
    }

    #[test]
    fn test_kebab_to_snake() {
        let input = "this-is-the-test";
        let output = convert(input, CaseType::Kebab, CaseType::Snake);
        assert_eq!(output, "this_is_the_test");
    }

    #[test]
    fn test_camel_to_kebab() {
        let input = "thisIsTheTest";
        let output = convert(input, CaseType::Camel, CaseType::Kebab);
        assert_eq!(output, "this-is-the-test");
    }

    #[test]
    fn test_parse_case_types() {
        let inputs = vec!["camel", "snake", "kebab"];
        let outputs = vec![CaseType::Camel, CaseType::Snake, CaseType::Kebab];
        for (i, input) in inputs.iter().enumerate() {
            let output = input.parse::<CaseType>().expect("Bad parse");
            assert_eq!(output, outputs[i]);
        }
    }

    #[test]
    fn test_parse_case_types_case_insensitive() {
        let inputs = vec!["camel", "SNAKE", "kEbaB"];
        let outputs = vec![CaseType::Camel, CaseType::Snake, CaseType::Kebab];
        for (i, input) in inputs.iter().enumerate() {
            let output = input.parse::<CaseType>().expect("Bad parse");
            assert_eq!(output, outputs[i]);
        }
    }

    #[test]
    fn test_bas_parse() {
        let output = "bad".parse::<CaseType>();
        match output {
            Ok(_) => panic!("Parse should fail"),
            Err(e) => assert_eq!(e, CaseTypeParseError::BadCaseType),
        }
    }
}
