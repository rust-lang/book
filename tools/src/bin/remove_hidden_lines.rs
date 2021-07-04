use std::io;
use std::io::prelude::*;

fn main() {
    write_md(remove_hidden_lines(&read_md()));
}

fn read_md() -> String {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => buffer,
        Err(error) => panic!("{}", error),
    }
}

fn write_md(output: String) {
    write!(io::stdout(), "{}", output).unwrap();
}

fn remove_hidden_lines(input: &str) -> String {
    let mut resulting_lines = vec![];
    let mut within_codeblock = false;

    for line in input.lines() {
        if line.starts_with("```") {
            within_codeblock = !within_codeblock;
        }

        if !within_codeblock || (!line.starts_with("# ") && line != "#") {
            resulting_lines.push(line)
        }
    }

    resulting_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::remove_hidden_lines;

    #[test]
    fn hidden_line_in_code_block_is_removed() {
        let input = r#"
In this listing:

```
fn main() {
# secret
}
```

you can see that...
        "#;
        let output = remove_hidden_lines(input);

        let desired_output = r#"
In this listing:

```
fn main() {
}
```

you can see that...
        "#;

        assert_eq!(output, desired_output);
    }

    #[test]
    fn headings_arent_removed() {
        let input = r#"
# Heading 1
        "#;
        let output = remove_hidden_lines(input);

        let desired_output = r#"
# Heading 1
        "#;

        assert_eq!(output, desired_output);
    }
}
