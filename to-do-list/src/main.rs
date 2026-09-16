use std::io;

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        let (cmd, rest) = input.trim().split_once(' ').unwrap_or((input.trim(), ""));

        match cmd {
            "add" => add(&mut todos, rest),
            "list" => list(&todos),
            "remove" => remove(&mut todos, rest),
            "exit" => return,
            _ => println!("unknown command"),
        };
    }
}

fn add(todos: &mut Vec<String>, item: &str) {
    todos.push(item.to_string());
}

fn list(todos: &Vec<String>) {
    for (i, item) in todos.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }
}

fn remove(todos: &mut Vec<String>, arg: &str) {
    let index: usize = match arg.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("give a valid number");
            return;
        }
    };

    if index == 0 || index > todos.len() {
        println!("no such item");
        return;
    }
    todos.remove(index - 1);
}