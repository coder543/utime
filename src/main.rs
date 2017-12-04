use std::time::Instant;
use std::process::Command;
use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 1 {
        panic!("No command provided!");
    }

    let start = Instant::now();
    let _ = Command::new(&args[0]).args(&args[1..]).status();
    let finish = start.elapsed();

    let total = finish.as_secs() as f64 + (finish.subsec_nanos() as f64) / 1_000_000_000.0;

    print!("\"{}\" took ", args[0]);
    if total < 0.000001 {
        print!("{} ns", total * 1000.0 * 1000.0 * 1000.0);
    } else if total < 0.001 {
        print!("{} us", total * 1000.0 * 1000.0);
    } else if total < 1.0 {
        print!("{} ms", total * 1000.0);
    } else {
        print!("{} secs", total);
    }
    println!(" to complete.");
}
