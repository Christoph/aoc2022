fn main() {
    let commands: Vec<_> = include_str!("input.txt").lines().map(parse_line).collect();

    let measurements = [20, 60, 100, 140, 180, 220];
    let mut cycle = 0;
    let mut value = 1;
    let mut total_signal_strength = 0;
    for command in commands {
        let runtime = match command.operation {
            Operations::Noop(duration) => duration,
            Operations::Add(duration) => duration,
        };

        for _ in 0..runtime {
            cycle += 1;

            if measurements.contains(&cycle) {
                println!("{cycle} * {value}");
                total_signal_strength += (value * cycle)
            }
        }

        value += command.value;
    }
    println!("cresult = {total_signal_strength}");
}

fn parse_line(line: &str) -> Command {
    if line.contains(' ') {
        let split: Vec<&str> = line.split(' ').collect();
        Command {
            operation: Operations::Add(2),
            value: split.get(1).unwrap().parse().unwrap(),
        }
    } else {
        Command {
            operation: Operations::Noop(1),
            value: 0,
        }
    }
}

#[derive(Debug)]
enum Operations {
    Noop(u8),
    Add(u8),
}

#[derive(Debug)]
struct Command {
    operation: Operations,
    value: i32,
}
