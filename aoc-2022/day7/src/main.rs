use std::path::{Path, PathBuf};
use nom::{IResult, Parser, branch::alt, bytes::complete::{tag, take_while1}, combinator::{all_consuming, map}, sequence::{preceded, separated_pair}};


#[derive(Debug, PartialEq)]
struct Ls;


#[derive(Debug, PartialEq)]
struct Cd<'l>(&'l Path);


#[derive(Debug, PartialEq)]
enum Command<'l> {
    List(Ls),
    ChangeDirectory(Cd<'l>)
}


#[derive(Debug, PartialEq)]
enum Entry {
    Directory(PathBuf),
    File(u64, PathBuf)
}


#[derive(Debug, PartialEq)]
enum Line<'l> {
    Command(Command<'l>),
    Entry(Entry)
}


fn parse_path(input: &str) -> IResult<&str, &Path> {
    let path_parser = take_while1(|character: char| matches!(character, 'a'..='z' | '.' | '/'));
    let mut parser = map(path_parser, |path: &str| Path::new(path));

    parser.parse(input)
}


fn parse_list_command(input: &str) -> IResult<&str, Ls> {
    let mut parser = map(tag("ls"), |_| Ls);

    parser.parse(input)
}


fn parse_change_directory_command<'l>(input: &'l str) -> IResult<&'l str, Cd<'l>> {
    let mut parser = map(preceded(tag("cd "), parse_path), Cd);

    parser.parse(input)
}


fn parse_command<'l>(input: &'l str) -> IResult<&'l str, Command<'l>> {
    let mut prompt_parser = tag("$ ");
    let (input, _) = prompt_parser.parse(input)?;

    let mut command_parser = alt((
        map(parse_list_command, Command::List),
        map(parse_change_directory_command, Command::ChangeDirectory)
    ));

    command_parser.parse(input)
}


fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let parse_directory = map(preceded(tag("dir "), parse_path), |path: &Path| Entry::Directory(path.to_owned()));
    let parse_file = map(separated_pair(nom::character::complete::u64, tag(" "), parse_path), |(size, path)| Entry::File(size, path.to_owned()));

    let mut parser = alt((parse_file, parse_directory));

    parser.parse(input)
}


fn parse_line<'l>(input: &'l str) -> IResult<&'l str, Line<'l>> {
    let mut parser = alt((
        map(parse_command, Line::Command), 
        map(parse_entry, Line::Entry)
    ));

    parser.parse(input)
}


fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines()
        .map(|line| all_consuming(parse_line).parse(line));

    for line in lines {
        println!("{:?}", line);
    }
}


#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};
    use crate::{Line, Cd, Command, Entry, Ls, parse_command, parse_entry, parse_line, parse_path};


    #[test]
    fn test_parse_path() {
        assert_eq!(
            parse_path("a"),
            Ok(("", Path::new("a")))
        );

        assert_eq!(
            parse_path("a/b"),
            Ok(("", Path::new("a/b")))
        );

        assert_eq!(
            parse_path("a/b/"),
            Ok(("", Path::new("a/b/")))
        );

        assert_eq!(
            parse_path("a.b/c/d"),
            Ok(("", Path::new("a.b/c/d")))
        );

        assert_eq!(
            parse_path("a/b/c/d.txt"),
            Ok(("", Path::new("a/b/c/d.txt")))
        );
    }


    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command("$ ls"),
            Ok(("", Command::List(Ls)))
        );

        assert_eq!(
            parse_command("$ cd .."),
            Ok(("", Command::ChangeDirectory(Cd(Path::new("..")))))
        );

        assert_eq!(
            parse_command("$ cd a/b/c.txt"),
            Ok(("", Command::ChangeDirectory(Cd(Path::new("a/b/c.txt")))))
        );

        assert_eq!(
            parse_command("$ cd gcfbqh"),
            Ok(("", Command::ChangeDirectory(Cd(Path::new("gcfbqh")))))
        );
    }


    #[test]
    fn test_parse_entry() {
        assert_eq!(
            parse_entry("dir x"),
            Ok(("", Entry::Directory(PathBuf::from("x"))))
        );

        assert_eq!(
            parse_entry("89668 bplz.rdp"),
            Ok(("", Entry::File(89668, PathBuf::from("bplz.rdp"))))
        );
    }


    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("$ ls"),
            Ok(("", Line::Command(Command::List(Ls))))
        );

        assert_eq!(
            parse_line("$ cd a/b/c.txt"),
            Ok(("", Line::Command(Command::ChangeDirectory(Cd(Path::new("a/b/c.txt"))))))
        );

        assert_eq!(
            parse_line("89668 bplz.rdp"),
            Ok(("", Line::Entry(Entry::File(89668, PathBuf::from("bplz.rdp")))))
        ); 

        assert_eq!(
            parse_line("dir btcjthr"),
            Ok(("", Line::Entry(Entry::Directory(PathBuf::from("btcjthr")))))
        );  
    }
}