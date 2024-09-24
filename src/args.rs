use std::env;

#[derive(Debug)]
pub struct Args {
    pub command: Command,
    pub options: Vec<GeneralOption>,
}

#[derive(Debug)]
pub enum GeneralOption {
    Help,
    Version,
}

#[derive(Debug)]
pub enum Command {
    Help,
    Version,
    New(NewThing),
    Serve,
    Build,
}

#[derive(Debug)]
pub enum NewThing {
    Wiki,
    Page(String),
}

pub fn get_args() -> Args {
    let mut raw_args = String::new();

    let mut args = env::args();
    args.next();
    for arg in args {
        raw_args += &arg;
        raw_args.push(' ');
    }

    match parse(&raw_args) {
        Some(("", args)) => {
            return args;
        }

        _ => {
            panic!("Parsing error");
        }
    }
}

type Parser<T> = fn(&str) -> Option<(&str, T)>;

pub fn parse(raw_args: &str) -> Option<(&str, Args)> {
    let (raw_args, _) = skip_whitespace(raw_args)?;
    let (raw_args, general_options) = many(parse_general_option, raw_args)?;
    let (raw_args, command) = parse_command(raw_args)?;

    return Some((
        raw_args,
        Args {
            command,
            options: general_options,
        },
    ));
}

// Utils
fn parse_word<'a>(search: &str, raw_args: &'a str) -> Option<(&'a str, ())> {
    if !raw_args.starts_with(search) {
        return None;
    }

    return Some((raw_args.get(search.len()..).unwrap(), ()));
}

fn skip_whitespace(raw_args: &str) -> Option<(&str, ())> {
    return Some((raw_args.trim_start(), ()));
}

fn required_whitespace(raw_args: &str) -> Option<(&str, ())> {
    let trimmed = raw_args.trim_start();
    if trimmed == raw_args {
        return None;
    }

    return Some((trimmed, ()));
}

fn many<T>(parser: Parser<T>, mut raw_args: &str) -> Option<(&str, Vec<T>)> {
    let mut results: Vec<T> = vec![];

    loop {
        match parser(raw_args) {
            None => {
                break;
            }

            Some((remaining, result)) => {
                results.push(result);
                raw_args = remaining;
            }
        };
    }

    return Some((raw_args, results));
}

fn one_of<'a, T>(parsers: &[Parser<T>], raw_args: &'a str) -> Option<(&'a str, T)> {
    for parser in parsers {
        let result = parser(raw_args);

        match result {
            None => {
                continue;
            }

            result => {
                return result;
            }
        }
    }
    return None;
}

// Options
fn parse_general_option(raw_args: &str) -> Option<(&str, GeneralOption)> {
    let (raw_args, _) = parse_word("--", raw_args)?;

    let (raw_args, result) = one_of(&[parse_help_option, parse_version_option], raw_args)?;

    let (raw_args, _) = required_whitespace(raw_args)?;

    return Some((raw_args, result));
}

fn parse_help_option(raw_args: &str) -> Option<(&str, GeneralOption)> {
    let (raw_args, _) = parse_word("help", raw_args)?;

    return Some((raw_args, GeneralOption::Help));
}

fn parse_version_option(raw_args: &str) -> Option<(&str, GeneralOption)> {
    let (raw_args, _) = parse_word("version", raw_args)?;

    return Some((raw_args, GeneralOption::Version));
}

// Commands
fn parse_command(raw_args: &str) -> Option<(&str, Command)> {
    let (raw_args, result) = one_of(
        &[
            parse_help_command,
            parse_version_command,
            parse_new_command,
            parse_serve_command,
            parse_build_command,
        ],
        raw_args,
    )?;

    let (raw_args, _) = skip_whitespace(raw_args)?;

    return Some((raw_args, result));
}

fn parse_help_command(raw_args: &str) -> Option<(&str, Command)> {
    let (raw_args, _) = parse_word("help", raw_args)?;

    return Some((raw_args, Command::Help));
}

fn parse_version_command(raw_args: &str) -> Option<(&str, Command)> {
    let (raw_args, _) = parse_word("version", raw_args)?;

    return Some((raw_args, Command::Version));
}

fn parse_build_command(raw_args: &str) -> Option<(&str, Command)> {
    let (raw_args, _) = parse_word("build", raw_args)?;

    return Some((raw_args, Command::Build));
}

fn parse_new_command(raw_args: &str) -> Option<(&str, Command)> {
    let (raw_args, _) = parse_word("new", raw_args)?;
    let (raw_args, _) = required_whitespace(raw_args)?;
    let (raw_args, new_thing) = one_of(&[parse_wiki_new_thing, parse_page_new_thing], raw_args)?;

    return Some((raw_args, Command::New(new_thing)));
}

fn parse_wiki_new_thing(raw_args: &str) -> Option<(&str, NewThing)> {
    let (raw_args, _) = parse_word("wiki", raw_args)?;

    return Some((raw_args, NewThing::Wiki));
}

fn parse_page_new_thing(raw_args: &str) -> Option<(&str, NewThing)> {
    let (raw_args, _) = parse_word("page", raw_args)?;
    let (raw_args, _) = required_whitespace(raw_args)?;
    let (raw_args, name) = parse_page_name(raw_args)?;

    return Some((raw_args, NewThing::Page(name)));
}

fn parse_page_name(raw_args: &str) -> Option<(&str, String)> {
    let mut name = String::from("");
    let valid_name_chars = &['(', ')', '_'];

    for char in raw_args.chars() {
        if !char.is_alphanumeric() && !valid_name_chars.contains(&char) {
            break;
        }

        name.push(char);
    }

    if name.len() == 0 {
        return None;
    }

    return Some((raw_args.get(name.len()..).unwrap(), name));
}

fn parse_serve_command(raw_args: &str) -> Option<(&str, Command)> {
    let (raw_args, _) = parse_word("serve", raw_args)?;

    return Some((raw_args, Command::Serve));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_word_success() {
        let raw_args = "fine addition";
        let result = parse_word("fine", raw_args);

        assert_eq!(result, Some((" addition", ())));
    }

    #[test]
    fn many_success() {
        let raw_args = "aaab";
        let result = many(|a| parse_word("a", a), raw_args);

        assert_eq!(result, Some(("b", vec![(), (), ()])));
    }

    #[test]
    fn one_of_success() {
        let raw_args = "c";
        let parsers: &[Parser<()>] = &[
            |a| {
                return parse_word("a", a);
            },
            |a| {
                return parse_word("b", a);
            },
            |a| {
                return parse_word("c", a);
            },
        ];
        let result = one_of(parsers, raw_args);

        assert_eq!(result, Some(("", ())));
    }
}
