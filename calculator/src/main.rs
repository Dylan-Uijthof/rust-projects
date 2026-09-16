use std::io;

fn main() {
    let mut n1_input = String::new();
    io::stdin()
        .read_line(&mut n1_input)
        .expect("failed to read line");
    let n1: f64 = n1_input.trim().parse().expect("not a valid numer");

    let mut op_input = String::new();
    io::stdin()
        .read_line(&mut op_input)
        .expect("failed to read line");
    let op = op_input.trim();

    let mut n2_input = String::new();
    io::stdin()
        .read_line(&mut n2_input)
        .expect("failed to read line");
    let n2: f64 = n2_input.trim().parse().expect("not a valid number");

    let ans: f64;

    ans = match op {
        "+" => add(n1, n2),
        "-" => sub(n1, n2),
        "*" => mul(n1, n2),
        "/" => div(n1, n2),
        _ => panic!("unknown operator"),
    };
    println!("{ans}");
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
    if n1 == 0.0 {
        println!("can't devide by 0");
        return 0.0;
    } else {
        return n1 / n2;
    }
}
