use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("invalid argument number");
        std::process::exit(1)
    }
    let n: i32 = args[1].parse().unwrap();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", n);
    println!("  ret");
}
