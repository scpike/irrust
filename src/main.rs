#![feature(test)]
extern crate itertools;
extern crate test;
extern crate unidecode;
extern crate csv;
extern crate regex;

use std::io::{self, BufRead};
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Write;

mod jaccard;
mod normalize;

fn block(s: &str) -> Vec<char> {
    s.chars().filter(|l| l.is_alphanumeric()).take(2).collect()
}

macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

// Read lines in from stdin, do n^2 comparisons, print
// {Score, Word1, Word2} to stdout
//
fn main() {
    let stdin = io::stdin();

    let mut wtr = csv::Writer::from_memory();
    let mut lines: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    lines.sort_by(|a, b| a.cmp(b));
    let mut normals = HashMap::new();

    let mut i = 0;
    for l in &lines {
        normals.insert(l, normalize::normalize(l));
    }

    for (_, group) in lines.iter().group_by(|l| block(l)) {
        for (a, b) in group.iter().combinations() {
            i += 1;
            let na = normals.get(a);
            let nb = normals.get(b);
            let dist = jaccard::compare_normals(na.unwrap(), nb.unwrap());
            if dist > 0.5 {
                let record = wtr.write(vec![&dist.to_string(), a, b].into_iter());
                assert!(record.is_ok())
            }
        }
    }
    println!("{}", wtr.as_string());
}
