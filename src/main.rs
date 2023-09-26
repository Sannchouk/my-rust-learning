use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your fibonacci index");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed");
    let n : i32 = input.trim().parse().expect("Wrong format");
    fibonacci(n);
}

fn fibonacci(n: i32){
    let mut x0 = 0;
    let mut x1 = 1;
    for _ in 1..=n {
        println!("{x1}");
        x1 = x1 + x0;
        x0 = x1 - x0;
    }
}