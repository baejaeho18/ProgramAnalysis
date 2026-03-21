#![feature(rustc_private)]

use std::fs;
use std::io::{self, Read};
use std::path::Path;

use clap::{Arg, Command};
use type_analysis::*;

fn main() {
    let matches = Command::new("analysis")
        .arg(Arg::new("file").value_name("FILE.rs"))
        .get_matches();

    let code = if let Some(path) = matches.get_one::<String>("file") {
        if Path::new(path).extension().and_then(|ext| ext.to_str()) != Some("rs") {
            eprintln!("expected a single .rs file or no arguments");
            std::process::exit(1);
        }
        fs::read_to_string(path).expect("failed to read input file")
    } else {
        let mut stdin = String::new();
        io::stdin()
            .read_to_string(&mut stdin)
            .expect("failed to read stdin");
        stdin
    };

    if let Some(code) = utils::run_compiler_on_str(&code, transformation::transform).unwrap() {
        println!("{code}");
        utils::run_compiler_on_str(&code, utils::type_check).unwrap();
    } else {
        println!("no solution");
    }
}
