use std::io;

fn main() {
    println!("number 1: ");
    let mut n1_input = String::new();
    io::stdin().read_line(&mut n1_input).expect("failed to read line");
    let n1: f64 = match n1_input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("input is not a valid number");
                    std::process::exit(1);
                }
            };

    println!("operator: ");
    let mut op_input = String::new();
    io::stdin()
        .read_line(&mut op_input)
        .expect("failed to read line");
    let op = op_input.trim();

    println!("number 2: ");
    let mut n2_input = String::new();
    io::stdin()
        .read_line(&mut n2_input)
        .expect("failed to read line");
    let n2: f64 = match n2_input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("input is not a valid number");
                    std::process::exit(1);
                }
            };

    let ans: f64;

    ans = match op {
        "+" => add(n1, n2),
        "-" => sub(n1, n2),
        "*" => mul(n1, n2),
        "/" => div(n1, n2),
        _ => {println!("unknown operator"); std::process::exit(1);},
    };
    println!("answer: {ans}");
}

fn add(n1: f64, n2: f64) -> f64 {
    return n1 + n2;
}

fn sub(n1: f64, n2: f64) -> f64 {
    return n1 - n2;
}

fn mul(n1: f64, n2: f64) -> f64 {
    return n1 * n2;
}
fn div(n1: f64, n2: f64) -> f64 {
    if n2 == 0.0 {
        println!("can't devide by 0");
        std::process::exit(1);
    } else {
        return n1 / n2;
    }
}
