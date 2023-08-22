use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::combinator::all_consuming;
use nom::sequence::{preceded, separated_pair};
use nom::Finish;
use nom::{bytes::complete::take_while1, combinator::map, IResult};

#[derive(Debug)]
struct Item {
    path: String,
    size: u64,
    children: Vec<Item>,
}

impl Item {
    fn size(&self) -> u64 {
        self.size + self.children.iter().map(|child| child.size()).sum::<u64>()
    }

    fn directories(&self) -> Box<dyn Iterator<Item = &Item> + '_> {
        let a = std::iter::once(self).chain(
            self.children
                .iter()
                .filter(|child| !child.children.is_empty())
                .flat_map(|child| child.directories()),
        );
        Box::new(a)
    }
}

#[derive(Debug)]
struct Ls;

fn parse_ls(line: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(line)
}

#[derive(Debug)]
struct Cd(String);

fn parse_cd(line: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_line), Cd)(line)
}

fn parse_line(line: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(line)
}

#[derive(Debug)]
enum Command {
    Ls(Ls),
    Cd(Cd),
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Command::Ls), map(parse_cd, Command::Cd)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(u64, String),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_line),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_line), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_input_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

fn main() {
    let data = include_str!("input.txt")
        .lines()
        .map(|l| all_consuming(parse_input_line)(l).finish().unwrap().1);

    let mut fs = vec![Item {
        path: "/".into(),
        size: 0,
        children: Vec::default(),
    }];

    let currentItem = fs.pop().unwrap();

    for d in data {
        match d {
            Line::Command(command) => match command {
                Command::Ls(_) => {}
                Command::Cd(path) => match path.0.as_str() {
                    "/" => {}
                    ".." => {
                        let dir = fs.pop().unwrap();
                        fs.last_mut().unwrap().children.push(dir);
                    }
                    _ => {
                        let item = Item {
                            path: path.0,
                            size: 0,
                            children: Vec::default(),
                        };
                        fs.push(item)
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {}
                Entry::File(size, path) => {
                    let item = Item {
                        path,
                        size,
                        children: Vec::default(),
                    };
                    currentItem.children.push(item);
                    fs.last_mut().unwrap().children.push(item);
                }
            },
        }
    }

    let mut root = fs.pop().unwrap();
    while let Some(mut next) = fs.pop() {
        next.children.push(root);
        root = next;
    }
    // dbg!(&root);

    let result: u64 = root
        .directories()
        .map(|d| d.size())
        .filter(|&size| size <= 100000)
        .sum();

    println!("Result day 1: {result}");

    let total_space = 70000000_u64;
    let used_space = root.size();
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let result_day2 = root
        .directories()
        .map(|d| d.size())
        .filter(|&size| size >= minimum_space_to_free)
        .min()
        .unwrap();

    println!("Result day 2: {result_day2}");
}
