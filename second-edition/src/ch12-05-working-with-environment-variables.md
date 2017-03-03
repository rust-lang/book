## Working with Environment Variables

Let's add one more feature: case insensitive searching. In addition, this
setting won't be a command line option: it'll be an environment variable
instead. We could choose to make case insensitivity a command line option, but
our users have requested an environment variable that they could set once and
make all their searches case insensitive in that terminal session.

### Implement and Test a Case-Insensitive `grep` Function

First, let's add a new function that we will call when the environment variable
is on. Let's start by adding a new test and re-naming our existing one:

```rust,ignore
#[cfg(test)]
mod test {
    use {grep, grep_case_insensitive};

    #[test]
    fn case_sensitive() {
        let search = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            grep(search, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let search = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            grep_case_insensitive(search, contents)
        );
    }
}
```

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We're going to define a new function named `grep_case_insensitive`. Its
implementation will be almost the same as the `grep` function, but with some
minor changes as shown in Listing 12-16:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let search = search.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&search) {
            results.push(line);
        }
    }

    results
}
```

<figcaption>

Listing 12-16: Implementing a `grep_case_insensitive` function by changing the
search string and the lines of the contents to lowercase before comparing them

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

First, we lowercase the `search` string, and store it in a shadowed variable
with the same name. Note that `search` is now a `String` rather than a string
slice, so we need to add an ampersand when we pass `search` to `contains` since
`contains` takes a string slice.

Second, we add a call to `to_lowercase` each `line` before we check if it
contains `search`. Since we've converted both `line` and `search` into all
lowercase, we'll find matches no matter what case they used in the file and the
command line arguments, respectively. Let's see if this passes the tests:

```text
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target\debug\deps\greprs-e58e9b12d35dc861.exe

running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured

     Running target\debug\greprs-8a7faa2662b5030a.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests greprs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Great! Now, we have to actually use the new `grep_case_insensitive` function.
First, let's add a configuration option for it to the `Config` struct:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Config {
    pub search: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

<!-- Will add ghosting in libreoffice /Carol -->

And then check for that option inside of the `run` function, and decide which
function to call based on the value of the `case_sensitive` function:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        grep(&config.search, &contents)
    } else {
        grep_case_insensitive(&config.search, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

<!-- Will add ghosting in libreoffice /Carol -->

Finally, we need to actually check the environment for the variable. To bring
the `env` module from the standard library into our project, we add a `use` line
at the top of *src/lib.rs*:

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::env;
```

And then use the `vars` method from the `env` module inside of `Config::new`:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::env;
#
# struct Config {
#     search: String,
#     filename: String,
#     case_sensitive: bool,
# }
#
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let search = args[1].clone();
        let filename = args[2].clone();

        let mut case_sensitive = true;

        for (name, _) in env::vars() {
            if name == "CASE_INSENSITIVE" {
                case_sensitive = false;
            }
        }

        Ok(Config {
            search: search,
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}
```

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Here, we call `env::vars`, which works in a similar way as `env::args`. The
difference is `env::vars` returns an iterator of environment variables rather
than command line arguments. Instead of using `collect` to create a vector of
all of the environment variables, we're using a `for` loop. `env::vars` returns
tuples: the name of the environment variable and its value. We never care about
the values, only if the variable is set at all, so we use the `_` placeholder
instead of a name to let Rust know that it shouldn't warn us about an unused
variable. Finally, we have a `case_sensitive` variable, which is set to true by
default. If we ever find a `CASE_INSENSITIVE` environment variable, we set the
`case_sensitive` variable to false instead. Then we return the value as part of
the `Config`.

Let's give it a try!

```text
$ cargo run to poem.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

```text
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Excellent! Our `greprs` program can now do case insensitive searching controlled
by an environment variable. Now you know how to manage options set using
either command line arguments or environment variables!

Some programs allow both arguments _and_ environment variables for the same
configuration. In those cases, the programs decide that one or the other of
arguments or environment variables take precedence. For another exercise on
your own, try controlling case insensitivity through a command line argument as
well, and decide which should take precedence if you run the program with
contradictory values.

The `std::env` module contains many more useful features for dealing with
environment variables; check out its documentation to see what's available.
