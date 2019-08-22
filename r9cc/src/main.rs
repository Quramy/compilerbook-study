use std::env;

fn strtol(x: &Vec<char>, offset: &mut usize) -> u32 {
    let mut n = *offset;
    let mut r: u32 = 0;
    while n < x.len() && x[n].is_digit(10) {
        r = r * 10 + x[n].to_digit(10).unwrap();
        n = n + 1;
    }
    *offset = n;
    r
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("invalid argument number");
        std::process::exit(1)
    }
    let p = args[1].chars().collect::<Vec<char>>();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    let mut i: usize = 0;
    println!("  mov rax, {}", strtol(&p, &mut i));
    let l = p.len();
    while i < l {
        let x = p[i];
        match x {
            '+' => {
                i = i + 1;
                println!("  add rax, {}", strtol(&p, &mut i));
            },
            '-' => {
                i = i + 1;
                println!("  sub rax, {}", strtol(&p, &mut i));
            },
            _ => { },
        }
    }
    println!("  ret");
}
