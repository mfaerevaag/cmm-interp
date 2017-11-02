extern crate cmm;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = match env::args().nth(1) {
        Some(s) => s,
        _ => panic!("no filename given"),
    };

    let mut f = File::open(filename).unwrap();

    let mut prog = String::new();
    f.read_to_string(&mut prog).unwrap();

    match cmm::run(prog) {
        Ok(ret) => println!("returned {:?}", ret),
        Err(()) => println!("returned error"),
    }
}
