#![feature(test)]
use rug::Integer;
use std::env;
extern crate test;

use factor::factor::factor_core;
use factor::util::str_to_vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: Integer = args[1].parse().unwrap();
    let p_bits_str = args[2].as_bytes();
    let q_bits_str = args[3].as_bytes();
    let bit_len = p_bits_str.len().max(q_bits_str.len());
    let p_bits = str_to_vec(p_bits_str, bit_len);
    let q_bits = str_to_vec(q_bits_str, bit_len);
    assert_eq!(p_bits.len(), q_bits.len());
    match factor_core(&n, p_bits, q_bits, bit_len, false, "bfs".to_string()) {
        Some((p, q)) => println!("{}, {}", p, q),
        None => println!("Not found"),
    };
}
