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
    children: Box<Vec<Item>>,
}

impl Item {
    fn get_folder(&self, path: String) -> &mut Item {
        self.directories()
            .find_map(|d| {
                if d.path == path {
                    return Some(d);
                }
                None
            })
            // .as_deref_mut()
            .unwrap()
    }

    fn size(&self) -> u64 {
        self.size + self.children.iter().map(|child| child.size()).sum::<u64>()
    }

    fn directories(&self) -> Box<dyn Iterator<Item = &mut Item> + '_> {
        let a = self
            .children
            .iter_mut()
            .filter(|child| !child.children.is_empty())
            .flat_map(|child| child.directories());
        return Box::new(a);
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

    let mut fs = Box::new(vec![Item {
        path: "/".into(),
        size: 0,
        children: Box::default(),
    }]);
    let mut active_path = fs.first_mut().unwrap();

    for d in data {
        println!("{d:?}");
        match d {
            Line::Command(command) => match command {
                Command::Ls(_) => {}
                Command::Cd(path) => match path.0.as_str() {
                    "/" => {
                        active_path = fs.first_mut().unwrap();
                    } // We start there and cannot navigate back
                    ".." => {}
                    _ => active_path = fs.first().unwrap().get_folder(path.0),
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(path) => active_path.children.push(Item {
                    path,
                    size: 0,
                    children: Box::default(),
                }),
                Entry::File(size, path) => active_path.children.push(Item {
                    path,
                    size,
                    children: Box::default(),
                }),
            },
        }
    }

    let root = fs.pop().unwrap();
    dbg!(&root);
}
