use regex::Regex;

pub fn extract_backtick_text(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_text = String::new();
    let mut inside_backticks = false;
    let mut backtick_count = 0;

    for c in input.chars() {
        match c {
            '`' => {
                if inside_backticks {
                    backtick_count -= 1;
                    if backtick_count == 0 {
                        if !current_text.is_empty() {
                            result.push(current_text.clone());
                        }
                        current_text.clear();
                        inside_backticks = false;
                    } else {
                        current_text.push(c);
                    }
                } else {
                    inside_backticks = true;
                    backtick_count += 1;
                }
            }
            _ if inside_backticks => {
                current_text.push(c);
            }
            _ => {}
        }
    }

    if inside_backticks {
        vec![]
    } else {
        result
    }
}

// Test cases
#[test]
fn test_backtick_extraction() {
    let cases = vec![
        ("Hello `world`", vec!["world"]),
        ("No backticks here", vec![]),
        ("`one` `two` `three`", vec!["one", "two", "three"]),
        ("Empty `` backticks", vec![]),
        ("Nested ```nested``` backticks", vec!["nested"]),
    ];

    for (input, expected) in cases {
        assert_eq!(
            extract_backtick_text(input),
            expected
        );
    }
}


// Test error case
#[test]
fn test_unmatched_backtick() {
    let result = extract_backtick_text("Unmatched `backtick");
    assert!(result.is_empty());
}


pub fn extract_error_blocks(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let error_pattern = Regex::new(r"error\[E\d{4}\]").unwrap();
    let mut remaining = input;

    while let Some(error_match) = error_pattern.find(remaining) {
        let start_pos = error_match.start();
        let error_section = &remaining[start_pos..];

        // Get lines starting from error
        let mut lines = error_section.lines();
        let mut current_block = Vec::new();

        // Always include first line (error message)
        if let Some(first_line) = lines.next() {
            current_block.push(first_line.to_string());

            // Include second line (file location)
            if let Some(second_line) = lines.next() {
                current_block.push(second_line.to_string());

                // Include subsequent lines with '|'
                for line in lines.take_while(|line| line.contains('|')) {
                    current_block.push(line.to_string());
                }
            }
        }

        if !current_block.is_empty() {
            result.push(current_block.join("\n"));
        }

        // Move past this error block
        remaining = &error_section[1..];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_error_blocks() {
        let input = r#"
error[E0596]: cannot borrow `vec` as mutable, as it is not declared as mutable
 --> <anon>:6:8
  |
6 |     f2(&mut vec);
  |        ^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
5 |     let mut vec: Vec<String> = vec![String::from("support"), String::from("local area network")];
  |         +++

error[E0382]: borrow of moved value: `vec`
 --> <anon>:8:27
  |
5 |     let vec: Vec<String> = vec![String::from("support"), String::from("local area network")];
  |         --- move occurs because `vec` has type `Vec<String>`, which does not implement the `Copy` trait
6 |     f2(&mut vec);
7 |     f1(vec);
  |        --- value moved here
8 |     let mut s: &String = &vec[0];
  |                           ^^^ value borrowed here after move
  |
note: consider changing this parameter type in function `f1` to borrow instead if owning the value isn't necessary
 --> <anon>:1:13
  |
1 | fn f1(arg1: Vec<String>) {}
  |    --       ^^^^^^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
help: consider cloning the value if the performance cost is acceptable
  |
7 |     f1(vec.clone());
  |           ++++++++

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0382, E0596.
For more information about an error, try `rustc --explain E0382`.
"#;

        let blocks = extract_error_blocks(input);
        assert_eq!(blocks.len(), 2);
        println!("{:?}", blocks);
        assert!(blocks[0].contains("E0596"));
        assert!(blocks[1].contains("E0382"));
    }
}
