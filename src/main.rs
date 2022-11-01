use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Provide two numbers to add!");
        return;
    }

    let res = args[1].parse::<i32>().unwrap_or(0) + args[2].parse::<i32>().unwrap_or(0);
    println!("{res}");
}
