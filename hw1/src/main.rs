//Calculate an exponential modulo of some (usually prime) number

use std::env;

fn modexp(x: u64, y: u64, m: u64) -> u64 {
    assert!(m != 0);
    if x == 0 {
        return 0;
    }
    if y == 0 {
        return 1;
    }
    let mut z = modexp(x, y / 2, m);
    z = (z * z) % m;
    if y % 2 == 1 {
        z = (z * x) % m
    }
    z
}

#[test]
fn test_modexp() {
    assert_eq!(modexp(2, 20, 17), 16);
}

fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");
    std::process::exit(1);
}

// Pass 3 numbers from argument and print modexp as result
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let arg1 = args[1].parse::<u64>().unwrap_or_else(|_| error());
    let arg2 = args[2].parse::<u64>().unwrap_or_else(|_| error());
    let arg3 = args[3].parse::<u64>().unwrap_or_else(|_| error());

    let r = modexp(arg1, arg2, arg3);
    println!("{}", r);
}
