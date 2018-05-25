use std::process::Command;
use std::fs;
use std::env;

extern crate criterion;
use criterion::Criterion;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 1 {
        panic!("No command provided!");
    }

    Criterion::default()
        .bench_function(&args[0].clone(), move |b| b.iter(|| {
            Command::new(&args[0]).args(&args[1..]).status().unwrap();
        }));

    let _ = fs::remove_dir_all("target/criterion");
}
