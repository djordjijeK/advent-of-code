use core::fmt;
use std::{cell::RefCell, collections::{BTreeMap}, path::{Path, PathBuf}, rc::Rc};
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


#[derive(Default)]
struct TreeNode {
    parent: Option<Rc<RefCell<TreeNode>>>,
    children: BTreeMap<PathBuf, Rc<RefCell<TreeNode>>>,
    size: usize
}


impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}


impl TreeNode {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }


    fn total_size(&self) -> u64 {
        self.children.values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>() + self.size as u64
    }
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


fn solve_part_1(root: Rc<RefCell<TreeNode>>) -> u64 {
    all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>()
} 


fn solve_part_2(root: Rc<RefCell<TreeNode>>) -> u64 {
    let total_space = 70000000_u64;
    let used_space = root.borrow().total_size();
    let free_space = total_space.checked_sub(used_space).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    all_dirs(root).map(|d| d.borrow().total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .min()
        .unwrap()
}


fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines()
        .map(|line| all_consuming(parse_line).parse(line).unwrap().1);

    let root = Rc::new(RefCell::new(TreeNode::default()));
    let mut node = root.clone();

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::List(_) => {
                    // we are parsing file line by line, we do not have to do anything here
                }
                Command::ChangeDirectory(Cd(path)) => match path.to_str().unwrap() {
                    "/" => {
                        // ignore, we're already there
                    }
                    ".." => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    }
                    _ => {
                        let child = node.borrow_mut().children.entry(path.to_path_buf()).or_default().clone();
                        node = child;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Directory(dir) => {
                    let entry = node.borrow_mut().children.entry(dir).or_default().clone();
                    entry.borrow_mut().parent = Some(node.clone());
                }
                Entry::File(size, file) => {
                    let entry = node.borrow_mut().children.entry(file).or_default().clone();
                    entry.borrow_mut().size = size as usize;
                    entry.borrow_mut().parent = Some(node.clone());
                }
            },
        }
    }

    let part_1 = solve_part_1(root.clone());
    let part_2 = solve_part_2(root.clone());

    println!("Part 1 result: {part_1}");
    println!("Part 2 result: {part_2}");
}


fn all_dirs(n: Rc<RefCell<TreeNode>>) -> Box<dyn Iterator<Item = Rc<RefCell<TreeNode>>>> {
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(std::iter::once(n)
        .chain(children.into_iter()
            .filter_map(|c| {
                if c.borrow().is_dir() {
                    Some(all_dirs(c))
                } else {
                    None
                }
            })
            .flatten()
    ))
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