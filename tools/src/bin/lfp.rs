// We have some long regex literals, so:
// ignore-tidy-linelength

use docopt::Docopt;
use serde::Deserialize;
use std::io::BufRead;
use std::{fs, io, path};

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let src_dir = &path::Path::new(&args.arg_src_dir);
    let found_errs = walkdir::WalkDir::new(src_dir)
        .min_depth(1)
        .into_iter()
        .map(|entry| match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("{:?}", err);
                std::process::exit(911)
            }
        })
        .map(|entry| {
            let path = entry.path();
            if is_file_of_interest(path) {
                let err_vec = lint_file(path);
                for err in &err_vec {
                    match *err {
                        LintingError::LineOfInterest(line_num, ref line) => {
                            eprintln!(
                                "{}:{}\t{}",
                                path.display(),
                                line_num,
                                line
                            )
                        }
                        LintingError::UnableToOpenFile => {
                            eprintln!("Unable to open {}.", path.display())
                        }
                    }
                }
                !err_vec.is_empty()
            } else {
                false
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .any(|result| *result);

    if found_errs {
        std::process::exit(1)
    } else {
        std::process::exit(0)
    }
}

const USAGE: &'static str = "
counter
Usage:
  lfp <src-dir>
  lfp (-h | --help)
Options:
  -h --help         Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_src_dir: String,
}

fn lint_file(path: &path::Path) -> Vec<LintingError> {
    match fs::File::open(path) {
        Ok(file) => lint_lines(io::BufReader::new(&file).lines()),
        Err(_) => vec![LintingError::UnableToOpenFile],
    }
}

fn lint_lines<I>(lines: I) -> Vec<LintingError>
where
    I: Iterator<Item = io::Result<String>>,
{
    lines
        .enumerate()
        .map(|(line_num, line)| {
            let raw_line = line.unwrap();
            if is_line_of_interest(&raw_line) {
                Err(LintingError::LineOfInterest(line_num, raw_line))
            } else {
                Ok(())
            }
        })
        .filter(|result| result.is_err())
        .map(|result| result.unwrap_err())
        .collect()
}

fn is_file_of_interest(path: &path::Path) -> bool {
    path.extension().map_or(false, |ext| ext == "md")
}

fn is_line_of_interest(line: &str) -> bool {
    !line
        .split_whitespace()
        .filter(|sub_string| {
            sub_string.contains("file://")
                && !sub_string.contains("file:///projects/")
        })
        .collect::<Vec<_>>()
        .is_empty()
}

#[derive(Debug)]
enum LintingError {
    UnableToOpenFile,
    LineOfInterest(usize, String),
}

#[cfg(test)]
mod tests {

    use std::path;

    #[test]
    fn lint_file_returns_a_vec_with_errs_when_lines_of_interest_are_found() {
        let string = r#"
        $ cargo run
               Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
                 Running `target/guessing_game`
            Guess the number!
            The secret number is: 61
            Please input your guess.
            10
            You guessed: 10
            Too small!
            Please input your guess.
            99
            You guessed: 99
            Too big!
            Please input your guess.
            foo
            Please input your guess.
            61
            You guessed: 61
            You win!
            $ cargo run
               Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
                 Running `target/debug/guessing_game`
            Guess the number!
            The secret number is: 7
            Please input your guess.
            4
            You guessed: 4
            $ cargo run
                 Running `target/debug/guessing_game`
            Guess the number!
            The secret number is: 83
            Please input your guess.
            5
            $ cargo run
               Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
                 Running `target/debug/guessing_game`
            Hello, world!
        "#;

        let raw_lines = string.to_string();
        let lines = raw_lines.lines().map(|line| Ok(line.to_string()));

        let result_vec = super::lint_lines(lines);

        assert!(!result_vec.is_empty());
        assert_eq!(3, result_vec.len());
    }

    #[test]
    fn lint_file_returns_an_empty_vec_when_no_lines_of_interest_are_found() {
        let string = r#"
            $ cargo run
               Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
                 Running `target/guessing_game`
            Guess the number!
            The secret number is: 61
            Please input your guess.
            10
            You guessed: 10
            Too small!
            Please input your guess.
            99
            You guessed: 99
            Too big!
            Please input your guess.
            foo
            Please input your guess.
            61
            You guessed: 61
            You win!
        "#;

        let raw_lines = string.to_string();
        let lines = raw_lines.lines().map(|line| Ok(line.to_string()));

        let result_vec = super::lint_lines(lines);

        assert!(result_vec.is_empty());
    }

    #[test]
    fn is_file_of_interest_returns_false_when_the_path_is_a_directory() {
        let uninteresting_fn = "src/img";

        assert!(!super::is_file_of_interest(path::Path::new(
            uninteresting_fn
        )));
    }

    #[test]
    fn is_file_of_interest_returns_false_when_the_filename_does_not_have_the_md_extension(
    ) {
        let uninteresting_fn = "src/img/foo1.png";

        assert!(!super::is_file_of_interest(path::Path::new(
            uninteresting_fn
        )));
    }

    #[test]
    fn is_file_of_interest_returns_true_when_the_filename_has_the_md_extension()
    {
        let interesting_fn = "src/ch01-00-introduction.md";

        assert!(super::is_file_of_interest(path::Path::new(interesting_fn)));
    }

    #[test]
    fn is_line_of_interest_does_not_report_a_line_if_the_line_contains_a_file_url_which_is_directly_followed_by_the_project_path(
    ) {
        let sample_line =
            "Compiling guessing_game v0.1.0 (file:///projects/guessing_game)";

        assert!(!super::is_line_of_interest(sample_line));
    }

    #[test]
    fn is_line_of_interest_reports_a_line_if_the_line_contains_a_file_url_which_is_not_directly_followed_by_the_project_path(
    ) {
        let sample_line = "Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)";

        assert!(super::is_line_of_interest(sample_line));
    }
}
